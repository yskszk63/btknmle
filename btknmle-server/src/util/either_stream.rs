use std::pin::Pin;
use std::task::{Context, Poll};

use futures::future::Either;
use futures::stream::{Fuse, FusedStream};
use futures::{Stream, StreamExt as _};

#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub(crate) struct EitherStream<L, R> {
    left: Fuse<L>,
    right: Fuse<R>,
    flag: bool,
}

impl<L, R> EitherStream<L, R> {
    pub(crate) fn get_mut(&mut self) -> (&mut L, &mut R) {
        (self.left.get_mut(), self.right.get_mut())
    }
}

impl<L, R> EitherStream<L, R>
where
    L: Stream,
    R: Stream,
{
    pub(crate) fn new(left: L, right: R) -> Self {
        let left = left.fuse();
        let right = right.fuse();
        Self {
            left,
            right,
            flag: false,
        }
    }
}

impl<L: Unpin, R: Unpin> Unpin for EitherStream<L, R> {}

impl<L, R> Stream for EitherStream<L, R>
where
    L: Stream,
    R: Stream,
{
    type Item = Either<<L as Stream>::Item, <R as Stream>::Item>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let Self { flag, left, right } = unsafe { self.get_unchecked_mut() };
        let left = unsafe { Pin::new_unchecked(left) };
        let right = unsafe { Pin::new_unchecked(right) };

        if *flag {
            match left.poll_next(cx) {
                Poll::Ready(Some(item)) => {
                    *flag = !*flag;
                    return Poll::Ready(Some(Either::Left(item)));
                }
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Pending => (),
            }

            match right.poll_next(cx) {
                Poll::Ready(Some(item)) => Poll::Ready(Some(Either::Right(item))),
                Poll::Ready(None) => Poll::Ready(None),
                Poll::Pending => Poll::Pending,
            }
        } else {
            match right.poll_next(cx) {
                Poll::Ready(Some(item)) => {
                    *flag = !*flag;
                    return Poll::Ready(Some(Either::Right(item)));
                }
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Pending => (),
            }

            match left.poll_next(cx) {
                Poll::Ready(Some(item)) => Poll::Ready(Some(Either::Left(item))),
                Poll::Ready(None) => Poll::Ready(None),
                Poll::Pending => Poll::Pending,
            }
        }
    }
}

impl<L, R> FusedStream for EitherStream<L, R>
where
    L: Stream,
    R: Stream,
{
    fn is_terminated(&self) -> bool {
        self.left.is_terminated() || self.right.is_terminated()
    }
}
