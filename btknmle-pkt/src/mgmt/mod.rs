use std::fmt;

use bytes::buf::BufExt as _;
use bytes::{Buf, BufMut as _, BytesMut};

use super::{Codec, CodecError, Result};

pub use add_advertising_command::*;
pub use address::*;
pub use address_type::*;
pub use advertising_flags::*;
pub use authentication_failed_event::*;
pub use command_complete_event::*;
pub use command_status_event::*;
pub use current_settings::*;
pub use device_connected_event::*;
pub use device_disconnected_event::*;
pub use device_found_event::*;
pub use extended_controller_information_changed_event::*;
pub use key::*;
pub use load_identity_resolving_keys_command::*;
pub use load_long_term_keys_command::*;
pub use new_identity_resolving_key_event::*;
pub use new_long_term_key_event::*;
pub use new_signature_resolving_key_event::*;
pub use passkey_notify_event::*;
pub use set_advertising_command::*;
pub use set_appearance_command::*;
pub use set_bondable_command::*;
pub use set_br_edr_command::*;
pub use set_connectable_command::*;
pub use set_discoverable_command::*;
pub use set_io_capability_command::*;
pub use set_local_name_command::*;
pub use set_low_energy_command::*;
pub use set_powered_command::*;
pub use set_privacy_command::*;
pub use set_secure_connections_command::*;
pub use status::*;
pub use user_confirmation_negative_reply_command::*;
pub use user_confirmation_reply_command::*;
pub use user_confirmation_request_event::*;
pub use user_passkey_reply_command::*;
pub use user_passkey_request_event::*;

mod add_advertising_command;
mod address;
mod address_type;
mod advertising_flags;
mod authentication_failed_event;
mod command_complete_event;
mod command_status_event;
mod current_settings;
mod device_connected_event;
mod device_disconnected_event;
mod device_found_event;
mod extended_controller_information_changed_event;
mod key;
mod load_identity_resolving_keys_command;
mod load_long_term_keys_command;
mod new_identity_resolving_key_event;
mod new_long_term_key_event;
mod new_signature_resolving_key_event;
mod passkey_notify_event;
mod set_advertising_command;
mod set_appearance_command;
mod set_bondable_command;
mod set_br_edr_command;
mod set_connectable_command;
mod set_discoverable_command;
mod set_io_capability_command;
mod set_local_name_command;
mod set_low_energy_command;
mod set_powered_command;
mod set_privacy_command;
mod set_secure_connections_command;
mod status;
mod user_confirmation_negative_reply_command;
mod user_confirmation_reply_command;
mod user_confirmation_request_event;
mod user_passkey_reply_command;
mod user_passkey_request_event;

#[derive(Clone, PartialEq, Eq)]
pub struct Code(u16);

impl fmt::Debug for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:04X}", self.0)
    }
}

impl From<u16> for Code {
    fn from(v: u16) -> Self {
        Self(v)
    }
}

impl From<Code> for u16 {
    fn from(v: Code) -> Self {
        v.0
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum ControlIndex {
    ControllerId(u16),
    NonController,
}

impl fmt::Debug for ControlIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ControllerId(v) => v.fmt(f),
            Self::NonController => "N/A".fmt(f),
        }
    }
}

impl From<u16> for ControlIndex {
    fn from(v: u16) -> Self {
        if v == 0xFFFF {
            Self::NonController
        } else {
            Self::ControllerId(v)
        }
    }
}

impl From<ControlIndex> for u16 {
    fn from(v: ControlIndex) -> Self {
        match v {
            ControlIndex::ControllerId(v) => v,
            ControlIndex::NonController => 0xFFFF,
        }
    }
}

impl Default for ControlIndex {
    fn default() -> Self {
        ControlIndex::NonController
    }
}

pub trait ManagementCommand<T>: Into<MgmtCommand> {
    fn parse_result(buf: &mut impl Buf) -> Result<T>;
}

trait CommandItem: Codec + Into<MgmtCommand> {
    const CODE: Code;

    fn controller_index(&self) -> ControlIndex;

    fn code(&self) -> Code {
        Self::CODE
    }
}

trait EventItem: Codec + Into<MgmtEvent> {
    const CODE: Code;

    fn with_controller_index(self, idx: ControlIndex) -> Self;

    fn code(&self) -> Code {
        Self::CODE
    }
}

