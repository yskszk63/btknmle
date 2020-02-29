use bytes::buf::BufExt as _;
use bytes::{Buf, BufMut, BytesMut};
use std::fmt;

use super::AdvertisingFlags;
use super::CurrentSettings;
use super::IdentityResolvingKey;
use super::LongTermKey;
use super::{Address, AddressType};
use super::{Code, ControlIndex};
use super::{CompleteName, Name, ShortName};
use crate::util::HexDisplay;
use crate::{PackError, PacketData, UnpackError};

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

pub trait ManagementCommand {
    type Result: PacketData;

    fn unpack_result(buf: &mut impl Buf) -> Result<Self::Result, UnpackError> {
        Self::Result::unpack(buf)
    }

    fn into_mgmt(self, index: ControlIndex) -> MgmtCommand;
}

trait CommandItem: PacketData {
    const CODE: Code;

    fn code(&self) -> Code {
        Self::CODE
    }

    fn pack_mgmt(&self, index: &ControlIndex, buf: &mut impl BufMut) -> Result<(), PackError> {
        u16::from(self.code()).pack(buf)?;
        index.pack(buf)?;
        let mut b = BytesMut::new();
        self.pack(&mut b)?;
        let mut b = b.freeze();
        (b.len() as u16).pack(buf)?;
        buf.put(&mut b);
        Ok(())
    }
}

#[derive(PartialEq, Eq)]
pub enum MgmtCommand {
    SetPoweredCommand(ControlIndex, SetPoweredCommand),
    SetConnectableCommand(ControlIndex, SetConnectableCommand),
    SetBondableCommand(ControlIndex, SetBondableCommand),
    SetLowEnergyCommand(ControlIndex, SetLowEnergyCommand),
    SetLocalNameCommand(ControlIndex, Box<SetLocalNameCommand>),
    SetAdvertisingCommand(ControlIndex, SetAdvertisingCommand),
    SetBrEdrCommand(ControlIndex, SetBrEdrCommand),
    SetDiscoverableCommand(ControlIndex, SetDiscoverableCommand),
    UserConfirmationReplyCommand(ControlIndex, UserConfirmationReplyCommand),
    UserConfirmationNegativeReplyCommand(ControlIndex, UserConfirmationNegativeReplyCommand),
    SetSecureConnectionsCommand(ControlIndex, SetSecureConnectionsCommand),
    SetPrivacyCommand(ControlIndex, SetPrivacyCommand),
    SetIoCapabilityCommand(ControlIndex, SetIoCapabilityCommand),
    LoadIdentityResolvingKeysCommand(ControlIndex, LoadIdentityResolvingKeysCommand),
    LoadLongTermKeysCommand(ControlIndex, LoadLongTermKeysCommand),
    SetAppearanceCommand(ControlIndex, SetAppearanceCommand),
    AddAdvertisingCommand(ControlIndex, AddAdvertisingCommand),
    UserPasskeyReplyCommand(ControlIndex, UserPasskeyReplyCommand),
    AddDeviceCommand(ControlIndex, AddDeviceCommand),
    RemoveDeviceCommand(ControlIndex, RemoveDeviceCommand),
    ReadControllerInformationCommand(ControlIndex, ReadControllerInformationCommand),
    RemoveAdvertisingCommand(ControlIndex, RemoveAdvertisingCommand),
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
            MgmtCommand::SetPoweredCommand(i, v) => (i, v).fmt(f),
            MgmtCommand::SetConnectableCommand(i, v) => (i, v).fmt(f),
            MgmtCommand::SetBondableCommand(i, v) => (i, v).fmt(f),
            MgmtCommand::SetLowEnergyCommand(i, v) => (i, v).fmt(f),
            MgmtCommand::SetLocalNameCommand(i, v) => (i, v).fmt(f),
            MgmtCommand::SetAdvertisingCommand(i, v) => (i, v).fmt(f),
            MgmtCommand::SetBrEdrCommand(i, v) => (i, v).fmt(f),
            MgmtCommand::SetDiscoverableCommand(i, v) => (i, v).fmt(f),
            MgmtCommand::UserConfirmationReplyCommand(i, v) => (i, v).fmt(f),
            MgmtCommand::UserConfirmationNegativeReplyCommand(i, v) => (i, v).fmt(f),
            MgmtCommand::SetSecureConnectionsCommand(i, v) => (i, v).fmt(f),
            MgmtCommand::SetPrivacyCommand(i, v) => (i, v).fmt(f),
            MgmtCommand::SetIoCapabilityCommand(i, v) => (i, v).fmt(f),
            MgmtCommand::LoadIdentityResolvingKeysCommand(i, v) => (i, v).fmt(f),
            MgmtCommand::LoadLongTermKeysCommand(i, v) => (i, v).fmt(f),
            MgmtCommand::SetAppearanceCommand(i, v) => (i, v).fmt(f),
            MgmtCommand::AddAdvertisingCommand(i, v) => (i, v).fmt(f),
            MgmtCommand::UserPasskeyReplyCommand(i, v) => (i, v).fmt(f),
            MgmtCommand::AddDeviceCommand(i, v) => (i, v).fmt(f),
            MgmtCommand::RemoveDeviceCommand(i, v) => (i, v).fmt(f),
            MgmtCommand::ReadControllerInformationCommand(i, v) => (i, v).fmt(f),
            MgmtCommand::RemoveAdvertisingCommand(i, v) => (i, v).fmt(f),
        }
    }
}

