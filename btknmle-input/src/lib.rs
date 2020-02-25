#![warn(clippy::all)]

use std::collections::HashSet;
use std::io;
use std::os::unix::ffi::OsStrExt as _;
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use futures::{ready, Stream};
pub use input::event;
use input::{Event, Libinput, LibinputInterface};
use log::{debug, warn};
use mio::unix::EventedFd;
use mio::{Evented, Poll as MioPoll, PollOpt, Ready, Token};
use tokio::io::PollEvented;

pub use codes::{ButtonCodes, KeyCodes};

pub mod model {
    pub use input::{Device, DeviceCapability};
}
mod codes;
mod sys;

fn grab(fd: RawFd, grab: bool) -> io::Result<()> {
    let v = if grab { 1 } else { 0 };

    #[cfg(not(target_env = "musl"))]
    let eviocgrab = sys::linux_input::_EVIOCGRAB;

    #[cfg(target_env = "musl")]
    let eviocgrab = sys::linux_input::_EVIOCGRAB as libc::c_long;

    match unsafe { libc::ioctl(fd, eviocgrab, v) } {
        err if err < 0 => Err(io::Error::last_os_error()),
        _ => Ok(()),
    }
}

#[derive(Debug, Default)]
struct GrabCollection {
    grabbed: bool,
    fds: HashSet<RawFd>,
}

impl GrabCollection {
    fn add(&mut self, fd: RawFd) -> io::Result<()> {
        self.fds.insert(fd);
        if self.grabbed {
            grab(fd, true)?;
        }
        Ok(())
    }

    fn remove(&mut self, fd: RawFd) -> io::Result<()> {
        self.fds.remove(&fd);
        if self.grabbed {
            grab(fd, false)?;
        }
        Ok(())
    }

    fn grab(&mut self) -> io::Result<()> {
        self.grabbed = true;
        for fd in &self.fds {
            grab(*fd, true)?;
        }
        Ok(())
    }

    fn ungrab(&mut self) -> io::Result<()> {
        self.grabbed = false;
        for fd in &self.fds {
            grab(*fd, false)?;
        }
        Ok(())
    }
}

struct Env(Arc<Mutex<GrabCollection>>);

impl LibinputInterface for Env {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<RawFd, i32> {
        let cpath = std::ffi::CString::new(path.as_os_str().as_bytes()).map_err(|_| -1)?;
        match unsafe { libc::open(cpath.as_ptr(), flags) } {
            err if err < 0 => Err(io::Error::last_os_error().raw_os_error().unwrap_or(err)),
            fd => {
                debug!("open {:?} {}", path, fd);
                match self.0.lock() {
                    Ok(mut fds) => {
                        if let Err(e) = fds.add(fd) {
                            warn!("failed to add for grab collection {}", e);
                        }
                    }
                    Err(e) => {
                        warn!("failed to add for grab collection {}", e);
                    }
                }
                Ok(fd)
            }
        }
    }

    fn close_restricted(&mut self, fd: RawFd) {
        debug!("close {}", fd);
        match self.0.lock() {
            Ok(mut fds) => {
                if let Err(e) = fds.remove(fd) {
                    warn!("failed to remove for grab collection {}", e);
                }
            }
            Err(e) => {
                warn!("failed to remove for grab collection {}", e);
            }
        }
        unsafe { libc::close(fd) };
    }
}

#[derive(Debug)]
struct EventedLibinput(Libinput);

impl Evented for EventedLibinput {
    fn register(
        &self,
        poll: &MioPoll,
        token: Token,
        interest: Ready,
        opts: PollOpt,
    ) -> io::Result<()> {
        EventedFd(&self.0.as_raw_fd()).register(poll, token, interest, opts)
    }
    fn reregister(
        &self,
        poll: &MioPoll,
        token: Token,
        interest: Ready,
        opts: PollOpt,
    ) -> io::Result<()> {
        EventedFd(&self.0.as_raw_fd()).reregister(poll, token, interest, opts)
    }
    fn deregister(&self, poll: &MioPoll) -> io::Result<()> {
        EventedFd(&self.0.as_raw_fd()).deregister(poll)
    }
}

#[derive(Debug)]
pub struct LibinputStream {
    grabs: Arc<Mutex<GrabCollection>>,
    io: PollEvented<EventedLibinput>,
}

impl LibinputStream {
    pub fn new_from_udev(udev_seat: &str) -> io::Result<LibinputStream> {
        let grabs = Arc::new(Mutex::new(Default::default()));
        let udevcx = udev::Context::new()?;
        let mut libinput = Libinput::new_from_udev(Env(grabs.clone()), &udevcx);
        libinput.udev_assign_seat(udev_seat).unwrap();
        libinput.dispatch()?;
        Ok(LibinputStream {
            grabs,
            io: PollEvented::new(EventedLibinput(libinput))?,
        })
    }

    pub fn grab(&mut self) -> io::Result<()> {
        match self.grabs.lock() {
            Ok(mut grabs) => grabs.grab(),
            Err(..) => Err(io::Error::new(io::ErrorKind::Other, "failed to lock")),
        }
    }

    pub fn ungrab(&mut self) -> io::Result<()> {
        match self.grabs.lock() {
            Ok(mut grabs) => grabs.ungrab(),
            Err(..) => Err(io::Error::new(io::ErrorKind::Other, "failed to lock")),
        }
    }
}

impl Stream for LibinputStream {
    type Item = Result<Event, io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match (self.io).get_mut().0.next() {
            Some(evt) => Poll::Ready(Some(Ok(evt))),
            None => {
                ready!(self.io.poll_read_ready(cx, Ready::readable()))?;
                match (self.io).get_mut().0.dispatch() {
                    Ok(..) => match (self.io).get_mut().0.next() {
                        Some(evt) => Poll::Ready(Some(Ok(evt))),
                        None => {
                            self.io.clear_read_ready(cx, Ready::readable())?;
                            Poll::Pending
                        }
                    },
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        self.io.clear_read_ready(cx, Ready::readable())?;
                        Poll::Pending
                    }
                    Err(x) => Poll::Ready(Some(Err(x))),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        fn assert_send<T: Send>() {};
        fn assert_sync<T: Sync>() {};

        assert_send::<Env>();
        assert_sync::<Env>();

        //assert_send::<EventedLibinput>();
        //assert_sync::<EventedLibinput>();
    }

    #[test]
    fn test_env_libinputif() {
        use std::fs;
        use std::path::PathBuf;

        let path = PathBuf::from("/tmp/btknmle-input-6c9e069b-8099-4c5a-8c44-7c2387e391d8");
        {
            fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open(&path)
                .unwrap();
        }

        let grabs = Arc::new(Mutex::new(Default::default()));
        let mut env = Env(grabs);
        if let Ok(fd) = env.open_restricted(&path, 0) {
            env.close_restricted(fd)
        }
    }

    #[tokio::test]
    async fn test_libinput() {
        use tokio::stream::StreamExt;
        let stream = LibinputStream::new_from_udev("default").unwrap();
        let mut stream = stream
            .timeout(tokio::time::Duration::from_millis(100))
            .take_while(Result::is_ok);
        while let Some(..) = stream.next().await {}
    }
}