pub enum MgmtCommand {
    SetPoweredCommand(SetPoweredCommand),
    SetConnectableCommand(SetConnectableCommand),
    SetBondableCommand(SetBondableCommand),
    SetLowEnergyCommand(SetLowEnergyCommand),
    SetLocalNameCommand(SetLocalNameCommand),
    SetAdvertisingCommand(SetAdvertisingCommand),
    SetBrEdrCommand(SetBrEdrCommand),
    SetDiscoverableCommand(SetDiscoverableCommand),
    UserConfirmationReplyCommand(UserConfirmationReplyCommand),
    UserConfirmationNegativeReplyCommand(UserConfirmationNegativeReplyCommand),
    SetSecureConnectionsCommand(SetSecureConnectionsCommand),
    SetPrivacyCommand(SetPrivacyCommand),
    SetIoCapabilityCommand(SetIoCapabilityCommand),
    LoadIdentityResolvingKeysCommand(LoadIdentityResolvingKeysCommand),
    LoadLongTermKeysCommand(LoadLongTermKeysCommand),
    SetAppearanceCommand(SetAppearanceCommand),
    AddAdvertisingCommand(AddAdvertisingCommand),
    UserPasskeyReplyCommand(UserPasskeyReplyCommand),
}

impl fmt::Debug for MgmtCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MgmtCommand::SetPoweredCommand(v) => v.fmt(f),
            MgmtCommand::SetConnectableCommand(v) => v.fmt(f),
            MgmtCommand::SetBondableCommand(v) => v.fmt(f),
            MgmtCommand::SetLowEnergyCommand(v) => v.fmt(f),
            MgmtCommand::SetLocalNameCommand(v) => v.fmt(f),
            MgmtCommand::SetAdvertisingCommand(v) => v.fmt(f),
            MgmtCommand::SetBrEdrCommand(v) => v.fmt(f),
            MgmtCommand::SetDiscoverableCommand(v) => v.fmt(f),
            MgmtCommand::UserConfirmationReplyCommand(v) => v.fmt(f),
            MgmtCommand::UserConfirmationNegativeReplyCommand(v) => v.fmt(f),
            MgmtCommand::SetSecureConnectionsCommand(v) => v.fmt(f),
            MgmtCommand::SetPrivacyCommand(v) => v.fmt(f),
            MgmtCommand::SetIoCapabilityCommand(v) => v.fmt(f),
            MgmtCommand::LoadIdentityResolvingKeysCommand(v) => v.fmt(f),
            MgmtCommand::LoadLongTermKeysCommand(v) => v.fmt(f),
            MgmtCommand::SetAppearanceCommand(v) => v.fmt(f),
            MgmtCommand::AddAdvertisingCommand(v) => v.fmt(f),
            MgmtCommand::UserPasskeyReplyCommand(v) => v.fmt(f),
        }
    }
}

impl Codec for MgmtCommand {
    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        let code = match self {
            MgmtCommand::SetPoweredCommand(v) => v.code(),
            MgmtCommand::SetConnectableCommand(v) => v.code(),
            MgmtCommand::SetBondableCommand(v) => v.code(),
            MgmtCommand::SetLowEnergyCommand(v) => v.code(),
            MgmtCommand::SetLocalNameCommand(v) => v.code(),
            MgmtCommand::SetAdvertisingCommand(v) => v.code(),
            MgmtCommand::SetBrEdrCommand(v) => v.code(),
            MgmtCommand::SetDiscoverableCommand(v) => v.code(),
            MgmtCommand::UserConfirmationReplyCommand(v) => v.code(),
            MgmtCommand::UserConfirmationNegativeReplyCommand(v) => v.code(),
            MgmtCommand::SetSecureConnectionsCommand(v) => v.code(),
            MgmtCommand::SetPrivacyCommand(v) => v.code(),
            MgmtCommand::SetIoCapabilityCommand(v) => v.code(),
            MgmtCommand::LoadIdentityResolvingKeysCommand(v) => v.code(),
            MgmtCommand::LoadLongTermKeysCommand(v) => v.code(),
            MgmtCommand::SetAppearanceCommand(v) => v.code(),
            MgmtCommand::AddAdvertisingCommand(v) => v.code(),
            MgmtCommand::UserPasskeyReplyCommand(v) => v.code(),
        };
        buf.put_u16_le(code.into());

        let mut b = BytesMut::new();
        match self {
            MgmtCommand::SetPoweredCommand(v) => v.write_to(&mut b)?,
            MgmtCommand::SetConnectableCommand(v) => v.write_to(&mut b)?,
            MgmtCommand::SetBondableCommand(v) => v.write_to(&mut b)?,
            MgmtCommand::SetLowEnergyCommand(v) => v.write_to(&mut b)?,
            MgmtCommand::SetLocalNameCommand(v) => v.write_to(&mut b)?,
            MgmtCommand::SetAdvertisingCommand(v) => v.write_to(&mut b)?,
            MgmtCommand::SetBrEdrCommand(v) => v.write_to(&mut b)?,
            MgmtCommand::SetDiscoverableCommand(v) => v.write_to(&mut b)?,
            MgmtCommand::UserConfirmationReplyCommand(v) => v.write_to(&mut b)?,
            MgmtCommand::UserConfirmationNegativeReplyCommand(v) => v.write_to(&mut b)?,
            MgmtCommand::SetSecureConnectionsCommand(v) => v.write_to(&mut b)?,
            MgmtCommand::SetPrivacyCommand(v) => v.write_to(&mut b)?,
            MgmtCommand::SetIoCapabilityCommand(v) => v.write_to(&mut b)?,
            MgmtCommand::LoadIdentityResolvingKeysCommand(v) => v.write_to(&mut b)?,
            MgmtCommand::LoadLongTermKeysCommand(v) => v.write_to(&mut b)?,
            MgmtCommand::SetAppearanceCommand(v) => v.write_to(&mut b)?,
            MgmtCommand::AddAdvertisingCommand(v) => v.write_to(&mut b)?,
            MgmtCommand::UserPasskeyReplyCommand(v) => v.write_to(&mut b)?,
        };
        let b = b.freeze();

