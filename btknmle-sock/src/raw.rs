use std::io;
use std::mem::size_of;
use std::os::unix::io::RawFd;
use std::ptr;

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

#[repr(C)]
#[derive(Debug, Default)]
struct bt_security {
    level: u8,
    key_size: u8,
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

const SOL_BLUETOOTH: libc::c_int = 274;
const BT_SECURITY: libc::c_int = 4;
//pub(crate) const BT_SECURITY_SDP: u8 = 0;
pub(crate) const BT_SECURITY_LOW: u8 = 1;
pub(crate) const BT_SECURITY_MEDIUM: u8 = 2;
pub(crate) const BT_SECURITY_HIGH: u8 = 3;
//pub(crate) const BT_SECURITY_FIPS: u8 = 4;

#[derive(Debug)]
pub(crate) struct RawSocket(RawFd);

impl RawSocket {
    #[cfg(test)]
    pub(crate) fn from_raw_fd(fd: RawFd) -> Self {
        RawSocket(fd)
    }

    fn new_internal(
        socket: unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> libc::c_int,
        domain: libc::c_int,
        r#type: libc::c_int,
        proto: libc::c_int,
    ) -> io::Result<Self> {
        let r = unsafe { socket(domain, r#type, proto) };
        if r < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(Self(r as RawFd))
        }
    }

    pub(crate) fn new_mgmt() -> io::Result<Self> {
        Self::new_internal(
            libc::socket,
            libc::AF_BLUETOOTH,
            libc::SOCK_RAW | libc::SOCK_CLOEXEC | libc::SOCK_NONBLOCK,
            BTPROTO_HCI,
        )
    }

    pub(crate) fn new_l2cap() -> io::Result<Self> {
        Self::new_internal(
            libc::socket,
            libc::AF_BLUETOOTH,
            libc::SOCK_SEQPACKET | libc::SOCK_CLOEXEC | libc::SOCK_NONBLOCK,
            BTPROTO_L2CAP,
        )
    }

    fn set_sockopt_l2cap_security_internal(
        &self,
        setsockopt: unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *const libc::c_void,
            libc::socklen_t,
        ) -> libc::c_int,
        level: u8,
    ) -> io::Result<()> {
        let sock = self.0;
        let value = bt_security {
            level,
            ..Default::default()
        };
        let len = size_of::<bt_security>() as libc::socklen_t;

        let r = unsafe {
            setsockopt(
                sock,
                SOL_BLUETOOTH,
                BT_SECURITY,
                &value as *const _ as *const libc::c_void,
                len,
            )
        };
        if r < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    pub(crate) fn set_sockopt_l2cap_security(&self, level: u8) -> io::Result<()> {
        self.set_sockopt_l2cap_security_internal(libc::setsockopt, level)
    }

    fn bind_internal<T>(
        &self,
        bind: unsafe extern "C" fn(
            libc::c_int,
            *const libc::sockaddr,
            libc::socklen_t,
        ) -> libc::c_int,
        addr: &T,
    ) -> io::Result<()> {
        let r = unsafe {
            bind(
                self.0,
                addr as *const _ as *const libc::sockaddr,
                size_of::<T>() as libc::socklen_t,
            )
        };

        if r == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }

    pub(crate) fn bind_mgmt(&self) -> io::Result<()> {
        let addr = socketaddr_hci {
            hci_family: (libc::AF_BLUETOOTH as libc::sa_family_t),
            hci_dev: HCI_DEV_NONE,
            hci_channel: HCI_CHANNEL_CONTROL,
        };
        self.bind_internal(libc::bind, &addr)
    }

    pub(crate) fn bind_l2cap(&self, cid: u16) -> io::Result<()> {
        let addr = socketaddr_l2 {
            l2_family: (libc::AF_BLUETOOTH as libc::sa_family_t),
            l2_psm: Default::default(),
            l2_cid: cid.to_le(),
            l2_bdaddr: [0; 6],
            l2_bdaddr_type: BDADDR_LE_PUBLIC,
        };
        self.bind_internal(libc::bind, &addr)
    }

    fn listen_internal(
        &self,
        listen: unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
        backlog: libc::c_int,
    ) -> io::Result<usize> {
        let r = unsafe { listen(self.0, backlog) };
        if r < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(r as usize)
        }
    }

    pub(crate) fn listen(&self, backlog: libc::c_int) -> io::Result<usize> {
        self.listen_internal(libc::listen, backlog)
    }

