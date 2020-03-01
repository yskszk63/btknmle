use bytes::buf::BufExt as _;
use bytes::{Buf, BufMut, BytesMut};
use std::fmt;

use super::Action;
use super::CurrentSettings;
use super::IdentityResolvingKey;
use super::Key;
use super::LongTermKey;
use super::{command::MgmtCommand, Status};
use super::{Address, AddressType};
use super::{Code, ControlIndex};
use crate::{PackError, PacketData, UnpackError};

pub use authentication_failed_event::*;
pub use command_complete_event::*;
pub use command_status_event::*;
pub use connection_failed_event::*;
pub use device_added_event::*;
pub use device_connected_event::*;
pub use device_disconnected_event::*;
pub use device_found_event::*;
pub use device_removed_event::*;
pub use discovering_event::*;
pub use extended_controller_information_changed_event::*;
pub use new_identity_resolving_key_event::*;
pub use new_long_term_key_event::*;
pub use new_settings_event::*;
pub use new_signature_resolving_key_event::*;
pub use passkey_notify_event::*;
pub use user_confirmation_request_event::*;
pub use user_passkey_request_event::*;

mod authentication_failed_event;
mod command_complete_event;
mod command_status_event;
mod connection_failed_event;
mod device_added_event;
mod device_connected_event;
mod device_disconnected_event;
mod device_found_event;
mod device_removed_event;
mod discovering_event;
mod extended_controller_information_changed_event;
mod new_identity_resolving_key_event;
mod new_long_term_key_event;
mod new_settings_event;
mod new_signature_resolving_key_event;
mod passkey_notify_event;
mod user_confirmation_request_event;
mod user_passkey_request_event;

trait EventItem: PacketData {
    const CODE: Code;

    fn code(&self) -> Code {
        Self::CODE
    }

    fn into_mgmt(self, index: ControlIndex) -> MgmtEvent;

    fn unpack_event(buf: &mut impl Buf, index: ControlIndex) -> Result<MgmtEvent, UnpackError> {
        let val = Self::unpack(buf)?;
        Ok(val.into_mgmt(index))
    }

    fn pack_mgmt(&self, index: &ControlIndex, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.code().pack(buf)?;
        index.pack(buf)?;

        let mut b = BytesMut::new();
        self.pack(&mut b)?;

        let b = b.freeze();
        (b.len() as u16).pack(buf)?;
        if buf.remaining_mut() < b.len() {
            return Err(PackError::InsufficientBufLength);
        }
        buf.put(&mut b.as_ref());
        Ok(())
    }
}

#[derive(PartialEq, Eq)]
pub enum MgmtEvent {
    AuthenticationFailedEvent(ControlIndex, AuthenticationFailedEvent),
    CommandCompleteEvent(ControlIndex, CommandCompleteEvent),
    CommandStatusEvent(ControlIndex, CommandStatusEvent),
    DeviceConnectedEvent(ControlIndex, DeviceConnectedEvent),
    DeviceFoundEvent(ControlIndex, DeviceFoundEvent),
    DeviceDisconnectedEvent(ControlIndex, DeviceDisconnectedEvent),
    NewLongTermKeyEvent(ControlIndex, NewLongTermKeyEvent),
    NewSignatureResolvingKeyEvent(ControlIndex, NewSignatureResolvingKeyEvent),
    ExtendedControllerInformationChangedEvent(
        ControlIndex,
        ExtendedControllerInformationChangedEvent,
    ),
    UserPasskeyRequestEvent(ControlIndex, UserPasskeyRequestEvent),
    UserConfirmationRequestEvent(ControlIndex, UserConfirmationRequestEvent),
    PasskeyNotifyEvent(ControlIndex, PasskeyNotifyEvent),
    NewIdentityResolvingKeyEvent(ControlIndex, NewIdentityResolvingKeyEvent),
    NewSettingsEvent(ControlIndex, NewSettingsEvent),
    DiscoveringEvent(ControlIndex, DiscoveringEvent),
    DeviceAddedEvent(ControlIndex, DeviceAddedEvent),
    DeviceRemovedEvent(ControlIndex, DeviceRemovedEvent),
    ConnectionFailedEvent(ControlIndex, ConnectionFailedEvent),
}

impl fmt::Debug for MgmtEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MgmtEvent::AuthenticationFailedEvent(i, v) => (i, v).fmt(f),
            MgmtEvent::CommandCompleteEvent(i, v) => (i, v).fmt(f),
            MgmtEvent::CommandStatusEvent(i, v) => (i, v).fmt(f),
            MgmtEvent::DeviceConnectedEvent(i, v) => (i, v).fmt(f),
            MgmtEvent::DeviceFoundEvent(i, v) => (i, v).fmt(f),
            MgmtEvent::DeviceDisconnectedEvent(i, v) => (i, v).fmt(f),
            MgmtEvent::NewLongTermKeyEvent(i, v) => (i, v).fmt(f),
            MgmtEvent::NewSignatureResolvingKeyEvent(i, v) => (i, v).fmt(f),
            MgmtEvent::ExtendedControllerInformationChangedEvent(i, v) => (i, v).fmt(f),
            MgmtEvent::UserPasskeyRequestEvent(i, v) => (i, v).fmt(f),
            MgmtEvent::UserConfirmationRequestEvent(i, v) => (i, v).fmt(f),
            MgmtEvent::PasskeyNotifyEvent(i, v) => (i, v).fmt(f),
            MgmtEvent::NewIdentityResolvingKeyEvent(i, v) => (i, v).fmt(f),
            MgmtEvent::NewSettingsEvent(i, v) => (i, v).fmt(f),
            MgmtEvent::DiscoveringEvent(i, v) => (i, v).fmt(f),
            MgmtEvent::DeviceAddedEvent(i, v) => (i, v).fmt(f),
            MgmtEvent::DeviceRemovedEvent(i, v) => (i, v).fmt(f),
            MgmtEvent::ConnectionFailedEvent(i, v) => (i, v).fmt(f),
        }
    }
}

