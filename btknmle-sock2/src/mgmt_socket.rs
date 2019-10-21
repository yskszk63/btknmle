use std::io;
use std::task::{Context, Poll};

use futures::future::poll_fn;
use futures::ready;
use tokio_net::util::PollEvented;

use crate::raw::RawSocket;
use crate::frame::Framed;

#[derive(Debug)]
pub struct MgmtSocket {
    io: PollEvented<RawSocket>,
}

impl MgmtSocket {
    pub fn bind() -> io::Result<Self> {
        let inner = RawSocket::new_mgmt()?;
        inner.bind_mgmt()?;
        Ok(Self {
            io: PollEvented::new(inner),
        })
    }

    pub async fn send(&mut self, buf: &[u8]) -> io::Result<usize> {
        poll_fn(|cx| self.poll_send_priv(cx, buf)).await
    }

    pub(crate) fn poll_send_priv(
        &self,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        ready!(self.io.poll_write_ready(cx))?;

        match self.io.get_ref().send(buf) {
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                self.io.clear_write_ready(cx)?;
                Poll::Pending
            }
            x => Poll::Ready(x),
        }
    }

    pub async fn recv(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        poll_fn(|cx| self.poll_recv_priv(cx, buf)).await
    }

    pub(crate) fn poll_recv_priv(
        &self,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        ready!(self.io.poll_read_ready(cx, mio::Ready::readable()))?;

        match self.io.get_ref().recv(buf) {
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                self.io.clear_read_ready(cx, mio::Ready::readable())?;
                Poll::Pending
            }
            x => Poll::Ready(x),
        }
    }

    pub fn framed<C>(self, codec: C) -> Framed<Self, C> {
        Framed::new(self, codec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::{BytesMut, BufMut};

    #[tokio::test]
    async fn test() {
        let mut sock = MgmtSocket::bind().unwrap();
        let mut command = BytesMut::new();
        command.put_u16_le(0x0001);
        command.put_u16_le(0xFFFF);
        command.put_u16_le(0x0000);
        println!("{:?}", command);
        sock.send(&command.freeze()).await.unwrap();

        let mut buf = [0; 32];
        sock.recv(&mut buf).await.unwrap();
        println!("{:?}", buf);
    }

    #[tokio::test]
    async fn test2() {
        use tokio::codec::BytesCodec;
        use tokio::prelude::*;

        let mut sock = MgmtSocket::bind().unwrap().framed(BytesCodec::new());

        let mut command = BytesMut::new();
        command.put_u16_le(0x0001);
        command.put_u16_le(0xFFFF);
        command.put_u16_le(0x0000);
        sock.send(command.freeze()).await.unwrap();

        let result = sock.next().await.unwrap();
        println!("{:?}", result);

        let mut command = BytesMut::new();
        command.put_u16_le(0x0003);
        command.put_u16_le(0xFFFF);
        command.put_u16_le(0x0000);
        sock.send(command.freeze()).await.unwrap();

        let result = sock.next().await.unwrap();
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test3() {
        use tokio::codec::BytesCodec;
        use tokio::prelude::*;

        let mut sock = MgmtSocket::bind().unwrap().framed(BytesCodec::new());

        // Powered
        let mut command = BytesMut::new();
        command.put_u16_le(0x0005);
        command.put_u16_le(0x0000);
        command.put_u16_le(0x0001);
        command.put_u8(0x00);
        sock.send(command.freeze()).await.unwrap();

        let result = sock.next().await.unwrap();
        println!("{:?}", result);

        // Name
        let mut command = BytesMut::with_capacity(512);
        command.put_u16_le(0x000F);
        command.put_u16_le(0x0000);
        command.put_u16_le(0x0104);
        command.put("ABC");
        command.put(&[0; 246][..]);
        command.put("ABC");
        command.put(&[0; 8][..]);
        sock.send(command.freeze()).await.unwrap();

        let result = sock.next().await.unwrap();
        println!("{:?}", result);

        // Powered
        let mut command = BytesMut::new();
        command.put_u16_le(0x0005);
        command.put_u16_le(0x0000);
        command.put_u16_le(0x0001);
        command.put_u8(0x01);
        sock.send(command.freeze()).await.unwrap();

        let result = sock.next().await.unwrap();
        println!("{:?}", result);

        // Connectable
        let mut command = BytesMut::new();
        command.put_u16_le(0x0007);
        command.put_u16_le(0x0000);
        command.put_u16_le(0x0001);
        command.put_u8(0x01);
        sock.send(command.freeze()).await.unwrap();

        let result = sock.next().await.unwrap();
        println!("{:?}", result);
 
        // Bondable
        let mut command = BytesMut::new();
        command.put_u16_le(0x0009);
        command.put_u16_le(0x0000);
        command.put_u16_le(0x0001);
        command.put_u8(0x01);
        sock.send(command.freeze()).await.unwrap();

        let result = sock.next().await.unwrap();
        println!("{:?}", result);

        // LE Energy
        let mut command = BytesMut::new();
        command.put_u16_le(0x000D);
        command.put_u16_le(0x0000);
        command.put_u16_le(0x0001);
        command.put_u8(0x01);
        sock.send(command.freeze()).await.unwrap();

        let result = sock.next().await.unwrap();
        println!("{:?}", result);

        // Advertise
        let mut command = BytesMut::new();
        command.put_u16_le(0x0029);
        command.put_u16_le(0x0000);
        command.put_u16_le(0x0001);
        command.put_u8(0x02);
        sock.send(command.freeze()).await.unwrap();

        let result = sock.next().await.unwrap();
        println!("{:?}", result);
    }
}
