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
        if buf.is_empty() {
            return Ok(None);
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tx() {
        use crate::pkt::mgmt::command::ReadControllerInformationCommand;
        use crate::pkt::mgmt::ControlIndex;
        use futures::sink::SinkExt;
        use futures::stream::StreamExt;
        use tokio::net::UnixStream;
        use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite};

        let (socka, sockb) = UnixStream::pair().unwrap();

        let mut socka = FramedRead::new(socka, BytesCodec::new());
        let mut sockb = FramedWrite::new(sockb, MgmtCodec);

        tokio::spawn(async move {
            sockb
                .send(ReadControllerInformationCommand::new().into_mgmt(ControlIndex::from(0)))
                .await
                .unwrap();
        });

        let result = socka.next().await.unwrap().unwrap();
        let mut buf = BytesMut::new();
        ReadControllerInformationCommand::new()
            .into_mgmt(ControlIndex::from(0))
            .pack(&mut buf)
            .unwrap();
        assert_eq!(buf.freeze(), result.freeze());
    }

    #[tokio::test]
    async fn test_rx() {
        use crate::pkt::mgmt::event::{ConnectionFailedEvent, MgmtEvent};
        use crate::pkt::mgmt::{AddressType, ControlIndex};
        use futures::sink::SinkExt;
        use futures::stream::StreamExt;
        use tokio::net::UnixStream;
        use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite};

        let (socka, sockb) = UnixStream::pair().unwrap();

        let mut socka = FramedRead::new(socka, MgmtCodec);
        let mut sockb = FramedWrite::new(sockb, BytesCodec::new());

        tokio::spawn(async move {
            let mut buf = BytesMut::new();
            let event = ConnectionFailedEvent::new(
                "00:11:22:33:44:55".parse().unwrap(),
                AddressType::LePublic,
                1,
            );
            let event = MgmtEvent::ConnectionFailedEvent(ControlIndex::from(0), event);
            event.pack(&mut buf).unwrap();
            sockb.send(buf.freeze()).await.unwrap();
        });

        let result = socka.next().await.unwrap().unwrap();
        let event = ConnectionFailedEvent::new(
            "00:11:22:33:44:55".parse().unwrap(),
            AddressType::LePublic,
            1,
        );
        let event = MgmtEvent::ConnectionFailedEvent(ControlIndex::from(0), event);
        assert_eq!(event, result);
    }

    #[tokio::test]
    async fn test_mgmt_invoke() {
        use crate::pkt::mgmt::command::{
            Advertising, Discoverable, IoCapability, SecureConnections,
        };
        use crate::pkt::mgmt::command::{
            ReadControllerInformationResult, SetLocalNameCommandResult,
        };
        use crate::pkt::mgmt::event::{CommandCompleteEvent, MgmtEvent};
        use crate::pkt::mgmt::{
            Action, Address, AddressType, AdvertisingFlags, Code, ControlIndex, CurrentSettings,
            Status,
        };
        use futures::sink::SinkExt;
        use futures::stream::StreamExt;
        use std::str::FromStr;
        use tokio::net::UnixStream;
        use tokio::time::{timeout, Duration};
        use tokio_util::codec::{BytesCodec, Framed};

        let (socka, sockb) = UnixStream::pair().unwrap();

        let idx = ControlIndex::from(0);
        let mgmt = Framed::new(socka, MgmtCodec);
        let mut mgmt = Mgmt::new_internal(0, mgmt);
        let mut peer = Framed::new(sockb, BytesCodec::new());

        tokio::spawn(async move {
            // powered
            let mut buf = peer.next().await.unwrap().unwrap();
            let code = Code::unpack(&mut buf).unwrap();
            let mut buf = BytesMut::new();
            CurrentSettings::empty().pack(&mut buf).unwrap();
            let reply = CommandCompleteEvent::new(code, Status::Success, buf.freeze());
            let mut buf = BytesMut::new();
            MgmtEvent::CommandCompleteEvent(idx.clone(), reply)
                .pack(&mut buf)
                .unwrap();
            peer.send(buf.freeze()).await.unwrap();

            // low_energy
            let mut buf = peer.next().await.unwrap().unwrap();
            let code = Code::unpack(&mut buf).unwrap();
            let mut buf = BytesMut::new();
            CurrentSettings::empty().pack(&mut buf).unwrap();
            let reply = CommandCompleteEvent::new(code, Status::Success, buf.freeze());
            let mut buf = BytesMut::new();
            MgmtEvent::CommandCompleteEvent(idx.clone(), reply)
                .pack(&mut buf)
                .unwrap();
            peer.send(buf.freeze()).await.unwrap();

            // br_edr
            let mut buf = peer.next().await.unwrap().unwrap();
            let code = Code::unpack(&mut buf).unwrap();
            let mut buf = BytesMut::new();
            CurrentSettings::empty().pack(&mut buf).unwrap();
            let reply = CommandCompleteEvent::new(code, Status::Success, buf.freeze());
            let mut buf = BytesMut::new();
            MgmtEvent::CommandCompleteEvent(idx.clone(), reply)
                .pack(&mut buf)
                .unwrap();
            peer.send(buf.freeze()).await.unwrap();

            // connectable
            let mut buf = peer.next().await.unwrap().unwrap();
            let code = Code::unpack(&mut buf).unwrap();
            let mut buf = BytesMut::new();
            CurrentSettings::empty().pack(&mut buf).unwrap();
            let reply = CommandCompleteEvent::new(code, Status::Success, buf.freeze());
            let mut buf = BytesMut::new();
            MgmtEvent::CommandCompleteEvent(idx.clone(), reply)
                .pack(&mut buf)
                .unwrap();
            peer.send(buf.freeze()).await.unwrap();

            // bondable
            let mut buf = peer.next().await.unwrap().unwrap();
            let code = Code::unpack(&mut buf).unwrap();
            let mut buf = BytesMut::new();
            CurrentSettings::empty().pack(&mut buf).unwrap();
            let reply = CommandCompleteEvent::new(code, Status::Success, buf.freeze());
            let mut buf = BytesMut::new();
            MgmtEvent::CommandCompleteEvent(idx.clone(), reply)
                .pack(&mut buf)
                .unwrap();
            peer.send(buf.freeze()).await.unwrap();

            // secure_connections
            let mut buf = peer.next().await.unwrap().unwrap();
            let code = Code::unpack(&mut buf).unwrap();
            let mut buf = BytesMut::new();
            CurrentSettings::empty().pack(&mut buf).unwrap();
            let reply = CommandCompleteEvent::new(code, Status::Success, buf.freeze());
            let mut buf = BytesMut::new();
            MgmtEvent::CommandCompleteEvent(idx.clone(), reply)
                .pack(&mut buf)
                .unwrap();
            peer.send(buf.freeze()).await.unwrap();

            // privacy
            let mut buf = peer.next().await.unwrap().unwrap();
            let code = Code::unpack(&mut buf).unwrap();
            let mut buf = BytesMut::new();
            CurrentSettings::empty().pack(&mut buf).unwrap();
            let reply = CommandCompleteEvent::new(code, Status::Success, buf.freeze());
            let mut buf = BytesMut::new();
            MgmtEvent::CommandCompleteEvent(idx.clone(), reply)
                .pack(&mut buf)
                .unwrap();
            peer.send(buf.freeze()).await.unwrap();

            // advertising
            let mut buf = peer.next().await.unwrap().unwrap();
            let code = Code::unpack(&mut buf).unwrap();
            let mut buf = BytesMut::new();
            CurrentSettings::empty().pack(&mut buf).unwrap();
            let reply = CommandCompleteEvent::new(code, Status::Success, buf.freeze());
            let mut buf = BytesMut::new();
            MgmtEvent::CommandCompleteEvent(idx.clone(), reply)
                .pack(&mut buf)
                .unwrap();
            peer.send(buf.freeze()).await.unwrap();

            // discoverable
            let mut buf = peer.next().await.unwrap().unwrap();
            let code = Code::unpack(&mut buf).unwrap();
            let mut buf = BytesMut::new();
            CurrentSettings::empty().pack(&mut buf).unwrap();
            let reply = CommandCompleteEvent::new(code, Status::Success, buf.freeze());
            let mut buf = BytesMut::new();
            MgmtEvent::CommandCompleteEvent(idx.clone(), reply)
                .pack(&mut buf)
                .unwrap();
            peer.send(buf.freeze()).await.unwrap();

            // user_confirmation
            let mut buf = peer.next().await.unwrap().unwrap();
            let code = Code::unpack(&mut buf).unwrap();
            let mut buf = BytesMut::new();
            (
                Address::from_str("00:11:22:33:44:55").unwrap(),
                AddressType::LeRandom,
            )
                .pack(&mut buf)
                .unwrap();
            let reply = CommandCompleteEvent::new(code, Status::Success, buf.freeze());
            let mut buf = BytesMut::new();
            MgmtEvent::CommandCompleteEvent(idx.clone(), reply)
                .pack(&mut buf)
                .unwrap();
            peer.send(buf.freeze()).await.unwrap();

            // user_confirmation_negative
            let mut buf = peer.next().await.unwrap().unwrap();
            let code = Code::unpack(&mut buf).unwrap();
            let mut buf = BytesMut::new();
            (
                Address::from_str("00:11:22:33:44:55").unwrap(),
                AddressType::LeRandom,
            )
                .pack(&mut buf)
                .unwrap();
            let reply = CommandCompleteEvent::new(code, Status::Success, buf.freeze());
            let mut buf = BytesMut::new();
            MgmtEvent::CommandCompleteEvent(idx.clone(), reply)
                .pack(&mut buf)
                .unwrap();
            peer.send(buf.freeze()).await.unwrap();

            // io_capability
            let mut buf = peer.next().await.unwrap().unwrap();
            let code = Code::unpack(&mut buf).unwrap();
            let mut buf = BytesMut::new();
            ().pack(&mut buf).unwrap();
            let reply = CommandCompleteEvent::new(code, Status::Success, buf.freeze());
            let mut buf = BytesMut::new();
            MgmtEvent::CommandCompleteEvent(idx.clone(), reply)
                .pack(&mut buf)
                .unwrap();
            peer.send(buf.freeze()).await.unwrap();

            // load_ltks
            let mut buf = peer.next().await.unwrap().unwrap();
            let code = Code::unpack(&mut buf).unwrap();
            let mut buf = BytesMut::new();
            ().pack(&mut buf).unwrap();
            let reply = CommandCompleteEvent::new(code, Status::Success, buf.freeze());
            let mut buf = BytesMut::new();
            MgmtEvent::CommandCompleteEvent(idx.clone(), reply)
                .pack(&mut buf)
                .unwrap();
            peer.send(buf.freeze()).await.unwrap();

            // load_irks
            let mut buf = peer.next().await.unwrap().unwrap();
            let code = Code::unpack(&mut buf).unwrap();
            let mut buf = BytesMut::new();
            ().pack(&mut buf).unwrap();
            let reply = CommandCompleteEvent::new(code, Status::Success, buf.freeze());
            let mut buf = BytesMut::new();
            MgmtEvent::CommandCompleteEvent(idx.clone(), reply)
                .pack(&mut buf)
                .unwrap();
            peer.send(buf.freeze()).await.unwrap();

            // appearance
            let mut buf = peer.next().await.unwrap().unwrap();
            let code = Code::unpack(&mut buf).unwrap();
            let mut buf = BytesMut::new();
            ().pack(&mut buf).unwrap();
            let reply = CommandCompleteEvent::new(code, Status::Success, buf.freeze());
            let mut buf = BytesMut::new();
            MgmtEvent::CommandCompleteEvent(idx.clone(), reply)
                .pack(&mut buf)
                .unwrap();
            peer.send(buf.freeze()).await.unwrap();

            // local_name
            let mut buf = peer.next().await.unwrap().unwrap();
            let code = Code::unpack(&mut buf).unwrap();
            let mut buf = BytesMut::new();
            SetLocalNameCommandResult::new("abc", "def")
                .pack(&mut buf)
                .unwrap();
            let reply = CommandCompleteEvent::new(code, Status::Success, buf.freeze());
            let mut buf = BytesMut::new();
            MgmtEvent::CommandCompleteEvent(idx.clone(), reply)
                .pack(&mut buf)
                .unwrap();
            peer.send(buf.freeze()).await.unwrap();

            // add_advertising
            let mut buf = peer.next().await.unwrap().unwrap();
            let code = Code::unpack(&mut buf).unwrap();
            let mut buf = BytesMut::new();
            8u8.pack(&mut buf).unwrap();
            let reply = CommandCompleteEvent::new(code, Status::Success, buf.freeze());
            let mut buf = BytesMut::new();
            MgmtEvent::CommandCompleteEvent(idx.clone(), reply)
                .pack(&mut buf)
                .unwrap();
            peer.send(buf.freeze()).await.unwrap();

            // remove_advertising
            let mut buf = peer.next().await.unwrap().unwrap();
            let code = Code::unpack(&mut buf).unwrap();
            let mut buf = BytesMut::new();
            8u8.pack(&mut buf).unwrap();
            let reply = CommandCompleteEvent::new(code, Status::Success, buf.freeze());
            let mut buf = BytesMut::new();
            MgmtEvent::CommandCompleteEvent(idx.clone(), reply)
                .pack(&mut buf)
                .unwrap();
            peer.send(buf.freeze()).await.unwrap();

            // user_passkey_reply
            let mut buf = peer.next().await.unwrap().unwrap();
            let code = Code::unpack(&mut buf).unwrap();
            let mut buf = BytesMut::new();
            (
                Address::from_str("00:11:22:33:44:55").unwrap(),
                AddressType::LeRandom,
            )
                .pack(&mut buf)
                .unwrap();
            let reply = CommandCompleteEvent::new(code, Status::Success, buf.freeze());
            let mut buf = BytesMut::new();
            MgmtEvent::CommandCompleteEvent(idx.clone(), reply)
                .pack(&mut buf)
                .unwrap();
            peer.send(buf.freeze()).await.unwrap();

            // add_device
            let mut buf = peer.next().await.unwrap().unwrap();
            let code = Code::unpack(&mut buf).unwrap();
            let mut buf = BytesMut::new();
            (
                Address::from_str("00:11:22:33:44:55").unwrap(),
                AddressType::LeRandom,
            )
                .pack(&mut buf)
                .unwrap();
            let reply = CommandCompleteEvent::new(code, Status::Success, buf.freeze());
            let mut buf = BytesMut::new();
            MgmtEvent::CommandCompleteEvent(idx.clone(), reply)
                .pack(&mut buf)
                .unwrap();
            peer.send(buf.freeze()).await.unwrap();

            // remove_device
            let mut buf = peer.next().await.unwrap().unwrap();
            let code = Code::unpack(&mut buf).unwrap();
            let mut buf = BytesMut::new();
            (
                Address::from_str("00:11:22:33:44:55").unwrap(),
                AddressType::LeRandom,
            )
                .pack(&mut buf)
                .unwrap();
            let reply = CommandCompleteEvent::new(code, Status::Success, buf.freeze());
            let mut buf = BytesMut::new();
            MgmtEvent::CommandCompleteEvent(idx.clone(), reply)
                .pack(&mut buf)
                .unwrap();
            peer.send(buf.freeze()).await.unwrap();

            // read_controller_information
            let mut buf = peer.next().await.unwrap().unwrap();
            let code = Code::unpack(&mut buf).unwrap();
            let mut buf = BytesMut::new();
            ReadControllerInformationResult::new(
                Address::from_str("00:11:22:33:44:55").unwrap(),
                8,
                12,
                CurrentSettings::empty(),
                CurrentSettings::empty(),
                [1; 3],
                "abc".into(),
                "def".into(),
            )
            .pack(&mut buf)
            .unwrap();
            let reply = CommandCompleteEvent::new(code, Status::Success, buf.freeze());
            let mut buf = BytesMut::new();
            MgmtEvent::CommandCompleteEvent(idx.clone(), reply)
                .pack(&mut buf)
                .unwrap();
            peer.send(buf.freeze()).await.unwrap();
        });

        // powered
        let r = timeout(Duration::from_secs(1u64), mgmt.powered(true))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(CurrentSettings::empty(), r);

        // low_energy
        let r = timeout(Duration::from_secs(1u64), mgmt.low_energy(true))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(CurrentSettings::empty(), r);

        // br_edr
        let r = timeout(Duration::from_secs(1u64), mgmt.br_edr(true))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(CurrentSettings::empty(), r);

        // connectable
        let r = timeout(Duration::from_secs(1u64), mgmt.connectable(true))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(CurrentSettings::empty(), r);

        // bondable
        let r = timeout(Duration::from_secs(1u64), mgmt.bondable(true))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(CurrentSettings::empty(), r);

        // secure_connections
        let r = timeout(
            Duration::from_secs(1u64),
            mgmt.secure_connections(SecureConnections::Only),
        )
        .await
        .unwrap()
        .unwrap();
        assert_eq!(CurrentSettings::empty(), r);

        // privacy
        let r = timeout(Duration::from_secs(1u64), mgmt.privacy(true, [0; 16]))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(CurrentSettings::empty(), r);

        // advertising
        let r = timeout(
            Duration::from_secs(1u64),
            mgmt.advertising(Advertising::Enabled),
        )
        .await
        .unwrap()
        .unwrap();
        assert_eq!(CurrentSettings::empty(), r);

        // discoverable
        let r = timeout(
            Duration::from_secs(1u64),
            mgmt.discoverable(Discoverable::General),
        )
        .await
        .unwrap()
        .unwrap();
        assert_eq!(CurrentSettings::empty(), r);

        // user_confirmation
        let r = timeout(
            Duration::from_secs(1u64),
            mgmt.user_confirmation(
                Address::from_str("00:11:22:33:44:55").unwrap(),
                AddressType::LeRandom,
            ),
        )
        .await
        .unwrap()
        .unwrap();
        assert_eq!(
            (
                Address::from_str("00:11:22:33:44:55").unwrap(),
                AddressType::LeRandom
            ),
            r
        );

        // user_confirmation_negative
        let r = timeout(
            Duration::from_secs(1u64),
            mgmt.user_confirmation_negative(
                Address::from_str("00:11:22:33:44:55").unwrap(),
                AddressType::LeRandom,
            ),
        )
        .await
        .unwrap()
        .unwrap();
        assert_eq!(
            (
                Address::from_str("00:11:22:33:44:55").unwrap(),
                AddressType::LeRandom
            ),
            r
        );

        // io_capability
        let r = timeout(
            Duration::from_secs(1u64),
            mgmt.io_capability(IoCapability::DisplayOnly),
        )
        .await
        .unwrap()
        .unwrap();
        assert_eq!((), r);

        // load_ltks
        let r = timeout(Duration::from_secs(1u64), mgmt.load_ltks(vec![]))
            .await
            .unwrap()
            .unwrap();
        assert_eq!((), r);

        // load_irks
        let r = timeout(Duration::from_secs(1u64), mgmt.load_irks(vec![]))
            .await
            .unwrap()
            .unwrap();
        assert_eq!((), r);

        // appearance
        let r = timeout(Duration::from_secs(1u64), mgmt.appearance(123))
            .await
            .unwrap()
            .unwrap();
        assert_eq!((), r);

        // local_name
        let r = timeout(Duration::from_secs(1u64), mgmt.local_name("abc", "def"))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(SetLocalNameCommandResult::new("abc", "def"), r);

        // add_advertising
        let r = timeout(
            Duration::from_secs(1u64),
            mgmt.add_advertising(1, AdvertisingFlags::empty(), 1, 2, &[], &[]),
        )
        .await
        .unwrap()
        .unwrap();
        assert_eq!(8, r);

        // remove_advertising
        let r = timeout(Duration::from_secs(1u64), mgmt.remove_advertising(None))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(8, r);

        // user_passkey_reply
        let r = timeout(
            Duration::from_secs(1u64),
            mgmt.user_passkey_reply(
                Address::from_str("00:11:22:33:44:55").unwrap(),
                AddressType::LeRandom,
                12345,
            ),
        )
        .await
        .unwrap()
        .unwrap();
        assert_eq!(
            (
                Address::from_str("00:11:22:33:44:55").unwrap(),
                AddressType::LeRandom
            ),
            r
        );

        // add_device
        let r = timeout(
            Duration::from_secs(1u64),
            mgmt.add_device(
                Address::from_str("00:11:22:33:44:55").unwrap(),
                AddressType::LeRandom,
                Action::BackgroundScanForDevice,
            ),
        )
        .await
        .unwrap()
        .unwrap();
        assert_eq!(
            (
                Address::from_str("00:11:22:33:44:55").unwrap(),
                AddressType::LeRandom
            ),
            r
        );

        // remove_device
        let r = timeout(
            Duration::from_secs(1u64),
            mgmt.remove_device(
                Address::from_str("00:11:22:33:44:55").unwrap(),
                AddressType::LeRandom,
            ),
        )
        .await
        .unwrap()
        .unwrap();
        assert_eq!(
            (
                Address::from_str("00:11:22:33:44:55").unwrap(),
                AddressType::LeRandom
            ),
            r
        );

        // read_controller_information
        let r = timeout(
            Duration::from_secs(1u64),
            mgmt.read_controller_information(),
        )
        .await
        .unwrap()
        .unwrap();
        assert_eq!(
            ReadControllerInformationResult::new(
                Address::from_str("00:11:22:33:44:55").unwrap(),
                8,
                12,
                CurrentSettings::empty(),
                CurrentSettings::empty(),
                [1; 3],
                "abc".into(),
                "def".into()
            ),
            r
        );
    }

    #[tokio::test]
    async fn test_mgmt_error() {
        use crate::pkt::mgmt::event::{CommandStatusEvent, MgmtEvent};
        use crate::pkt::mgmt::{Code, ControlIndex, Status};
        use futures::sink::SinkExt;
        use futures::stream::StreamExt;
        use tokio::net::UnixStream;
        use tokio::time::{timeout, Duration};
        use tokio_util::codec::{BytesCodec, Framed};

        let (socka, sockb) = UnixStream::pair().unwrap();

        let idx = ControlIndex::from(0);
        let mgmt = Framed::new(socka, MgmtCodec);
        let mut mgmt = Mgmt::new_internal(0, mgmt);
        let mut peer = Framed::new(sockb, BytesCodec::new());

        tokio::spawn(async move {
            // powered
            let mut buf = peer.next().await.unwrap().unwrap();
            let code = Code::unpack(&mut buf).unwrap();
            let mut buf = BytesMut::new();
            CurrentSettings::empty().pack(&mut buf).unwrap();
            let reply = CommandStatusEvent::new(code, Status::InvalidParameters);
            let mut buf = BytesMut::new();
            MgmtEvent::CommandStatusEvent(idx.clone(), reply)
                .pack(&mut buf)
                .unwrap();
            peer.send(buf.freeze()).await.unwrap();
        });

        // read_controller_information
        let r = timeout(
            Duration::from_secs(1u64),
            mgmt.read_controller_information(),
        )
        .await
        .unwrap();
        match r {
            Err(Error::CommandError(Status::InvalidParameters)) => {}
            _ => panic!(),
        }
    }

    #[tokio::test]
    async fn test_mgmt_pending() {
        use crate::pkt::mgmt::event::ConnectionFailedEvent;
        use crate::pkt::mgmt::event::{CommandCompleteEvent, MgmtEvent};
        use crate::pkt::mgmt::{Address, AddressType, Code, ControlIndex, Status};
        use futures::sink::SinkExt;
        use futures::stream::StreamExt;
        use std::str::FromStr;
        use tokio::net::UnixStream;
        use tokio::time::{timeout, Duration};
        use tokio_util::codec::{BytesCodec, Framed};

        let (socka, sockb) = UnixStream::pair().unwrap();

        let idx = ControlIndex::from(0);
        let mgmt = Framed::new(socka, MgmtCodec);
        let mut mgmt = Mgmt::new_internal(0, mgmt);
        let mut peer = Framed::new(sockb, BytesCodec::new());

        tokio::spawn(async move {
            // connection failed event
            let mut buf = BytesMut::new();
            let event = ConnectionFailedEvent::new(
                Address::from_str("00:11:22:33:44:55").unwrap(),
                AddressType::LeRandom,
                1,
            );
            MgmtEvent::ConnectionFailedEvent(idx.clone(), event)
                .pack(&mut buf)
                .unwrap();
            peer.send(buf.freeze()).await.unwrap();

            // powered
            let mut buf = peer.next().await.unwrap().unwrap();
            let code = Code::unpack(&mut buf).unwrap();
            let mut buf = BytesMut::new();
            CurrentSettings::empty().pack(&mut buf).unwrap();
            let reply = CommandCompleteEvent::new(code, Status::Success, buf.freeze());
            let mut buf = BytesMut::new();
            MgmtEvent::CommandCompleteEvent(idx.clone(), reply)
                .pack(&mut buf)
                .unwrap();
            peer.send(buf.freeze()).await.unwrap();
        });

        // read_controller_information
        let r = timeout(Duration::from_secs(1u64), mgmt.powered(true))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(CurrentSettings::empty(), r);

        // pending event
        let r = timeout(Duration::from_secs(1u64), mgmt.next())
            .await
            .unwrap()
            .unwrap()
            .unwrap();
        let idx = ControlIndex::from(0);
        let event = ConnectionFailedEvent::new(
            Address::from_str("00:11:22:33:44:55").unwrap(),
            AddressType::LeRandom,
            1,
        );
        let event = MgmtEvent::ConnectionFailedEvent(idx.clone(), event);
        assert_eq!(event, r);
    }
}
