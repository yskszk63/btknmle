use std::io;
use std::os::unix::ffi::OsStrExt as _;
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::{ready, Stream};
pub use input::event;
use input::{Event, Libinput, LibinputInterface};
use mio::unix::EventedFd;
use mio::{Evented, Poll as MioPoll, PollOpt, Ready, Token};
use tokio_net::util::PollEvented;
use log::debug;

pub use codes::{ButtonCodes, KeyCodes};

mod codes;

struct Env;

impl LibinputInterface for Env {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<RawFd, i32> {
        let cpath = std::ffi::CString::new(path.as_os_str().as_bytes()).map_err(|_| -1)?;
        match unsafe { libc::open(cpath.as_ptr(), flags) } {
            x if x < 0 => Err(io::Error::last_os_error().raw_os_error().unwrap_or(x)),
            x => {
                debug!("open {:?} {}", path, x);
                Ok(x)
            }
        }
    }

    fn close_restricted(&mut self, fd: RawFd) {
        debug!("close {}", fd);
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
    pub fn new_from_udev(udev_seat: &str) -> io::Result<LibinputStream> {
        let udevcx = udev::Context::new()?;
        let mut libinput = Libinput::new_from_udev(Env, &udevcx);
        libinput.udev_assign_seat(udev_seat).unwrap();
        libinput.dispatch()?;
        Ok(LibinputStream(PollEvented::new(EventedLibinput(libinput))))
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
