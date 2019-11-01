use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::{ready, Stream};

use super::l2_listener::L2Listener;
use super::l2_stream::L2Stream;

#[must_use = "streams do nothing unless polled"]
#[derive(Debug)]
pub struct L2Incoming {
    inner: L2Listener,
}

impl L2Incoming {
    pub(crate) fn new(listener: L2Listener) -> Self {
        Self { inner: listener }
    }
}

impl Stream for L2Incoming {
    type Item = io::Result<L2Stream>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let socket = ready!(self.inner.poll_accept(cx))?;
        Poll::Ready(Some(Ok(socket)))
    }
}
