use std::io;

use bytes::{BytesMut, IntoBuf};
use failure::Fail;
use futures::{Sink, SinkExt as _, Stream, StreamExt as _};
use log::debug;
use tokio::codec::{Decoder, Encoder};

use crate::pkt::mgmt::{
    self, Advertising, CurrentSettings, Discoverable, ManagementCommand, MgmtCommand, MgmtEvent,
    SetLocalNameCommandResult, Status,
};
use crate::pkt::{Codec as _, CodecError};
use crate::sock::{Framed, MgmtSocket};

pub use crate::pkt::mgmt as model;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "CodecError {}", _0)]
    CodecError(#[fail(cause)] CodecError),

    #[fail(display = "Io Error occurred {}", _0)]
    Io(#[fail(cause)] io::Error),

    #[fail(display = "Invalid state")]
    InvalidState,

    #[fail(display = "Command Error {:?}", _0)]
    CommandError(Status),
}

impl From<CodecError> for Error {
    fn from(v: CodecError) -> Self {
        Self::CodecError(v)
    }
}

impl From<io::Error> for Error {
    fn from(v: io::Error) -> Self {
        Self::Io(v)
    }
}

pub struct MgmtCodec;

impl Encoder for MgmtCodec {
    type Item = MgmtCommand;
    type Error = Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        debug!("> {:?}", item);
        item.write_to(dst)?;
        Ok(())
    }
}

impl Decoder for MgmtCodec {
    type Item = MgmtEvent;
    type Error = Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let result = Self::Item::parse(&mut buf.take().into_buf())?;
        debug!("< {:?}", result);
        Ok(Some(result))
    }
}

#[derive(Debug)]
pub struct Mgmt<IO> {
    index: u16,
    io: IO,
}

impl<IO> Mgmt<IO> {
    fn new_internal(index: u16, io: IO) -> Self {
        Self { index, io }
    }
}

impl Mgmt<Framed<MgmtSocket, MgmtCodec>> {
    pub async fn new(index: u16) -> Result<Self, Error> {
        let io = MgmtSocket::bind()?;
        let io = io.framed(MgmtCodec);
        Ok(Self::new_internal(index, io))
    }
}

impl<IO> Mgmt<IO>
where
    IO: Sink<MgmtCommand, Error = Error> + Stream<Item = Result<MgmtEvent, Error>> + Unpin,
{
    pub async fn powered(&mut self, powered: bool) -> Result<CurrentSettings, Error> {
        self.invoke(mgmt::SetPoweredCommand::new(self.index, powered))
            .await
    }

    pub async fn low_energy(&mut self, low_energy: bool) -> Result<CurrentSettings, Error> {
        self.invoke(mgmt::SetLowEnergyCommand::new(self.index, low_energy))
            .await
    }

    pub async fn br_edr(&mut self, br_edr: bool) -> Result<CurrentSettings, Error> {
        self.invoke(mgmt::SetBrEdrCommand::new(self.index, br_edr))
            .await
    }

    pub async fn connectable(&mut self, connectable: bool) -> Result<CurrentSettings, Error> {
        self.invoke(mgmt::SetConnectableCommand::new(self.index, connectable))
            .await
    }

    pub async fn bondable(&mut self, bondable: bool) -> Result<CurrentSettings, Error> {
        self.invoke(mgmt::SetBondableCommand::new(self.index, bondable))
            .await
    }

    pub async fn advertising(
        &mut self,
        advertising: Advertising,
    ) -> Result<CurrentSettings, Error> {
        self.invoke(mgmt::SetAdvertisingCommand::new(self.index, advertising))
            .await
    }

    pub async fn discoverable(
        &mut self,
        discoverable: Discoverable,
    ) -> Result<CurrentSettings, Error> {
        self.invoke(mgmt::SetDiscoverableCommand::new(self.index, discoverable))
            .await
    }

    pub async fn local_name(
        &mut self,
        name: impl ToString,
        short_name: impl ToString,
    ) -> Result<SetLocalNameCommandResult, Error> {
        let name = name.to_string();
        let short_name = short_name.to_string();
        self.invoke(mgmt::SetLocalNameCommand::new(self.index, name, short_name))
            .await
    }

    async fn invoke<I, O>(&mut self, msg: I) -> Result<O, Error>
    where
        I: ManagementCommand<O>,
    {
        self.io.send(msg.into()).await?;

        if let Some(evt) = self.io.next().await {
            let evt = evt?;
            match evt {
                MgmtEvent::CommandCompleteEvent(evt) => {
                    Ok(I::parse_result(&mut evt.parameters().into_buf())?)
                }
                MgmtEvent::CommandStatusEvent(evt) => Err(Error::CommandError(evt.status())),
            }
        } else {
            Err(Error::InvalidState)
        }
    }
}