impl PacketData for MgmtEvent {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let code = u16::unpack(buf)?.into();
        let index = PacketData::unpack(buf)?;
        let len = u16::unpack(buf)? as usize;

        let mut buf = buf.take(len);
        match code {
            AuthenticationFailedEvent::CODE => {
                AuthenticationFailedEvent::unpack_event(&mut buf, index)
            }
            CommandCompleteEvent::CODE => CommandCompleteEvent::unpack_event(&mut buf, index),
            CommandStatusEvent::CODE => CommandStatusEvent::unpack_event(&mut buf, index),
            DeviceConnectedEvent::CODE => DeviceConnectedEvent::unpack_event(&mut buf, index),
            DeviceFoundEvent::CODE => DeviceFoundEvent::unpack_event(&mut buf, index),
            DeviceDisconnectedEvent::CODE => DeviceDisconnectedEvent::unpack_event(&mut buf, index),
            NewLongTermKeyEvent::CODE => NewLongTermKeyEvent::unpack_event(&mut buf, index),
            NewSignatureResolvingKeyEvent::CODE => {
                NewSignatureResolvingKeyEvent::unpack_event(&mut buf, index)
            }
            ExtendedControllerInformationChangedEvent::CODE => {
                ExtendedControllerInformationChangedEvent::unpack_event(&mut buf, index)
            }
            UserPasskeyRequestEvent::CODE => UserPasskeyRequestEvent::unpack_event(&mut buf, index),
            UserConfirmationRequestEvent::CODE => {
                UserConfirmationRequestEvent::unpack_event(&mut buf, index)
            }
            PasskeyNotifyEvent::CODE => PasskeyNotifyEvent::unpack_event(&mut buf, index),
            NewIdentityResolvingKeyEvent::CODE => {
                NewIdentityResolvingKeyEvent::unpack_event(&mut buf, index)
            }
            NewSettingsEvent::CODE => NewSettingsEvent::unpack_event(&mut buf, index),
            DiscoveringEvent::CODE => DiscoveringEvent::unpack_event(&mut buf, index),
            DeviceAddedEvent::CODE => DeviceAddedEvent::unpack_event(&mut buf, index),
            DeviceRemovedEvent::CODE => DeviceRemovedEvent::unpack_event(&mut buf, index),
            ConnectionFailedEvent::CODE => ConnectionFailedEvent::unpack_event(&mut buf, index),
            x => Err(UnpackError::UnknownOpcode(x.into())),
        }
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        match self {
            MgmtEvent::AuthenticationFailedEvent(i, v) => v.pack_mgmt(i, buf),
            MgmtEvent::CommandCompleteEvent(i, v) => v.pack_mgmt(i, buf),
            MgmtEvent::CommandStatusEvent(i, v) => v.pack_mgmt(i, buf),
            MgmtEvent::DeviceConnectedEvent(i, v) => v.pack_mgmt(i, buf),
            MgmtEvent::DeviceFoundEvent(i, v) => v.pack_mgmt(i, buf),
            MgmtEvent::DeviceDisconnectedEvent(i, v) => v.pack_mgmt(i, buf),
            MgmtEvent::NewLongTermKeyEvent(i, v) => v.pack_mgmt(i, buf),
            MgmtEvent::NewSignatureResolvingKeyEvent(i, v) => v.pack_mgmt(i, buf),
            MgmtEvent::ExtendedControllerInformationChangedEvent(i, v) => v.pack_mgmt(i, buf),
            MgmtEvent::UserPasskeyRequestEvent(i, v) => v.pack_mgmt(i, buf),
            MgmtEvent::UserConfirmationRequestEvent(i, v) => v.pack_mgmt(i, buf),
            MgmtEvent::PasskeyNotifyEvent(i, v) => v.pack_mgmt(i, buf),
            MgmtEvent::NewIdentityResolvingKeyEvent(i, v) => v.pack_mgmt(i, buf),
            MgmtEvent::NewSettingsEvent(i, v) => v.pack_mgmt(i, buf),
            MgmtEvent::DiscoveringEvent(i, v) => v.pack_mgmt(i, buf),
            MgmtEvent::DeviceAddedEvent(i, v) => v.pack_mgmt(i, buf),
            MgmtEvent::DeviceRemovedEvent(i, v) => v.pack_mgmt(i, buf),
            MgmtEvent::ConnectionFailedEvent(i, v) => v.pack_mgmt(i, buf),
        }
    }
}
