use std::pin::Pin;
use std::task::{Context, Poll};

use futures::{ready, Stream, Sink};
use bytes::{Bytes, BytesMut, Buf as _, BufMut as _, IntoBuf as _};
use either::{Either, Left, Right};

use btknmle_pkt::HciPacket;
use btknmle_pkt::acldata::{AclData, AclFlags};

const ATT_CID: u16 = 0x04;

#[derive(Debug)]
pub struct Handle(u16);

#[derive(Debug)]
pub struct L2CapTransport<C> {
    inner: C,
}

impl<C> L2CapTransport<C> {
    pub fn new(inner: C) -> Self {
        Self { inner }
    }
}

impl<C, E> Stream for L2CapTransport<C> where C: Stream<Item=Result<HciPacket, E>> + Unpin {
    type Item = Result<Either<(Handle, Bytes), HciPacket>, E>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match ready!(Pin::new(&mut self.inner).poll_next(cx)) {
            Some(e) => match e? {
                HciPacket::Acldata(item) => {
                    let handle = item.handle();
                    if item.flags().contains(AclFlags::ACL_CONT) {
                        panic!("{:?}", item.flags())
                    }
                    let mut data = item.data().into_buf();
                    let len = data.get_u16_le() as usize;
                    let cid = data.get_u16_le();
                    if cid != ATT_CID {
                        panic!("{}", cid);
                    }
                    let data = data.take(len).iter().collect::<Bytes>();
                    if data.len() != len {
                        panic!("{}", len);
                    }
                    let handle = Handle(handle);
                    Poll::Ready(Some(Ok(Left((handle, data)))))
                }
                e => Poll::Ready(Some(Ok(Right(e))))
            }
            None => Poll::Ready(None)
        }
    }
}

impl<C, E> Sink<(Handle, Bytes)> for L2CapTransport<C> where C: Sink<HciPacket, Error=E> + Unpin {
    type Error = E;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.inner).poll_ready(cx)
    }

    fn start_send(mut self: Pin<&mut Self>, item: (Handle, Bytes)) -> Result<(), Self::Error> {
        let mut data = BytesMut::with_capacity(item.1.len() + 4);
        data.put_u16_le(item.1.len() as u16);
        data.put_u16_le(ATT_CID);
        data.put(item.1);

        let acldata = AclData::new(AclFlags::ACL_START_NO_FLUSH, (item.0).0, data.freeze());
        Pin::new(&mut self.inner).start_send(HciPacket::Acldata(acldata))
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.inner).poll_close(cx)
    }

}

