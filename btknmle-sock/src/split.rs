use std::fmt;
use std::io;
use std::sync::Arc;

use futures::future::poll_fn;

use super::HciSocket;

#[derive(Debug)]
pub struct HciSocketSendHalf(Arc<HciSocket>);

#[derive(Debug)]
pub struct HciSocketRecvHalf(Arc<HciSocket>);

pub(crate) fn split(socket: HciSocket) -> (HciSocketRecvHalf, HciSocketSendHalf) {
    let shared = Arc::new(socket);
    (HciSocketRecvHalf(shared.clone()), HciSocketSendHalf(shared))
}

#[derive(Debug)]
pub struct ReuniteError(pub HciSocketSendHalf, pub HciSocketRecvHalf);

impl fmt::Display for ReuniteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "tried to reunite halves that are not from the same socket"
        )
    }
}

impl std::error::Error for ReuniteError {}

fn reunite(s: HciSocketSendHalf, r: HciSocketRecvHalf) -> Result<HciSocket, ReuniteError> {
    if Arc::ptr_eq(&s.0, &r.0) {
        drop(r);

        Ok(Arc::try_unwrap(s.0).expect("try_unwrap failed in reunite"))
    } else {
        Err(ReuniteError(s, r))
    }
}

impl HciSocketRecvHalf {
    pub fn reunite(self, other: HciSocketSendHalf) -> Result<HciSocket, ReuniteError> {
        reunite(other, self)
    }

    pub async fn recv(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        poll_fn(|cx| self.0.poll_recv_priv(cx, buf)).await
    }
}

impl HciSocketSendHalf {
    pub fn reunite(self, other: HciSocketRecvHalf) -> Result<HciSocket, ReuniteError> {
        reunite(self, other)
    }

    pub async fn send(&mut self, buf: &[u8]) -> io::Result<usize> {
        poll_fn(|cx| self.0.poll_send_priv(cx, buf)).await
    }
}

impl AsRef<HciSocket> for HciSocketRecvHalf {
    fn as_ref(&self) -> &HciSocket {
        &self.0
    }
}

impl AsRef<HciSocket> for HciSocketSendHalf {
    fn as_ref(&self) -> &HciSocket {
        &self.0
    }
}
