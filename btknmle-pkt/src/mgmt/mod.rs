use std::fmt;

use bytes::buf::BufExt as _;
use bytes::{Buf, BufMut, BytesMut};

use crate::util::HexDisplay;
use crate::{PackError, PacketData, UnpackError};

pub use add_advertising_command::*;
pub use add_device_command::*;
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
pub use discovering_event::*;
pub use extended_controller_information_changed_event::*;
pub use key::*;
pub use load_identity_resolving_keys_command::*;
pub use load_long_term_keys_command::*;
pub use name::*;
pub use new_identity_resolving_key_event::*;
pub use new_long_term_key_event::*;
pub use new_settings_event::*;
pub use new_signature_resolving_key_event::*;
pub use passkey_notify_event::*;
pub use read_controller_information_command::*;
pub use remove_advertising_command::*;
pub use remove_device_command::*;
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
mod add_device_command;
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
mod discovering_event;
mod extended_controller_information_changed_event;
mod key;
mod load_identity_resolving_keys_command;
mod load_long_term_keys_command;
mod name;
mod new_identity_resolving_key_event;
mod new_long_term_key_event;
mod new_settings_event;
mod new_signature_resolving_key_event;
mod passkey_notify_event;
mod read_controller_information_command;
mod remove_advertising_command;
mod remove_device_command;
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

impl PacketData for Code {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        u16::unpack(buf).map(Into::into)
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        u16::from(self.clone()).pack(buf)
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

impl PacketData for ControlIndex {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        u16::unpack(buf).map(Into::into)
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        u16::from(self.clone()).pack(buf)
    }
}

impl Default for ControlIndex {
    fn default() -> Self {
        ControlIndex::NonController
    }
}

pub trait ManagementCommand: Into<MgmtCommand> {
    type Result: PacketData;

    fn unpack_result(buf: &mut impl Buf) -> Result<Self::Result, UnpackError> {
        Self::Result::unpack(buf)
    }
}

trait CommandItem: PacketData + Into<MgmtCommand> {
    const CODE: Code;

    fn controller_index(&self) -> ControlIndex;

    fn code(&self) -> Code {
        Self::CODE
    }

    fn pack_mgmt(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        u16::from(self.code()).pack(buf)?;
        self.controller_index().pack(buf)?;
        let mut b = BytesMut::new();
        self.pack(&mut b)?;
        let mut b = b.freeze();
        (b.len() as u16).pack(buf)?;
        buf.put(&mut b);
        Ok(())
    }
}

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

pub enum MgmtCommand {
    SetPoweredCommand(SetPoweredCommand),
    SetConnectableCommand(SetConnectableCommand),
    SetBondableCommand(SetBondableCommand),
    SetLowEnergyCommand(SetLowEnergyCommand),
    SetLocalNameCommand(Box<SetLocalNameCommand>),
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
    AddDeviceCommand(AddDeviceCommand),
    RemoveDeviceCommand(RemoveDeviceCommand),
    ReadControllerInformationCommand(ReadControllerInformationCommand),
    RemoveAdvertisingCommand(RemoveAdvertisingCommand),
}

impl MgmtCommand {
    fn debug<B, T>(parameters: &mut B) -> String
    where
        B: Buf + Clone,
        T: ManagementCommand,
        <T as ManagementCommand>::Result: fmt::Debug,
    {
        let mut parameters = parameters.clone();
        match T::unpack_result(&mut parameters) {
            Ok(result) => format!("{:?}", result),
            Err(e) => format!(
                "{:?} (parse failure {})",
                HexDisplay::new(parameters.to_bytes()),
                e
            ),
        }
    }

