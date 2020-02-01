#![deny(clippy::all)]

use std::io;
use std::os::unix::ffi::OsStrExt as _;
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::{ready, Stream};
pub use input::event;
use input::{Event, Libinput, LibinputInterface};
use log::{debug, warn};
use mio::unix::EventedFd;
use mio::{Evented, Poll as MioPoll, PollOpt, Ready, Token};
use tokio::io::PollEvented;

pub use codes::{ButtonCodes, KeyCodes};

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

struct Env(bool);

impl LibinputInterface for Env {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<RawFd, i32> {
        let cpath = std::ffi::CString::new(path.as_os_str().as_bytes()).map_err(|_| -1)?;
        match unsafe { libc::open(cpath.as_ptr(), flags) } {
            err if err < 0 => Err(io::Error::last_os_error().raw_os_error().unwrap_or(err)),
            fd => {
                debug!("open {:?} {}", path, fd);
                if self.0 {
                    if let Err(e) = grab(fd, true) {
                        warn!("grab failed {:?} {}", path, e)
                    }
                }
                Ok(fd)
            }
        }
    }

    fn close_restricted(&mut self, fd: RawFd) {
        debug!("close {}", fd);
        if self.0 {
            if let Err(e) = grab(fd, false) {
                warn!("ungrab failed {}", e)
            }
        }
        unsafe { libc::close(fd) };
    }
}

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

pub struct LibinputStream(PollEvented<EventedLibinput>);

impl LibinputStream {
    pub fn new_from_udev(udev_seat: &str, grab: bool) -> io::Result<LibinputStream> {
        let udevcx = udev::Context::new()?;
        let mut libinput = Libinput::new_from_udev(Env(grab), &udevcx);
        libinput.udev_assign_seat(udev_seat).unwrap();
        libinput.dispatch()?;
        Ok(LibinputStream(PollEvented::new(EventedLibinput(libinput))?))
    }
}

impl Stream for LibinputStream {
    type Item = Result<Event, io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match (self.0).get_mut().0.next() {
            Some(evt) => Poll::Ready(Some(Ok(evt))),
            None => {
                ready!(self.0.poll_read_ready(cx, Ready::readable()))?;
                match (self.0).get_mut().0.dispatch() {
                    Ok(..) => match (self.0).get_mut().0.next() {
                        Some(evt) => Poll::Ready(Some(Ok(evt))),
                        None => {
                            self.0.clear_read_ready(cx, Ready::readable())?;
                            Poll::Pending
                        }
                    },
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        self.0.clear_read_ready(cx, Ready::readable())?;
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
}
