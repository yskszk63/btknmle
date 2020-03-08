use std::collections::VecDeque;
use std::io;
use std::num::NonZeroU8;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::BytesMut;
use futures::{Sink, SinkExt as _, Stream, StreamExt as _};
use log::debug;
use thiserror::Error;
use tokio_util::codec::{Decoder, Encoder};

use crate::pkt::mgmt::{
    command::{
        self as cmd, Advertising, Discoverable, IoCapability, ManagementCommand, MgmtCommand,
        ReadControllerInformationResult, SecureConnections, SetLocalNameCommandResult,
    },
    event::MgmtEvent,
    Action, Address, AddressType, AdvertisingFlags, CurrentSettings, IdentityResolvingKey,
    LongTermKey, Status,
};
use crate::pkt::{PackError, PacketData, UnpackError};
use crate::sock::{Framed, MgmtSocket};

pub use crate::pkt::mgmt as model;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("invalid state")]
    InvalidState,

    #[error("command error {0:?}")]
    CommandError(Status),

    #[error("invalid event {0:?}")]
    InvalidEvent(MgmtEvent),

    #[error(transparent)]
    UnpackError(#[from] UnpackError),

    #[error(transparent)]
    PackError(#[from] PackError),
}

#[derive(Debug)]
pub struct MgmtCodec;

impl Encoder<MgmtCommand> for MgmtCodec {
    type Error = Error;

    fn encode(&mut self, item: MgmtCommand, dst: &mut BytesMut) -> Result<(), Self::Error> {
        debug!("> {:?}", item);
        item.pack(dst)?;
        Ok(())
    }
}

impl Decoder for MgmtCodec {
    type Item = MgmtEvent;
    type Error = Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let result = Self::Item::unpack(buf)?;
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
        self.invoke(cmd::SetPoweredCommand::new(powered)).await
    }

    pub async fn low_energy(&mut self, low_energy: bool) -> Result<CurrentSettings, Error> {
        self.invoke(cmd::SetLowEnergyCommand::new(low_energy)).await
    }

    pub async fn br_edr(&mut self, br_edr: bool) -> Result<CurrentSettings, Error> {
        self.invoke(cmd::SetBrEdrCommand::new(br_edr)).await
    }

    pub async fn connectable(&mut self, connectable: bool) -> Result<CurrentSettings, Error> {
        self.invoke(cmd::SetConnectableCommand::new(connectable))
            .await
    }

    pub async fn bondable(&mut self, bondable: bool) -> Result<CurrentSettings, Error> {
        self.invoke(cmd::SetBondableCommand::new(bondable)).await
    }

    pub async fn user_confirmation(
        &mut self,
        address: Address,
        address_type: AddressType,
    ) -> Result<(Address, AddressType), Error> {
        self.invoke(cmd::UserConfirmationReplyCommand::new(
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
        self.invoke(cmd::UserConfirmationNegativeReplyCommand::new(
            address,
            address_type,
        ))
        .await
    }

    pub async fn secure_connections(
        &mut self,
        secure_connections: SecureConnections,
    ) -> Result<CurrentSettings, Error> {
        self.invoke(cmd::SetSecureConnectionsCommand::new(secure_connections))
            .await
    }

    pub async fn privacy(
        &mut self,
        privacy: bool,
        identity_resolving_key: [u8; 16],
    ) -> Result<CurrentSettings, Error> {
        self.invoke(cmd::SetPrivacyCommand::new(privacy, identity_resolving_key))
            .await
    }

    pub async fn io_capability(&mut self, io_capability: IoCapability) -> Result<(), Error> {
        self.invoke(cmd::SetIoCapabilityCommand::new(io_capability))
            .await
    }

    pub async fn advertising(
        &mut self,
        advertising: Advertising,
    ) -> Result<CurrentSettings, Error> {
        self.invoke(cmd::SetAdvertisingCommand::new(advertising))
            .await
    }

    pub async fn discoverable(
        &mut self,
        discoverable: Discoverable,
    ) -> Result<CurrentSettings, Error> {
        self.invoke(cmd::SetDiscoverableCommand::new(discoverable))
            .await
    }

    pub async fn load_ltks(&mut self, keys: Vec<LongTermKey>) -> Result<(), Error> {
        self.invoke(cmd::LoadLongTermKeysCommand::new(keys)).await
    }

    pub async fn load_irks(&mut self, keys: Vec<IdentityResolvingKey>) -> Result<(), Error> {
        self.invoke(cmd::LoadIdentityResolvingKeysCommand::new(keys))
            .await
    }

    pub async fn appearance(&mut self, appearance: u16) -> Result<(), Error> {
        self.invoke(cmd::SetAppearanceCommand::new(appearance))
            .await
    }

    pub async fn local_name(
        &mut self,
        name: impl ToString,
        short_name: impl ToString,
    ) -> Result<SetLocalNameCommandResult, Error> {
        let name = name.to_string();
        let short_name = short_name.to_string();
        self.invoke(cmd::SetLocalNameCommand::new(name, short_name))
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
        self.invoke(cmd::AddAdvertisingCommand::new(
            instance, flags, duration, timeout, adv_data, scan_rsp,
        ))
        .await
    }

    pub async fn remove_advertising(&mut self, instance: Option<NonZeroU8>) -> Result<u8, Error> {
        self.invoke(cmd::RemoveAdvertisingCommand::new(instance))
            .await
    }

    pub async fn user_passkey_reply(
        &mut self,
        addr: Address,
        addr_type: AddressType,
        passkey: u32,
    ) -> Result<(Address, AddressType), Error> {
        self.invoke(cmd::UserPasskeyReplyCommand::new(addr, addr_type, passkey))
            .await
    }

    pub async fn add_device(
        &mut self,
        address: Address,
        address_type: AddressType,
        action: Action,
    ) -> Result<(Address, AddressType), Error> {
        self.invoke(cmd::AddDeviceCommand::new(address, address_type, action))
            .await
    }

    pub async fn remove_device(
        &mut self,
        address: Address,
        address_type: AddressType,
    ) -> Result<(Address, AddressType), Error> {
        self.invoke(cmd::RemoveDeviceCommand::new(address, address_type))
            .await
    }

    pub async fn read_controller_information(
        &mut self,
    ) -> Result<ReadControllerInformationResult, Error> {
        self.invoke(cmd::ReadControllerInformationCommand::new())
            .await
    }

    async fn invoke<I, O>(&mut self, msg: I) -> Result<O, Error>
    where
        O: PacketData,
        I: ManagementCommand<Result = O>,
    {
        self.io.send(msg.into_mgmt(self.index.into())).await?;

        while let Some(evt) = self.io.next().await {
            let evt = evt?;
            match evt {
                MgmtEvent::CommandCompleteEvent(_, evt) => {
                    return Ok(I::unpack_result(&mut evt.parameters())?)
                }
                MgmtEvent::CommandStatusEvent(_, evt) => {
                    return Err(Error::CommandError(evt.status()))
                }
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
