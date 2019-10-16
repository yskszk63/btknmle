use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::BytesMut;
use either::{Either, Left, Right};
use futures::{ready, Sink, Stream};

use btknmle_pkt::att::Att;
use btknmle_pkt::hci::HciPacket;
use btknmle_pkt::Codec as _;

use crate::l2cap::{Handle, L2CapTransport};

#[derive(Debug)]
pub struct AttTransport<C> {
    inner: L2CapTransport<C>,
}

impl<C> AttTransport<C> {
    pub fn new(inner: L2CapTransport<C>) -> Self {
        Self { inner }
    }
}

impl<C, E> Stream for AttTransport<C>
where
    C: Stream<Item = Result<HciPacket, E>> + Unpin,
{
    type Item = Result<Either<(Handle, Att), HciPacket>, E>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match ready!(Pin::new(&mut self.inner).poll_next(cx)) {
            Some(e) => match e? {
                Left((handle, data)) => {
                    dbg!(&data);
                    let acl = Att::parse(&mut io::Cursor::new(data)).unwrap();
                    Poll::Ready(Some(Ok(Left((handle, acl)))))
                }
                Right(v) => Poll::Ready(Some(Ok(Right(v)))),
            },
            None => Poll::Ready(None),
        }
    }
}

impl<C, E> Sink<(Handle, Att)> for AttTransport<C>
where
    C: Sink<HciPacket, Error = E> + Unpin,
{
    type Error = E;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.inner).poll_ready(cx)
    }

    fn start_send(mut self: Pin<&mut Self>, item: (Handle, Att)) -> Result<(), Self::Error> {
        let mut data = BytesMut::new();
        item.1.write_to(&mut data).unwrap(); // FIXME

        Pin::new(&mut self.inner).start_send((item.0, data.freeze()))
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.inner).poll_close(cx)
    }
}
