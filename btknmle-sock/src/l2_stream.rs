use std::io;
use std::task::{Context, Poll};

use futures::future::poll_fn;
use futures::ready;
use tokio::io::PollEvented;

use crate::frame::Framed;
use crate::raw::RawSocket;

#[derive(Debug)]
pub struct L2Stream {
    io: PollEvented<RawSocket>,
}

impl L2Stream {
    pub(crate) fn new(io: RawSocket) -> io::Result<Self> {
        Ok(Self {
            io: PollEvented::new(io)?,
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

    #[test]
    fn test() {
        fn assert_send<T: Send>() {};
        fn assert_sync<T: Sync>() {};

        assert_send::<L2Stream>();
        assert_sync::<L2Stream>();
    }

    #[tokio::test]
    async fn test_rxtx() {
        use std::os::unix::net::UnixDatagram;
        use std::os::unix::io::IntoRawFd;
        use crate::raw::RawSocket;

        let (socka, sockb) = UnixDatagram::pair().unwrap();
        socka.set_nonblocking(true).unwrap();
        sockb.set_nonblocking(true).unwrap();

        let mut socka = L2Stream::new(RawSocket::from_raw_fd(socka.into_raw_fd())).unwrap();
        let mut sockb = L2Stream::new(RawSocket::from_raw_fd(sockb.into_raw_fd())).unwrap();

        tokio::spawn(async move {
            sockb.send(&[1, 2, 3]).await.unwrap();
        });

        let mut buf = vec![];
        socka.recv(&mut buf).await.unwrap();
    }
}
