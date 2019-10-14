use std::io;
use std::mem::size_of;
use std::os::unix::io::RawFd;

use mio::event::Evented;
use mio::unix::EventedFd;
use mio::{Poll, PollOpt, Ready, Token};

#[repr(C)]
#[derive(Debug)]
struct socketaddr_hci {
    hci_family: libc::sa_family_t,
    hci_dev: libc::c_ushort,
    hci_channel: libc::c_ushort,
}

//const BTPROTO_L2CAP: libc::c_int = 0;
const BTPROTO_HCI: libc::c_int = 1;
//const BTPROTO_SCO: libc::c_int = 2;
//const BTPROTO_RFCOMM: libc::c_int = 3;
//const BTPROTO_BNEP: libc::c_int = 4;
//const BTPROTO_CMTP: libc::c_int = 5;
//const BTPROTO_HIDP: libc::c_int = 6;
//const BTPROTO_AVDTP: libc::c_int = 7;

//const HCI_DEV_NONE: libc::c_ushort = 0xffff;

//const HCI_CHANNEL_RAW: libc::c_ushort = 0;
const HCI_CHANNEL_USER: libc::c_ushort = 1;
//const HCI_CHANNEL_MONITOR: libc::c_ushort = 2;
//const HCI_CHANNEL_CONTROL: libc::c_ushort = 3;
//const HCI_CHANNEL_LOGGING: libc::c_ushort = 4;

#[derive(Debug)]
pub(crate) struct RawHciSocket(RawFd);

impl RawHciSocket {
    pub(crate) fn new(blocking: bool) -> io::Result<Self> {
        let mut socktype = libc::SOCK_RAW | libc::SOCK_CLOEXEC;
        if !blocking {
            socktype |= libc::SOCK_NONBLOCK;
        };

        let r = unsafe { libc::socket(libc::AF_BLUETOOTH, socktype, BTPROTO_HCI) };
        if r < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(Self(r as RawFd))
        }
    }

    pub(crate) fn bind(&self, hci_dev: u16) -> io::Result<()> {
        let addr = socketaddr_hci {
            hci_family: (libc::AF_BLUETOOTH as libc::sa_family_t),
            hci_dev,
            hci_channel: HCI_CHANNEL_USER,
        };

        let r = unsafe {
            libc::bind(
                self.0,
                &addr as *const _ as *const libc::sockaddr,
                size_of::<socketaddr_hci>() as libc::c_uint,
            )
        };

        if r == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }

    pub(crate) fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        let r = unsafe { libc::recv(self.0, buf.as_mut_ptr() as *mut libc::c_void, buf.len(), 0) };
        if r < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(r as usize)
        }
    }

    pub(crate) fn send(&self, buf: &[u8]) -> io::Result<usize> {
        let r = unsafe { libc::send(self.0, buf.as_ptr() as *const libc::c_void, buf.len(), 0) };
        if r < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(r as usize)
        }
    }
}

impl Drop for RawHciSocket {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.0);
        }
    }
}

impl Evented for RawHciSocket {
    fn register(&self, poll: &Poll, token: Token, interest: Ready, opt: PollOpt) -> io::Result<()> {
        EventedFd(&self.0).register(poll, token, interest, opt)
    }
    fn reregister(
        &self,
        poll: &Poll,
        token: Token,
        interest: Ready,
        opt: PollOpt,
    ) -> io::Result<()> {
        EventedFd(&self.0).reregister(poll, token, interest, opt)
    }
    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        EventedFd(&self.0).deregister(poll)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let sock = RawHciSocket::new(true).unwrap();
        sock.bind(0).unwrap();

        sock.send(&[0x01, 0x03, 0x0c, 0x00][..]).unwrap();

        let mut buf = [0; 32];
        let len = sock.recv(&mut buf).unwrap();
        let buf = &buf[..len];

        assert_eq!(buf[0], 0x04); // evt
        assert_eq!(buf[1], 0x0e); // command complete
        assert_eq!(buf[2], 0x04); // len
                                  //assert_eq!(buf[3], 0x??); // ncmd
        assert_eq!(buf[4], 0x03); // opcode[0]
        assert_eq!(buf[5], 0x0c); // opcode[1]
        assert_eq!(buf[6], 0x00); // status
    }
}
