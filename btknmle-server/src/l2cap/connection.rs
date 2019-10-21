use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::{Buf as _, BufMut as _, Bytes, BytesMut, IntoBuf as _};
use failure::Fail;
use futures::channel::mpsc;
use futures::{ready, Sink, Stream};

use btknmle_pkt::hci::acldata::{AclData, AclFlags};

#[derive(Debug, Fail)]
pub enum SendError {
    #[fail(display = "Send error {}", _0)]
    SendError(#[fail(cause)] mpsc::SendError),
}

impl From<mpsc::SendError> for SendError {
    fn from(v: mpsc::SendError) -> Self {
        Self::SendError(v)
    }
}

#[derive(Debug)]
pub struct L2capConnection {
    handle: u16,
    cid: u16,
    rx: mpsc::Receiver<AclData>,
    tx: mpsc::Sender<AclData>,
    pending: Option<(usize, BytesMut)>,
}

impl L2capConnection {
    pub fn cid(&self) -> u16 {
        self.cid
    }
}

impl L2capConnection {
    pub(crate) fn new(
        handle: u16,
        cid: u16,
        rx: mpsc::Receiver<AclData>,
        tx: mpsc::Sender<AclData>,
    ) -> Self {
        Self {
            handle,
            cid,
            rx,
            tx,
            pending: None,
        }
    }
}

impl Stream for L2capConnection {
    type Item = io::Result<Bytes>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            match ready!(Pin::new(&mut self.rx).poll_next(cx)) {
                Some(acl) => {
                    if acl.flags().contains(AclFlags::ACL_START) {
                        let mut data = acl.data().into_buf();
                        let len = data.get_u16_le() as usize;
                        let cid = data.get_u16_le();
                        let mut pending = BytesMut::with_capacity(len);
                        pending.extend(data.take(len).collect::<Bytes>());

                        if self.handle != acl.handle() {
                            panic!("{} != {}", self.handle, acl.handle())
                        }
                        if self.cid != cid {
                            panic!("{} != {}", self.cid, cid)
                        }

                        if pending.len() < len {
                            self.pending = Some((len, pending));
                        } else {
                            return Poll::Ready(Some(Ok(pending.freeze())));
                        }
                    } else if acl.flags().contains(AclFlags::ACL_CONT) {
                        let needmore = match self.pending.as_mut() {
                            Some((len, buf)) => {
                                buf.extend(acl.data());
                                buf.len() < *len
                            }
                            None => panic!(),
                        };

                        if !needmore {
                            let mut result = None;
                            std::mem::swap(&mut self.pending, &mut result);
                            return Poll::Ready(Some(Ok(result.unwrap().1.freeze())));
                        }
                    } else {
                        let mut data = acl.data().into_buf();
                        let len = data.get_u16_le() as usize;
                        let cid = data.get_u16_le();
                        let data = data.take(len).collect::<Bytes>();
                        self.pending = None;

                        if self.handle != acl.handle() {
                            panic!("{} != {}", self.handle, acl.handle())
                        }
                        if self.cid != cid {
                            panic!("{} != {}", self.cid, cid)
                        }
                        if len != data.len() {
                            panic!("{} != {}", len, data.len())
                        }

                        return Poll::Ready(Some(Ok(data)));
                    }
                }
                None => return Poll::Ready(None),
            };
        }
    }
}

impl Sink<Bytes> for L2capConnection {
    type Error = SendError;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.tx).poll_ready(cx).map_err(|e| e.into())
    }

    fn start_send(mut self: Pin<&mut Self>, item: Bytes) -> Result<(), Self::Error> {
        let mut data = BytesMut::with_capacity(item.len() + 4);
        data.put_u16_le(item.len() as u16);
        data.put_u16_le(self.cid);
        data.put(item);

        let acldata = AclData::new(AclFlags::ACL_START_NO_FLUSH, self.handle, data.freeze());
        Pin::new(&mut self.tx)
            .start_send(acldata)
            .map_err(|e| e.into())
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.tx).poll_flush(cx).map_err(|e| e.into())
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.tx).poll_close(cx).map_err(|e| e.into())
    }
}
