use std::fmt;
use bytes::{Buf, BufMut, BytesMut};

use crate::{PackError, PacketData, UnpackError};
use crate::util::HexDisplay;
use super::{Code, ControlIndex};
use super::AdvertisingFlags;
use super::{Address, AddressType};
use super::IdentityResolvingKey;
use super::LongTermKey;
use super::CurrentSettings;
use super::{CompleteName, Name, ShortName};

pub use add_advertising_command::*;
pub use add_device_command::*;
pub use load_identity_resolving_keys_command::*;
pub use load_long_term_keys_command::*;
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
pub use user_confirmation_negative_reply_command::*;
pub use user_confirmation_reply_command::*;
pub use user_passkey_reply_command::*;

mod add_advertising_command;
mod add_device_command;
mod load_identity_resolving_keys_command;
mod load_long_term_keys_command;
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
mod user_confirmation_negative_reply_command;
mod user_confirmation_reply_command;
mod user_passkey_reply_command;

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

    pub(crate) fn debug_result<B>(code: &Code, parameters: &mut B) -> String
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
