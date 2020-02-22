use std::convert::TryFrom;

use bytes::{Buf, BufMut as _, BytesMut};

use super::ManagementCommand;
use super::{Address, AddressType};
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use super::{Codec, CodecError, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    BackgroundScanForDevice,
    AllowIncommingConnection,
    AutoConnectRemoteDevice,
}

impl Codec for Action {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        Ok(match buf.get_u8() {
            0 => Self::BackgroundScanForDevice,
            1 => Self::AllowIncommingConnection,
            2 => Self::AutoConnectRemoteDevice,
            _ => return Err(CodecError::Invalid),
        })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        let v = match self {
            Self::BackgroundScanForDevice => 0x00,
            Self::AllowIncommingConnection => 0x01,
            Self::AutoConnectRemoteDevice => 0x02,
        };
        buf.put_u8(v);
        Ok(())
    }
}

impl From<Action> for u8 {
    fn from(v: Action) -> Self {
        match v {
            Action::BackgroundScanForDevice => 0x00,
            Action::AllowIncommingConnection => 0x01,
            Action::AutoConnectRemoteDevice => 0x02,
        }
    }
}

impl TryFrom<u8> for Action {
    type Error = u8;
    fn try_from(v: u8) -> std::result::Result<Self, Self::Error> {
        Ok(match v {
            0x00 => Action::BackgroundScanForDevice,
            0x01 => Action::AllowIncommingConnection,
            0x02 => Action::AutoConnectRemoteDevice,
            v => return Err(v),
        })
    }
}
#[derive(Debug)]
pub struct AddDeviceCommand {
    ctrl_idx: u16,
    address: Address,
    address_type: AddressType,
    action: Action,
}

impl AddDeviceCommand {
    pub fn new(ctrl_idx: u16, address: Address, address_type: AddressType, action: Action) -> Self {
        Self {
            ctrl_idx,
            address,
            address_type,
            action,
        }
    }
}

impl ManagementCommand<(Address, AddressType)> for AddDeviceCommand {
    fn parse_result(buf: &mut impl Buf) -> Result<(Address, AddressType)> {
        let address = Address::parse(buf)?;
        let address_type = AddressType::parse(buf)?;
        Ok((address, address_type))
    }
}

impl CommandItem for AddDeviceCommand {
    const CODE: Code = Code(0x0033);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl Codec for AddDeviceCommand {
    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        self.address.write_to(buf)?;
        self.address_type.write_to(buf)?;
        self.action.write_to(buf)?;
        Ok(())
    }

    fn parse(_buf: &mut impl Buf) -> Result<Self> {
        unimplemented!()
    }
}

impl From<AddDeviceCommand> for MgmtCommand {
    fn from(v: AddDeviceCommand) -> Self {
        Self::AddDeviceCommand(v)
    }
}