    fn debug_result<B>(code: &Code, parameters: &mut B) -> String
    where
        B: Buf + Clone,
    {
        match code.clone() {
            // ?
            SetPoweredCommand::CODE => Self::debug::<B, SetPoweredCommand>(parameters),
            SetConnectableCommand::CODE => Self::debug::<B, SetConnectableCommand>(parameters),
            SetBondableCommand::CODE => Self::debug::<B, SetBondableCommand>(parameters),
            SetLowEnergyCommand::CODE => Self::debug::<B, SetLowEnergyCommand>(parameters),
            SetLocalNameCommand::CODE => Self::debug::<B, SetLocalNameCommand>(parameters),
            SetAdvertisingCommand::CODE => Self::debug::<B, SetAdvertisingCommand>(parameters),
            SetBrEdrCommand::CODE => Self::debug::<B, SetBrEdrCommand>(parameters),
            SetDiscoverableCommand::CODE => Self::debug::<B, SetDiscoverableCommand>(parameters),
            UserConfirmationReplyCommand::CODE => {
                Self::debug::<B, UserConfirmationReplyCommand>(parameters)
            }
            UserConfirmationNegativeReplyCommand::CODE => {
                Self::debug::<B, UserConfirmationNegativeReplyCommand>(parameters)
            }
            SetSecureConnectionsCommand::CODE => {
                Self::debug::<B, SetSecureConnectionsCommand>(parameters)
            }
            SetPrivacyCommand::CODE => Self::debug::<B, SetPrivacyCommand>(parameters),
            SetIoCapabilityCommand::CODE => Self::debug::<B, SetIoCapabilityCommand>(parameters),
            LoadIdentityResolvingKeysCommand::CODE => {
                Self::debug::<B, LoadIdentityResolvingKeysCommand>(parameters)
            }
            LoadLongTermKeysCommand::CODE => Self::debug::<B, LoadLongTermKeysCommand>(parameters),
            SetAppearanceCommand::CODE => Self::debug::<B, SetAppearanceCommand>(parameters),
            AddAdvertisingCommand::CODE => Self::debug::<B, AddAdvertisingCommand>(parameters),
            UserPasskeyReplyCommand::CODE => Self::debug::<B, UserPasskeyReplyCommand>(parameters),
            AddDeviceCommand::CODE => Self::debug::<B, AddDeviceCommand>(parameters),
            RemoveDeviceCommand::CODE => Self::debug::<B, RemoveDeviceCommand>(parameters),
            ReadControllerInformationCommand::CODE => {
                Self::debug::<B, ReadControllerInformationCommand>(parameters)
            }
            RemoveAdvertisingCommand::CODE => {
                Self::debug::<B, RemoveAdvertisingCommand>(parameters)
            }
            _ => format!("{:?}", HexDisplay::new(parameters.to_bytes())),
        }
    }
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
            MgmtCommand::AddDeviceCommand(v) => v.fmt(f),
            MgmtCommand::RemoveDeviceCommand(v) => v.fmt(f),
            MgmtCommand::ReadControllerInformationCommand(v) => v.fmt(f),
            MgmtCommand::RemoveAdvertisingCommand(v) => v.fmt(f),
        }
    }
}

impl PacketData for MgmtCommand {
    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        match self {
            MgmtCommand::SetPoweredCommand(v) => v.pack_mgmt(buf),
            MgmtCommand::SetConnectableCommand(v) => v.pack_mgmt(buf),
            MgmtCommand::SetBondableCommand(v) => v.pack_mgmt(buf),
            MgmtCommand::SetLowEnergyCommand(v) => v.pack_mgmt(buf),
            MgmtCommand::SetLocalNameCommand(v) => v.pack_mgmt(buf),
            MgmtCommand::SetAdvertisingCommand(v) => v.pack_mgmt(buf),
            MgmtCommand::SetBrEdrCommand(v) => v.pack_mgmt(buf),
            MgmtCommand::SetDiscoverableCommand(v) => v.pack_mgmt(buf),
            MgmtCommand::UserConfirmationReplyCommand(v) => v.pack_mgmt(buf),
            MgmtCommand::UserConfirmationNegativeReplyCommand(v) => v.pack_mgmt(buf),
            MgmtCommand::SetSecureConnectionsCommand(v) => v.pack_mgmt(buf),
            MgmtCommand::SetPrivacyCommand(v) => v.pack_mgmt(buf),
            MgmtCommand::SetIoCapabilityCommand(v) => v.pack_mgmt(buf),
            MgmtCommand::LoadIdentityResolvingKeysCommand(v) => v.pack_mgmt(buf),
            MgmtCommand::LoadLongTermKeysCommand(v) => v.pack_mgmt(buf),
            MgmtCommand::SetAppearanceCommand(v) => v.pack_mgmt(buf),
            MgmtCommand::AddAdvertisingCommand(v) => v.pack_mgmt(buf),
            MgmtCommand::UserPasskeyReplyCommand(v) => v.pack_mgmt(buf),
            MgmtCommand::AddDeviceCommand(v) => v.pack_mgmt(buf),
            MgmtCommand::RemoveDeviceCommand(v) => v.pack_mgmt(buf),
            MgmtCommand::ReadControllerInformationCommand(v) => v.pack_mgmt(buf),
            MgmtCommand::RemoveAdvertisingCommand(v) => v.pack_mgmt(buf),
        }
    }

    fn unpack(_buf: &mut impl Buf) -> Result<Self, UnpackError> {
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
            x => Err(UnpackError::unexpected(format!(
                "unknown event code {:?}",
                x
            ))),
        }
    }

    fn pack(&self, _buf: &mut impl BufMut) -> Result<(), PackError> {
        unimplemented!()
    }
}
