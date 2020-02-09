use std::io;
use std::mem::size_of;
use std::os::unix::io::RawFd;

use mio::event::Evented;
use mio::unix::EventedFd;
use mio::{Poll, PollOpt, Ready, Token};

#[repr(C)]
#[derive(Debug, Default)]
struct socketaddr_hci {
    hci_family: libc::sa_family_t,
    hci_dev: libc::c_ushort,
    hci_channel: libc::c_ushort,
}

#[repr(C)]
#[derive(Debug, Default)]
struct socketaddr_l2 {
    l2_family: libc::sa_family_t,
    l2_psm: libc::c_ushort,
    l2_bdaddr: [u8; 6],
    l2_cid: libc::c_ushort,
    l2_bdaddr_type: u8,
}

const BTPROTO_L2CAP: libc::c_int = 0;
const BTPROTO_HCI: libc::c_int = 1;
//const BTPROTO_SCO: libc::c_int = 2;
//const BTPROTO_RFCOMM: libc::c_int = 3;
//const BTPROTO_BNEP: libc::c_int = 4;
//const BTPROTO_CMTP: libc::c_int = 5;
//const BTPROTO_HIDP: libc::c_int = 6;
//const BTPROTO_AVDTP: libc::c_int = 7;

const HCI_DEV_NONE: libc::c_ushort = 0xffff;

//const HCI_CHANNEL_RAW: libc::c_ushort = 0;
//const HCI_CHANNEL_USER: libc::c_ushort = 1;
//const HCI_CHANNEL_MONITOR: libc::c_ushort = 2;
const HCI_CHANNEL_CONTROL: libc::c_ushort = 3;
//const HCI_CHANNEL_LOGGING: libc::c_ushort = 4;

//const BDADDR_BREDR: u8 = 0x00;
const BDADDR_LE_PUBLIC: u8 = 0x01;
//const BDADDR_LE_RANDOM: u8 = 0x02;

#[derive(Debug)]
pub(crate) struct RawSocket(RawFd);

impl RawSocket {
    pub(crate) fn new_mgmt() -> io::Result<Self> {
        let r = unsafe {
            libc::socket(
                libc::AF_BLUETOOTH,
                libc::SOCK_RAW | libc::SOCK_CLOEXEC | libc::SOCK_NONBLOCK,
                BTPROTO_HCI,
            )
        };
        if r < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(Self(r as RawFd))
        }
    }

    pub(crate) fn new_l2cap() -> io::Result<Self> {
        let r = unsafe {
            libc::socket(
                libc::AF_BLUETOOTH,
                libc::SOCK_SEQPACKET | libc::SOCK_CLOEXEC | libc::SOCK_NONBLOCK,
                BTPROTO_L2CAP,
            )
        };
        if r < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(Self(r as RawFd))
        }
    }

    pub(crate) fn bind_mgmt(&self) -> io::Result<()> {
        let addr = socketaddr_hci {
            hci_family: (libc::AF_BLUETOOTH as libc::sa_family_t),
            hci_dev: HCI_DEV_NONE,
            hci_channel: HCI_CHANNEL_CONTROL,
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

    pub(crate) fn bind_l2cap(&self, cid: u16) -> io::Result<()> {
        let addr = socketaddr_l2 {
            l2_family: (libc::AF_BLUETOOTH as libc::sa_family_t),
            l2_psm: Default::default(),
            l2_cid: cid.to_le(),
            l2_bdaddr: [0; 6],
            l2_bdaddr_type: BDADDR_LE_PUBLIC,
        };

        let r = unsafe {
            libc::bind(
                self.0,
                &addr as *const _ as *const libc::sockaddr,
                size_of::<socketaddr_l2>() as libc::c_uint,
            )
        };

        if r == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }

    pub(crate) fn listen(&self, backlog: libc::c_int) -> io::Result<usize> {
        let r = unsafe { libc::listen(self.0, backlog) };
        if r < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(r as usize)
        }
    }

    pub(crate) fn accept(&self) -> io::Result<RawSocket> {
        let mut addr = socketaddr_l2::default();
        let mut len = size_of::<socketaddr_l2>() as libc::socklen_t;
        let r = unsafe {
            libc::accept(
                self.0,
                &mut addr as *mut _ as *mut libc::sockaddr,
                &mut len as *mut _,
            )
        };
        if r < 0 {
            Err(io::Error::last_os_error())
        } else {
            unsafe {
                let n = libc::fcntl(r, libc::F_GETFL);
                if n < 0 {
                    return Err(io::Error::last_os_error());
                }
                if libc::fcntl(r, libc::F_SETFL, n | libc::O_NONBLOCK) < 0 {
                    return Err(io::Error::last_os_error());
                }
            }
            Ok(RawSocket(r))
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

impl Drop for RawSocket {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.0);
        }
    }
}

impl Evented for RawSocket {
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
        fn assert_send<T: Send>() {};
        fn assert_sync<T: Sync>() {};

        assert_send::<socketaddr_hci>();
        assert_sync::<socketaddr_hci>();

        assert_send::<socketaddr_l2>();
        assert_sync::<socketaddr_l2>();

        assert_send::<RawSocket>();
        assert_sync::<RawSocket>();
    }

    #[tokio::test]
    #[ignore]
    async fn test_mgmt() {
        let sock = RawSocket::new_mgmt().unwrap();
        sock.bind_mgmt().unwrap();
        sock.send([0x01, 0x00, 0xff, 0xff, 0x00, 0x00].as_ref())
            .unwrap();
        let mut buf = [0; 32];
        let len = sock.recv(&mut buf).unwrap();
        let buf = &buf[..len];

        assert_eq!(buf.len(), 12);
        assert_eq!(u16::from_le_bytes([buf[0], buf[1]]), 0x0001);
        assert_eq!(u16::from_le_bytes([buf[2], buf[3]]), 0xFFFF);
        assert_eq!(u16::from_le_bytes([buf[4], buf[5]]), 0x0006);
        assert_eq!(u16::from_le_bytes([buf[6], buf[7]]), 0x0001);
        assert_eq!(buf[8], 0);
    }

    #[tokio::test]
    #[ignore]
    async fn test_l2() {
        let sock = RawSocket::new_l2cap().unwrap();
        sock.bind_l2cap(0x0004).unwrap();
        sock.listen(0).unwrap();
        sock.accept().ok();
    }
}
