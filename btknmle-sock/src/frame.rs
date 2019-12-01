use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::{BufMut as _, BytesMut};
use futures::{ready, Sink, Stream};
use tokio_util::codec::{Decoder, Encoder};

use super::{L2Stream, MgmtSocket};

const INITIAL_RD_CAPACITY: usize = 64 * 1024; // FIXME
const INITIAL_WR_CAPACITY: usize = 8 * 1024; // FIXME

#[must_use = "sinks do nothing unless polled"]
#[derive(Debug)]
pub struct Framed<S, C> {
    socket: S,
    codec: C,
    rd: BytesMut,
    wr: BytesMut,
    flushed: bool,
}

impl<C> Stream for Framed<MgmtSocket, C>
where
    C: Decoder + Unpin,
{
    type Item = Result<C::Item, C::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let pin = self.get_mut();

        pin.rd.reserve(INITIAL_RD_CAPACITY);

        unsafe {
            let bytes = &mut *(pin.rd.bytes_mut() as *mut _ as *mut [u8]);
            let n = ready!(Pin::new(&mut pin.socket).poll_recv_priv(cx, bytes))?;
            pin.rd.advance_mut(n);
        }

        let frame_res = pin.codec.decode(&mut pin.rd);
        pin.rd.clear();
        let frame = frame_res?.map(|v| Ok(v));
        Poll::Ready(frame)
    }
}

impl<C> Stream for Framed<L2Stream, C>
where
    C: Decoder + Unpin,
{
    type Item = Result<C::Item, C::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let pin = self.get_mut();

        pin.rd.reserve(INITIAL_RD_CAPACITY);

        unsafe {
            let bytes = &mut *(pin.rd.bytes_mut() as *mut _ as *mut [u8]);
            let n = ready!(Pin::new(&mut pin.socket).poll_recv_priv(cx, bytes))?;
            pin.rd.advance_mut(n);
        }

        let frame_res = pin.codec.decode(&mut pin.rd);
        pin.rd.clear();
        let frame = frame_res?.map(|v| Ok(v));
        Poll::Ready(frame)
    }
}

impl<C> Sink<C::Item> for Framed<MgmtSocket, C>
where
    C: Encoder + Unpin,
{
    type Error = C::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        if !self.flushed {
            match self.poll_flush(cx)? {
                Poll::Ready(()) => {}
                Poll::Pending => return Poll::Pending,
            }
        }

        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, item: C::Item) -> Result<(), Self::Error> {
        let pin = self.get_mut();

        pin.codec.encode(item, &mut pin.wr)?;
        pin.flushed = false;

        Ok(())
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        if self.flushed {
            return Poll::Ready(Ok(()));
        }

        let Self {
            ref mut socket,
            ref mut wr,
            ..
        } = *self;

        let n = ready!(socket.poll_send_priv(cx, &wr))?;

        let wrote_all = n == self.wr.len();
        self.wr.clear();
        self.flushed = true;

        let res = if wrote_all {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "failed to write entire datagram to socket",
            )
            .into())
        };

        Poll::Ready(res)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        ready!(self.poll_flush(cx))?;
        Poll::Ready(Ok(()))
    }
}

impl<C> Sink<C::Item> for Framed<L2Stream, C>
where
    C: Encoder + Unpin,
{
    type Error = C::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        if !self.flushed {
            match self.poll_flush(cx)? {
                Poll::Ready(()) => {}
                Poll::Pending => return Poll::Pending,
            }
        }

        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, item: C::Item) -> Result<(), Self::Error> {
        let pin = self.get_mut();

        pin.codec.encode(item, &mut pin.wr)?;
        pin.flushed = false;

        Ok(())
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        if self.flushed {
            return Poll::Ready(Ok(()));
        }

        let Self {
            ref mut socket,
            ref mut wr,
            ..
        } = *self;

        let n = ready!(socket.poll_send_priv(cx, &wr))?;

        let wrote_all = n == self.wr.len();
        self.wr.clear();
        self.flushed = true;

        let res = if wrote_all {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "failed to write entire datagram to socket",
            )
            .into())
        };

        Poll::Ready(res)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        ready!(self.poll_flush(cx))?;
        Poll::Ready(Ok(()))
    }
}

impl<S, C> Framed<S, C> {
    pub fn new(socket: S, codec: C) -> Framed<S, C> {
        Self {
            socket,
            codec,
            rd: BytesMut::with_capacity(INITIAL_RD_CAPACITY),
            wr: BytesMut::with_capacity(INITIAL_WR_CAPACITY),
            flushed: true,
        }
    }

    pub fn get_ref(&self) -> &S {
        &self.socket
    }

    pub fn get_mut(&mut self) -> &mut S {
        &mut self.socket
    }

    pub fn into_inner(self) -> S {
        self.socket
    }
}