    fn accept_internal(
        &self,
        accept4: unsafe extern "C" fn(
            libc::c_int,
            *mut libc::sockaddr,
            *mut libc::socklen_t,
            libc::c_int,
        ) -> libc::c_int,
    ) -> io::Result<RawSocket> {
        let r = unsafe {
            accept4(
                self.0,
                ptr::null_mut() as *mut libc::sockaddr,
                ptr::null_mut() as *mut _,
                libc::SOCK_NONBLOCK | libc::SOCK_CLOEXEC,
            )
        };
        if r < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(RawSocket(r))
        }
    }

    pub(crate) fn accept(&self) -> io::Result<RawSocket> {
        self.accept_internal(libc::accept4)
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

    extern "C" fn test_new_sock(
        domain: libc::c_int,
        r#type: libc::c_int,
        proto: libc::c_int,
    ) -> libc::c_int {
        assert_eq!(domain, 0);
        assert_eq!(r#type, 1);
        assert_eq!(proto, 2);
        3
    }

    #[test]
    fn test_new_internal() {
        RawSocket::new_internal(test_new_sock, 0, 1, 2).unwrap();
    }

    extern "C" fn test_new_failed_sock(
        _domain: libc::c_int,
        _type: libc::c_int,
        _proto: libc::c_int,
    ) -> libc::c_int {
        errno::set_errno(errno::Errno(1));
        -1
    }

    #[test]
    fn test_new_internal_failed() {
        assert!(RawSocket::new_internal(test_new_failed_sock, 0, 1, 2).is_err())
    }

    unsafe extern "C" fn test_set_sockopt_l2cap_security_internal_setsockopt(
        socket: libc::c_int,
        level: libc::c_int,
        name: libc::c_int,
        value: *const libc::c_void,
        opt_len: libc::socklen_t,
    ) -> libc::c_int {
        assert_eq!(3, socket);
        assert_eq!(SOL_BLUETOOTH, level);
        assert_eq!(BT_SECURITY, name);
        let value = &*(value as *const bt_security);
        assert_eq!(1, value.level);
        assert_eq!(size_of::<bt_security>(), opt_len as usize);

        0
    }

    #[test]
    fn test_set_sockopt_l2cap_security_internal() {
        let sock = RawSocket::new_internal(test_new_sock, 0, 1, 2).unwrap();
        sock.set_sockopt_l2cap_security_internal(
            test_set_sockopt_l2cap_security_internal_setsockopt,
            1,
        )
        .unwrap();
    }

    unsafe extern "C" fn test_set_sockopt_l2cap_security_internal_setsockopt_failed(
        _socket: libc::c_int,
        _level: libc::c_int,
        _name: libc::c_int,
        _value: *const libc::c_void,
        _opt_len: libc::socklen_t,
    ) -> libc::c_int {
        errno::set_errno(errno::Errno(1));
        -1
    }

    #[test]
    fn test_set_sockopt_l2cap_security_internal_failed() {
        let sock = RawSocket::new_internal(test_new_sock, 0, 1, 2).unwrap();
        assert!(sock
            .set_sockopt_l2cap_security_internal(
                test_set_sockopt_l2cap_security_internal_setsockopt_failed,
                1
            )
            .is_err())
    }

    unsafe extern "C" fn test_bind_internal_bind(
        socket: libc::c_int,
        addr: *const libc::sockaddr,
        addr_len: libc::socklen_t,
    ) -> libc::c_int {
        assert_eq!(3, socket);
        let addr = &*(addr as *const libc::sockaddr_in);
        assert_eq!(1234, addr.sin_port);
        assert_eq!(0x1234_5678, addr.sin_addr.s_addr);
        assert_eq!(size_of::<libc::sockaddr_in>(), addr_len as usize);
        0
    }

    #[test]
    fn test_bind_internal() {
        let sock = RawSocket::new_internal(test_new_sock, 0, 1, 2).unwrap();
        sock.bind_internal(
            test_bind_internal_bind,
            &libc::sockaddr_in {
                sin_family: libc::AF_INET as u16,
                sin_port: 1234,
                sin_addr: libc::in_addr {
                    s_addr: 0x1234_5678,
                },
                sin_zero: [0; 8],
            },
        )
        .unwrap();
    }

    unsafe extern "C" fn test_bind_internal_bind_failed(
        _socket: libc::c_int,
        _addr: *const libc::sockaddr,
        _addr_len: libc::socklen_t,
    ) -> libc::c_int {
        errno::set_errno(errno::Errno(1));
        -1
    }

    #[test]
    fn test_bind_internal_failed() {
        let sock = RawSocket::new_internal(test_new_sock, 0, 1, 2).unwrap();
        let result = sock
            .bind_internal(
                test_bind_internal_bind_failed,
                &libc::sockaddr_in {
                    sin_family: libc::AF_INET as u16,
                    sin_port: 1234,
                    sin_addr: libc::in_addr {
                        s_addr: 0x1234_5678,
                    },
                    sin_zero: [0; 8],
                },
            )
            .is_err();
        assert!(result);
    }

    unsafe extern "C" fn test_listen_internal_listen(
        socket: libc::c_int,
        backlog: libc::c_int,
    ) -> libc::c_int {
        assert_eq!(3, socket);
        assert_eq!(1, backlog);
        0
    }

    #[test]
    fn test_listen_internal() {
        let sock = RawSocket::new_internal(test_new_sock, 0, 1, 2).unwrap();
        sock.listen_internal(test_listen_internal_listen, 1)
            .unwrap();
    }

    unsafe extern "C" fn test_listen_internal_listen_failed(
        _socket: libc::c_int,
        _backlog: libc::c_int,
    ) -> libc::c_int {
        errno::set_errno(errno::Errno(1));
        -1
    }

    #[test]
    fn test_listen_internal_failed() {
        let sock = RawSocket::new_internal(test_new_sock, 0, 1, 2).unwrap();
        assert!(sock
            .listen_internal(test_listen_internal_listen_failed, 1)
            .is_err());
    }

    unsafe extern "C" fn test_accept_internal_accept4(
        socket: libc::c_int,
        addr: *mut libc::sockaddr,
        len: *mut libc::socklen_t,
        flag: libc::c_int,
    ) -> libc::c_int {
        assert_eq!(3, socket);
        assert!(std::ptr::eq(addr, std::ptr::null_mut()));
        assert!(std::ptr::eq(len, std::ptr::null_mut()));
        assert_eq!(flag, libc::SOCK_NONBLOCK | libc::SOCK_CLOEXEC);
        0
    }

    #[test]
    fn test_accept_internal() {
        let sock = RawSocket::new_internal(test_new_sock, 0, 1, 2).unwrap();
        sock.accept_internal(test_accept_internal_accept4).unwrap();
    }

    unsafe extern "C" fn test_accept_internal_accept4_failed(
        _socket: libc::c_int,
        _addr: *mut libc::sockaddr,
        _len: *mut libc::socklen_t,
        _flag: libc::c_int,
    ) -> libc::c_int {
        errno::set_errno(errno::Errno(1));
        -1
    }

    #[test]
    fn test_accept_internal_failed() {
        let sock = RawSocket::new_internal(test_new_sock, 0, 1, 2).unwrap();
        assert!(sock
            .accept_internal(test_accept_internal_accept4_failed)
            .is_err());
    }

    #[test]
    fn test_rxtx() {
        use mio::{Events, Poll, PollOpt, Ready, Token};
        use std::os::unix::io::IntoRawFd;
        use std::os::unix::net::UnixDatagram;

        let poll = loop {
            match Poll::new() {
                Ok(poll) => break poll,
                Err(e) => eprintln!("{:?}", e),
            }
        };

        let (socka, sockb) = UnixDatagram::pair().unwrap();
        socka.set_nonblocking(true).unwrap();
        sockb.set_nonblocking(true).unwrap();

        let socka = RawSocket(socka.into_raw_fd());
        let sockb = RawSocket(sockb.into_raw_fd());

        let mut events = Events::with_capacity(1024);
        poll.register(&socka, Token(0), Ready::readable(), PollOpt::edge())
            .unwrap();
        sockb.send(&[0, 1, 2]).unwrap();
        //poll.register(&sockb, tokenb, Ready::writable(), PollOpt::edge()).unwrap();

        poll.poll(&mut events, None).unwrap();
        assert!(!events.is_empty());
        for event in events {
            match event.token() {
                Token(0) => {
                    let mut buf = [0; 16];
                    let n = socka.recv(&mut buf).unwrap();
                    assert_eq!(&[0, 1, 2], &buf[..n]);
                }
                _ => unreachable!(),
            }
        }
    }
}
