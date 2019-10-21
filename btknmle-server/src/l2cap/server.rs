use std::collections::HashMap;
use std::io;

use bytes::Bytes;
use failure::Fail;
use futures::channel::mpsc::{self, channel, Sender};
use futures::future::Either::{Left, Right};
use futures::{Sink, SinkExt as _, Stream, StreamExt as _};
use log::debug;

use super::L2capConnection;
use crate::util::EitherStream;
use btknmle_pkt::hci::acldata::{AclData, AclFlags};
use btknmle_pkt::hci::event::Event;
use btknmle_pkt::hci::HciPacket;

#[derive(Debug, Fail)]
pub enum ServeError {
    #[fail(display = "IO Error occurred {}", _0)]
    Io(#[fail(cause)] io::Error),
    #[fail(display = "Send Error occurred {}", _0)]
    SendError(mpsc::SendError),
}

impl From<io::Error> for ServeError {
    fn from(v: io::Error) -> Self {
        Self::Io(v)
    }
}

impl From<mpsc::SendError> for ServeError {
    fn from(v: mpsc::SendError) -> Self {
        Self::SendError(v)
    }
}

#[derive(Debug)]
struct ConnectionInfo {
    handle: u16,
    cid: u16,
    send: Sender<AclData>,
}

#[derive(Debug)]
pub struct L2capServer<H> {
    io: H,
}

impl<H> L2capServer<H> {
    pub fn inner_mut(&mut self) -> &mut H {
        &mut self.io
    }
}

fn cid(v: &Bytes) -> u16 {
    ((v[2] as u16) << 0) | ((v[3] as u16) << 8)
}

async fn on_acldata(
    data: AclData,
    connection_infos: &mut HashMap<(u16, u16), ConnectionInfo>,
    sender: &mpsc::Sender<AclData>,
    on_accept: impl Fn(L2capConnection),
    pending_channel: &mut HashMap<u16, u16>,
) -> Result<(), ServeError> {
    let handle = data.handle();

    let cid = if data.flags().contains(AclFlags::ACL_START) {
        let cid = cid(data.data());
        pending_channel.insert(handle, cid);
        cid
    } else if data.flags().contains(AclFlags::ACL_CONT) {
        *(pending_channel.get(&handle).unwrap())
    } else {
        pending_channel.remove(&handle);
        cid(data.data())
    };

    let info = match connection_infos.get_mut(&(handle, cid)) {
        Some(info) => info,
        None => {
            let (tx, rx) = channel(2);
            let connection = L2capConnection::new(handle, cid, rx, sender.clone());
            on_accept(connection);
            connection_infos.insert(
                (handle, cid),
                ConnectionInfo {
                    handle,
                    cid,
                    send: tx,
                },
            );
            connection_infos.get_mut(&(handle, cid)).unwrap()
        }
    };

    info.send.send(data).await?;
    Ok(())
}

impl<H> L2capServer<H>
where
    H: Sink<HciPacket, Error = io::Error> + Stream<Item = Result<HciPacket, io::Error>> + Unpin,
{
    pub fn new(io: H) -> Self {
        Self { io }
    }

    pub async fn serve(&mut self, on_accept: impl Fn(L2capConnection)) -> Result<(), ServeError> {
        let mut connection_infos = HashMap::new();
        let mut pending_channel = HashMap::new();
        let (tx, rx) = channel(2);
        let mut io = EitherStream::new(&mut self.io, rx);

        while let Some(result) = io.next().await {
            match result {
                Left(packet) => match packet? {
                    HciPacket::Event(Event::DisconnComplete(..)) => break,
                    HciPacket::Acldata(acldata) => {
                        on_acldata(
                            acldata,
                            &mut connection_infos,
                            &tx,
                            &on_accept,
                            &mut pending_channel,
                        )
                        .await?
                    }
                    x => debug!("unimplemented {:?}", x),
                },
                Right(packet) => {
                    io.get_mut().0.send(HciPacket::Acldata(packet)).await?;
                }
            }
        }

        Ok(())
    }
}
