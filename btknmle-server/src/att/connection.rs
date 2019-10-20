use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::{Bytes, BytesMut, IntoBuf as _};
use failure::Fail;
use futures::{ready, Sink, Stream};

use crate::l2cap::SendError;
use btknmle_pkt::att::Att;
use btknmle_pkt::{Codec as _, CodecError};

#[derive(Debug, Fail)]
pub enum AttError {
    #[fail(display = "Send error {}", _0)]
    SendError(#[fail(cause)] SendError),
    #[fail(display = "IO error {}", _0)]
    Io(#[fail(cause)] io::Error),
    #[fail(display = "Codec error {}", _0)]
    Codec(#[fail(cause)] CodecError),
}

impl From<io::Error> for AttError {
    fn from(v: io::Error) -> Self {
        Self::Io(v)
    }
}

impl From<CodecError> for AttError {
    fn from(v: CodecError) -> Self {
        Self::Codec(v)
    }
}

impl From<SendError> for AttError {
    fn from(v: SendError) -> Self {
        Self::SendError(v)
    }
}

#[derive(Debug)]
pub struct AttConnection<C>(C);

impl<C> AttConnection<C> {
    pub(crate) fn new(io: C) -> Self {
        Self(io)
    }
}

impl<C> Stream for AttConnection<C>
where
    C: Stream<Item = io::Result<Bytes>> + Unpin,
{
    type Item = Result<Att, AttError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match ready!(Pin::new(&mut self.0).poll_next(cx)) {
            Some(data) => Poll::Ready(Some(Ok(Att::parse(&mut data?.into_buf())?))),
            None => Poll::Ready(None),
        }
    }
}

impl<C> Sink<Att> for AttConnection<C>
where
    C: Sink<Bytes, Error = SendError> + Unpin,
{
    type Error = AttError;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.0).poll_ready(cx).map_err(|e| e.into())
    }

    fn start_send(mut self: Pin<&mut Self>, item: Att) -> Result<(), Self::Error> {
        let mut data = BytesMut::new();
        item.write_to(&mut data)?;
        Pin::new(&mut self.0)
            .start_send(data.freeze())
            .map_err(|e| e.into())
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.0).poll_flush(cx).map_err(|e| e.into())
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.0).poll_close(cx).map_err(|e| e.into())
    }
}