impl PacketData for MgmtCommand {
    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        match self {
            MgmtCommand::SetPoweredCommand(i, v) => v.pack_mgmt(i, buf),
            MgmtCommand::SetConnectableCommand(i, v) => v.pack_mgmt(i, buf),
            MgmtCommand::SetBondableCommand(i, v) => v.pack_mgmt(i, buf),
            MgmtCommand::SetLowEnergyCommand(i, v) => v.pack_mgmt(i, buf),
            MgmtCommand::SetLocalNameCommand(i, v) => v.pack_mgmt(i, buf),
            MgmtCommand::SetAdvertisingCommand(i, v) => v.pack_mgmt(i, buf),
            MgmtCommand::SetBrEdrCommand(i, v) => v.pack_mgmt(i, buf),
            MgmtCommand::SetDiscoverableCommand(i, v) => v.pack_mgmt(i, buf),
            MgmtCommand::UserConfirmationReplyCommand(i, v) => v.pack_mgmt(i, buf),
            MgmtCommand::UserConfirmationNegativeReplyCommand(i, v) => v.pack_mgmt(i, buf),
            MgmtCommand::SetSecureConnectionsCommand(i, v) => v.pack_mgmt(i, buf),
            MgmtCommand::SetPrivacyCommand(i, v) => v.pack_mgmt(i, buf),
            MgmtCommand::SetIoCapabilityCommand(i, v) => v.pack_mgmt(i, buf),
            MgmtCommand::LoadIdentityResolvingKeysCommand(i, v) => v.pack_mgmt(i, buf),
            MgmtCommand::LoadLongTermKeysCommand(i, v) => v.pack_mgmt(i, buf),
            MgmtCommand::SetAppearanceCommand(i, v) => v.pack_mgmt(i, buf),
            MgmtCommand::AddAdvertisingCommand(i, v) => v.pack_mgmt(i, buf),
            MgmtCommand::UserPasskeyReplyCommand(i, v) => v.pack_mgmt(i, buf),
            MgmtCommand::AddDeviceCommand(i, v) => v.pack_mgmt(i, buf),
            MgmtCommand::RemoveDeviceCommand(i, v) => v.pack_mgmt(i, buf),
            MgmtCommand::ReadControllerInformationCommand(i, v) => v.pack_mgmt(i, buf),
            MgmtCommand::RemoveAdvertisingCommand(i, v) => v.pack_mgmt(i, buf),
        }
    }

    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let code = Code::unpack(buf)?;
        let index = ControlIndex::unpack(buf)?;
        let len = u16::unpack(buf)? as usize;
        let mut b = buf.take(len);
        Ok(match code {
            SetPoweredCommand::CODE => SetPoweredCommand::unpack(&mut b)?.into_mgmt(index),
            SetConnectableCommand::CODE => SetConnectableCommand::unpack(&mut b)?.into_mgmt(index),
            SetBondableCommand::CODE => SetBondableCommand::unpack(&mut b)?.into_mgmt(index),
            SetLowEnergyCommand::CODE => SetLowEnergyCommand::unpack(&mut b)?.into_mgmt(index),
            SetLocalNameCommand::CODE => SetLocalNameCommand::unpack(&mut b)?.into_mgmt(index),
            SetAdvertisingCommand::CODE => SetAdvertisingCommand::unpack(&mut b)?.into_mgmt(index),
            SetBrEdrCommand::CODE => SetBrEdrCommand::unpack(&mut b)?.into_mgmt(index),
            SetDiscoverableCommand::CODE => {
                SetDiscoverableCommand::unpack(&mut b)?.into_mgmt(index)
            }
            UserConfirmationReplyCommand::CODE => {
                UserConfirmationReplyCommand::unpack(&mut b)?.into_mgmt(index)
            }
            UserConfirmationNegativeReplyCommand::CODE => {
                UserConfirmationNegativeReplyCommand::unpack(&mut b)?.into_mgmt(index)
            }
            SetSecureConnectionsCommand::CODE => {
                SetSecureConnectionsCommand::unpack(&mut b)?.into_mgmt(index)
            }
            SetPrivacyCommand::CODE => SetPrivacyCommand::unpack(&mut b)?.into_mgmt(index),
            SetIoCapabilityCommand::CODE => {
                SetIoCapabilityCommand::unpack(&mut b)?.into_mgmt(index)
            }
            LoadIdentityResolvingKeysCommand::CODE => {
                LoadIdentityResolvingKeysCommand::unpack(&mut b)?.into_mgmt(index)
            }
            LoadLongTermKeysCommand::CODE => {
                LoadLongTermKeysCommand::unpack(&mut b)?.into_mgmt(index)
            }
            SetAppearanceCommand::CODE => SetAppearanceCommand::unpack(&mut b)?.into_mgmt(index),
            AddAdvertisingCommand::CODE => AddAdvertisingCommand::unpack(&mut b)?.into_mgmt(index),
            UserPasskeyReplyCommand::CODE => {
                UserPasskeyReplyCommand::unpack(&mut b)?.into_mgmt(index)
            }
            AddDeviceCommand::CODE => AddDeviceCommand::unpack(&mut b)?.into_mgmt(index),
            RemoveDeviceCommand::CODE => RemoveDeviceCommand::unpack(&mut b)?.into_mgmt(index),
            ReadControllerInformationCommand::CODE => {
                ReadControllerInformationCommand::unpack(&mut b)?.into_mgmt(index)
            }
            RemoveAdvertisingCommand::CODE => {
                RemoveAdvertisingCommand::unpack(&mut b)?.into_mgmt(index)
            }
            x => return Err(UnpackError::UnknownOpcode(x.into())),
        })
    }
}
