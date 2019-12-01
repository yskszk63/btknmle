use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use failure::Fail;
use futures::{ready, Stream, TryStream};
use tokio::sync::watch;

#[derive(Debug)]
pub struct CancelableStreamController {
    sender: watch::Sender<Message>,
    factory: CancelableStreamFactory,
}

impl CancelableStreamController {
    pub fn new() -> Self {
        let (sender, receiver) = watch::channel(Message::Running);
        let factory = CancelableStreamFactory { receiver };
        Self { sender, factory }
    }

    pub fn cancel(&mut self) {
        self.sender.broadcast(Message::Canceled).unwrap() // FIXME
    }

    pub fn factory(&self) -> CancelableStreamFactory {
        self.factory.clone()
    }
}

#[derive(Debug, Clone)]
pub struct CancelableStreamFactory {
    receiver: watch::Receiver<Message>,
}

impl CancelableStreamFactory {
    pub fn with_stream<S, E>(&self, stream: S) -> CancelableStream<S, E> {
        CancelableStream {
            inner: stream,
            receiver: self.receiver.clone(),
            _phantom: PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Running,
    Canceled,
}

#[derive(Debug, Fail)]
#[fail(display = "Canceled")]
pub struct Canceled;

#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct CancelableStream<S, E> {
    inner: S,
    receiver: watch::Receiver<Message>,
    _phantom: PhantomData<E>,
}

impl<S, E> Stream for CancelableStream<S, E>
where
    S: TryStream + Unpin,
    E: From<<S as TryStream>::Error> + From<Canceled> + Unpin,
{
    type Item = Result<<S as TryStream>::Ok, E>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<<S as TryStream>::Ok, E>>> {
        match Pin::new(&mut self.receiver).poll_next(cx) {
            Poll::Ready(Some(Message::Canceled)) | Poll::Ready(None) => {
                return Poll::Ready(Some(Err(Canceled.into())))
            }
            Poll::Ready(Some(Message::Running)) | Poll::Pending => (),
        }

        match ready!(Pin::new(&mut self.inner).try_poll_next(cx)) {
            Some(Ok(e)) => Poll::Ready(Some(Ok(e))),
            Some(Err(e)) => Poll::Ready(Some(Err(e.into()))),
            None => Poll::Ready(None),
        }
    }
}
