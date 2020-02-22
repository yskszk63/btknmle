use std::io;
use std::task::{Context, Poll};

use futures::ready;
use tokio::io::PollEvented;

use crate::l2_incoming::L2Incoming;
use crate::l2_stream::L2Stream;
use crate::raw::{self, RawSocket};

#[derive(Debug)]
pub enum AttSecurityLevel {
    NeedsBoundMitm,
    NeedsBound,
    None,
}

#[derive(Debug)]
pub struct L2Listener {
    io: PollEvented<RawSocket>,
}

impl L2Listener {
    pub fn bind_att(level: AttSecurityLevel) -> io::Result<Self> {
        Self::bind(
            0x0004,
            match level {
                AttSecurityLevel::NeedsBoundMitm => raw::BT_SECURITY_HIGH,
                AttSecurityLevel::NeedsBound => raw::BT_SECURITY_MEDIUM,
                AttSecurityLevel::None => raw::BT_SECURITY_LOW,
            },
        )
    }

    pub(crate) fn bind(cid: u16, level: u8) -> io::Result<Self> {
        let inner = RawSocket::new_l2cap()?;
        inner.set_sockopt_l2cap_security(level)?;
        inner.bind_l2cap(cid)?;
        inner.listen(1)?;
        Ok(Self {
            io: PollEvented::new(inner)?,
        })
    }

    pub(crate) fn poll_accept(&self, cx: &mut Context<'_>) -> Poll<io::Result<L2Stream>> {
        ready!(self.io.poll_read_ready(cx, mio::Ready::readable()))?;

        match self.io.get_ref().accept() {
            Ok(x) => Poll::Ready(L2Stream::new(x)),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                self.io.clear_read_ready(cx, mio::Ready::readable())?;
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e)),
        }
    }

    pub fn incoming(self) -> L2Incoming {
        L2Incoming::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        fn assert_send<T: Send>() {};
        fn assert_sync<T: Sync>() {};

        assert_send::<L2Listener>();
        assert_sync::<L2Listener>();
    }

    #[tokio::test]
    #[ignore]
    async fn test_l2() {
        L2Listener::bind(0x0004, raw::BT_SECURITY_LOW).unwrap();
    }
}
