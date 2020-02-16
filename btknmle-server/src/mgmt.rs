use std::collections::VecDeque;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::BytesMut;
use failure::Fail;
use futures::{Sink, SinkExt as _, Stream, StreamExt as _};
use log::debug;
use tokio_util::codec::{Decoder, Encoder};

use crate::pkt::mgmt::{
    self, Action, Address, AddressType, Advertising, AdvertisingFlags, CurrentSettings,
    Discoverable, IdentityResolvingKey, IoCapability, LongTermKey, ManagementCommand, MgmtCommand,
    MgmtEvent, SecureConnections, SetLocalNameCommandResult, Status,
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

    #[fail(display = "Invalid event {:?}", _0)]
    InvalidEvent(MgmtEvent),
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

#[derive(Debug)]
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
        let result = Self::Item::parse(buf)?;
        debug!("< {:?}", result);
        Ok(Some(result))
    }
}

#[derive(Debug)]
pub struct Mgmt<IO> {
    index: u16,
    io: IO,
    pending: VecDeque<MgmtEvent>,
}

impl<IO> Mgmt<IO> {
    fn new_internal(index: u16, io: IO) -> Self {
        Self {
            index,
            io,
            pending: VecDeque::new(),
        }
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

    pub async fn user_confirmation(
        &mut self,
        address: Address,
        address_type: AddressType,
    ) -> Result<(Address, AddressType), Error> {
        self.invoke(mgmt::UserConfirmationReplyCommand::new(
            self.index,
            address,
            address_type,
        ))
        .await
    }

    pub async fn user_confirmation_negative(
        &mut self,
        address: Address,
        address_type: AddressType,
    ) -> Result<(Address, AddressType), Error> {
        self.invoke(mgmt::UserConfirmationNegativeReplyCommand::new(
            self.index,
            address,
            address_type,
        ))
        .await
    }

    pub async fn secure_connections(
        &mut self,
        secure_connections: SecureConnections,
    ) -> Result<CurrentSettings, Error> {
        self.invoke(mgmt::SetSecureConnectionsCommand::new(
            self.index,
            secure_connections,
        ))
        .await
    }

    pub async fn privacy(
        &mut self,
        privacy: bool,
        identity_resolving_key: [u8; 16],
    ) -> Result<CurrentSettings, Error> {
        self.invoke(mgmt::SetPrivacyCommand::new(
            self.index,
            privacy,
            identity_resolving_key,
        ))
        .await
    }

    pub async fn io_capability(&mut self, io_capability: IoCapability) -> Result<(), Error> {
        self.invoke(mgmt::SetIoCapabilityCommand::new(self.index, io_capability))
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

    pub async fn load_ltks(&mut self, keys: Vec<LongTermKey>) -> Result<(), Error> {
        self.invoke(mgmt::LoadLongTermKeysCommand::new(self.index, keys))
            .await
    }

    pub async fn load_irks(&mut self, keys: Vec<IdentityResolvingKey>) -> Result<(), Error> {
        self.invoke(mgmt::LoadIdentityResolvingKeysCommand::new(
            self.index, keys,
        ))
        .await
    }

    pub async fn appearance(&mut self, appearance: u16) -> Result<(), Error> {
        self.invoke(mgmt::SetAppearanceCommand::new(self.index, appearance))
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

    pub async fn add_advertising(
        &mut self,
        instance: u8,
        flags: AdvertisingFlags,
        duration: u16,
        timeout: u16,
        adv_data: &[u8],
        scan_rsp: &[u8],
    ) -> Result<u8, Error> {
        self.invoke(mgmt::AddAdvertisingCommand::new(
            self.index, instance, flags, duration, timeout, adv_data, scan_rsp,
        ))
        .await
    }

    pub async fn user_passkey_reply(
        &mut self,
        addr: Address,
        addr_type: AddressType,
        passkey: u32,
    ) -> Result<(Address, AddressType), Error> {
        self.invoke(mgmt::UserPasskeyReplyCommand::new(
            self.index, addr, addr_type, passkey,
        ))
        .await
    }

    pub async fn add_device(
        &mut self,
        address: Address,
        address_type: AddressType,
        action: Action,
    ) -> Result<(Address, AddressType), Error> {
        self.invoke(mgmt::AddDeviceCommand::new(
            self.index,
            address,
            address_type,
            action,
        ))
        .await
    }

    pub async fn remove_device(
        &mut self,
        address: Address,
        address_type: AddressType,
    ) -> Result<(Address, AddressType), Error> {
        self.invoke(mgmt::RemoveDeviceCommand::new(
            self.index,
            address,
            address_type,
        ))
        .await
    }

    async fn invoke<I, O>(&mut self, msg: I) -> Result<O, Error>
    where
        I: ManagementCommand<O>,
    {
        self.io.send(msg.into()).await?;

        while let Some(evt) = self.io.next().await {
            let evt = evt?;
            match evt {
                MgmtEvent::CommandCompleteEvent(evt) => return Ok(I::parse_result(&mut evt.parameters())?),
                MgmtEvent::CommandStatusEvent(evt) => return Err(Error::CommandError(evt.status())),
                evt => self.pending.push_back(evt),
            }
        }
        Err(Error::InvalidState)
    }
}

impl<IO> Stream for Mgmt<IO>
where
    IO: Stream<Item = Result<MgmtEvent, Error>> + Unpin,
{
    type Item = Result<MgmtEvent, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Some(pending) = self.pending.pop_back() {
            Poll::Ready(Some(Ok(pending)))
        } else {
            Pin::new(&mut self.io).poll_next(cx)
        }
    }
}
