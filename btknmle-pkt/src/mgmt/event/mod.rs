use std::fmt;
use bytes::{Buf, BufMut};
use bytes::buf::BufExt as _;

use crate::{PackError, PacketData, UnpackError};
use super::{Code, ControlIndex};
use super::{Address, AddressType};
use super::{command::MgmtCommand, Status};
use super::IdentityResolvingKey;
use super::LongTermKey;
use super::CurrentSettings;
use super::Key;

pub use authentication_failed_event::*;
pub use command_complete_event::*;
pub use command_status_event::*;
pub use device_connected_event::*;
pub use device_disconnected_event::*;
pub use device_found_event::*;
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
mod device_connected_event;
mod device_disconnected_event;
mod device_found_event;
mod discovering_event;
mod extended_controller_information_changed_event;
mod new_identity_resolving_key_event;
mod new_long_term_key_event;
mod new_settings_event;
mod new_signature_resolving_key_event;
mod passkey_notify_event;
mod user_confirmation_request_event;
mod user_passkey_request_event;

trait EventItem: PacketData + Into<MgmtEvent> {
    const CODE: Code;

    fn with_controller_index(self, idx: ControlIndex) -> Self;

    fn code(&self) -> Code {
        Self::CODE
    }

    fn unpack_event(buf: &mut impl Buf, index: ControlIndex) -> Result<MgmtEvent, UnpackError> {
        Ok(Self::unpack(buf)?.with_controller_index(index).into())
    }
}

pub enum MgmtEvent {
    AuthenticationFailedEvent(AuthenticationFailedEvent),
    CommandCompleteEvent(CommandCompleteEvent),
    CommandStatusEvent(CommandStatusEvent),
    DeviceConnectedEvent(DeviceConnectedEvent),
    DeviceFoundEvent(DeviceFoundEvent),
    DeviceDisconnectedEvent(DeviceDisconnectedEvent),
    NewLongTermKeyEvent(NewLongTermKeyEvent),
    NewSignatureResolvingKeyEvent(NewSignatureResolvingKeyEvent),
    ExtendedControllerInformationChangedEvent(ExtendedControllerInformationChangedEvent),
    UserPasskeyRequestEvent(UserPasskeyRequestEvent),
    UserConfirmationRequestEvent(UserConfirmationRequestEvent),
    PasskeyNotifyEvent(PasskeyNotifyEvent),
    NewIdentityResolvingKeyEvent(NewIdentityResolvingKeyEvent),
    NewSettingsEvent(NewSettingsEvent),
    DiscoveringEvent(DiscoveringEvent),
}

impl fmt::Debug for MgmtEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MgmtEvent::AuthenticationFailedEvent(v) => v.fmt(f),
            MgmtEvent::CommandCompleteEvent(v) => v.fmt(f),
            MgmtEvent::CommandStatusEvent(v) => v.fmt(f),
            MgmtEvent::DeviceConnectedEvent(v) => v.fmt(f),
            MgmtEvent::DeviceFoundEvent(v) => v.fmt(f),
            MgmtEvent::DeviceDisconnectedEvent(v) => v.fmt(f),
            MgmtEvent::NewLongTermKeyEvent(v) => v.fmt(f),
            MgmtEvent::NewSignatureResolvingKeyEvent(v) => v.fmt(f),
            MgmtEvent::ExtendedControllerInformationChangedEvent(v) => v.fmt(f),
            MgmtEvent::UserPasskeyRequestEvent(v) => v.fmt(f),
            MgmtEvent::UserConfirmationRequestEvent(v) => v.fmt(f),
            MgmtEvent::PasskeyNotifyEvent(v) => v.fmt(f),
            MgmtEvent::NewIdentityResolvingKeyEvent(v) => v.fmt(f),
            MgmtEvent::NewSettingsEvent(v) => v.fmt(f),
            MgmtEvent::DiscoveringEvent(v) => v.fmt(f),
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
            x => Err(UnpackError::UnknownOpcode(x.into())),
        }
    }

    fn pack(&self, _buf: &mut impl BufMut) -> Result<(), PackError> {
        unimplemented!()
    }
}