        buf.put_u16_le(
            match self {
                MgmtCommand::SetPoweredCommand(v) => v.controller_index(),
                MgmtCommand::SetConnectableCommand(v) => v.controller_index(),
                MgmtCommand::SetBondableCommand(v) => v.controller_index(),
                MgmtCommand::SetLowEnergyCommand(v) => v.controller_index(),
                MgmtCommand::SetLocalNameCommand(v) => v.controller_index(),
                MgmtCommand::SetAdvertisingCommand(v) => v.controller_index(),
                MgmtCommand::SetBrEdrCommand(v) => v.controller_index(),
                MgmtCommand::SetDiscoverableCommand(v) => v.controller_index(),
                MgmtCommand::UserConfirmationReplyCommand(v) => v.controller_index(),
                MgmtCommand::UserConfirmationNegativeReplyCommand(v) => v.controller_index(),
                MgmtCommand::SetSecureConnectionsCommand(v) => v.controller_index(),
                MgmtCommand::SetPrivacyCommand(v) => v.controller_index(),
                MgmtCommand::SetIoCapabilityCommand(v) => v.controller_index(),
                MgmtCommand::LoadIdentityResolvingKeysCommand(v) => v.controller_index(),
                MgmtCommand::LoadLongTermKeysCommand(v) => v.controller_index(),
                MgmtCommand::SetAppearanceCommand(v) => v.controller_index(),
                MgmtCommand::AddAdvertisingCommand(v) => v.controller_index(),
                MgmtCommand::UserPasskeyReplyCommand(v) => v.controller_index(),
            }
            .into(),
        );

        buf.put_u16_le(b.len() as u16);
        buf.put(b);
        Ok(())
    }

    fn parse(_buf: &mut impl Buf) -> Result<Self> {
        unimplemented!()
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
        }
    }
}

impl Codec for MgmtEvent {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let code = buf.get_u16_le().into();
        let controller_index = buf.get_u16_le().into();
        let len = buf.get_u16_le() as usize;

        let mut data = buf.take(len);
        Ok(match code {
            AuthenticationFailedEvent::CODE => AuthenticationFailedEvent::parse(&mut data)?
                .with_controller_index(controller_index)
                .into(),
            CommandCompleteEvent::CODE => CommandCompleteEvent::parse(&mut data)?
                .with_controller_index(controller_index)
                .into(),
            CommandStatusEvent::CODE => CommandStatusEvent::parse(&mut data)?
                .with_controller_index(controller_index)
                .into(),
            DeviceConnectedEvent::CODE => DeviceConnectedEvent::parse(&mut data)?
                .with_controller_index(controller_index)
                .into(),
            DeviceFoundEvent::CODE => DeviceFoundEvent::parse(&mut data)?
                .with_controller_index(controller_index)
                .into(),
            DeviceDisconnectedEvent::CODE => DeviceDisconnectedEvent::parse(&mut data)?
                .with_controller_index(controller_index)
                .into(),
            NewLongTermKeyEvent::CODE => NewLongTermKeyEvent::parse(&mut data)?
                .with_controller_index(controller_index)
                .into(),
            NewSignatureResolvingKeyEvent::CODE => NewSignatureResolvingKeyEvent::parse(&mut data)?
                .with_controller_index(controller_index)
                .into(),
            ExtendedControllerInformationChangedEvent::CODE => {
                ExtendedControllerInformationChangedEvent::parse(&mut data)?
                    .with_controller_index(controller_index)
                    .into()
            }
            UserPasskeyRequestEvent::CODE => UserPasskeyRequestEvent::parse(&mut data)?
                .with_controller_index(controller_index)
                .into(),
            UserConfirmationRequestEvent::CODE => UserConfirmationRequestEvent::parse(&mut data)?
                .with_controller_index(controller_index)
                .into(),
            PasskeyNotifyEvent::CODE => PasskeyNotifyEvent::parse(&mut data)?
                .with_controller_index(controller_index)
                .into(),
            NewIdentityResolvingKeyEvent::CODE => NewIdentityResolvingKeyEvent::parse(&mut data)?
                .with_controller_index(controller_index)
                .into(),
            x => return Err(CodecError::UnknownMgmt(x.into())),
        })
    }

    fn write_to(&self, _buf: &mut BytesMut) -> Result<()> {
        unimplemented!()
    }
}
