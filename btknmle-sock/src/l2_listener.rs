use std::io;
use std::task::{Context, Poll};

use futures::future::poll_fn;
use futures::ready;
use tokio::io::PollEvented;

use crate::l2_incoming::L2Incoming;
use crate::l2_stream::L2Stream;
use crate::raw::RawSocket;

#[derive(Debug)]
pub struct L2Listener {
    io: PollEvented<RawSocket>,
}

impl L2Listener {
    pub fn bind(cid: u16) -> io::Result<Self> {
        let inner = RawSocket::new_l2cap()?;
        inner.bind_l2cap(cid)?;
        inner.listen(1)?;
        Ok(Self {
            io: PollEvented::new(inner)?,
        })
    }

    pub async fn accept(&mut self) -> io::Result<L2Stream> {
        poll_fn(|cx| self.poll_accept(cx)).await
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
    async fn _test() {
        use crate::MgmtSocket;
        use bytes::{BufMut, BytesMut};
        use futures::sink::SinkExt as _;
        use futures::stream::StreamExt as _;
        use tokio_util::codec::BytesCodec;

        let mut mgmt = MgmtSocket::bind().unwrap().framed(BytesCodec::new());
        // Advertise
        let mut command = BytesMut::new();
        command.put_u16_le(0x0029);
        command.put_u16_le(0x0000);
        command.put_u16_le(0x0001);
        command.put_u8(0x02);
        mgmt.send(command.freeze()).await.unwrap();

        let result = mgmt.next().await.unwrap();
        println!("{:?}", result);

        let sock = L2Listener::bind(0x0004).unwrap();
        let mut incoming = sock.incoming();
        while let Some(sock) = incoming.next().await {
            println!("{:?}", sock);
            let mut sock = sock.unwrap().framed(BytesCodec::new());
            while let Some(v) = sock.next().await {
                println!("{:?}", v);
            }
        }
    }
}
