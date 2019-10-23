use std::pin::Pin;
use std::task::{Context, Poll};

use futures::{ready, Stream};

use crate::sock::{L2Listener, L2Incoming, L2Stream, Framed};
use crate::att::AttCodec;
use crate::pkt::att::ATT_CID;
use super::{Database, GattConnection, Result};

#[derive(Debug)]
pub struct GattListener {
    io: L2Incoming,
    db: Database,
}

impl GattListener {
    pub fn new(db: Database) -> Result<Self> {
        let listener = L2Listener::bind(ATT_CID)?;
        let io = listener.incoming();
        Ok(Self { io, db })
    }
}

impl Stream for GattListener {
    type Item = Result<GattConnection<Framed<L2Stream, AttCodec>>>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match ready!(Pin::new(&mut self.io).poll_next(cx)) {
            Some(sock) => {
                let sock = sock?.framed(AttCodec);
                Poll::Ready(Some(Ok(GattConnection::new(self.db.clone(), sock))))
            },
            None => Poll::Ready(None),
        }
    }
}
