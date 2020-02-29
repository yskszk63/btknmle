use std::convert::{TryFrom, TryInto};

use bytes::{Buf, BufMut};

use super::ManagementCommand;
use super::{Address, AddressType};
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    BackgroundScanForDevice,
    AllowIncommingConnection,
    AutoConnectRemoteDevice,
}

impl PacketData for Action {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let b = u8::unpack(buf)?;
        b.try_into().map_err(UnpackError::UnexpectedValue)
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        buf.put_u8(self.clone().into());
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

#[derive(Debug, PartialEq, Eq)]
pub struct AddDeviceCommand {
    address: Address,
    address_type: AddressType,
    action: Action,
}

impl AddDeviceCommand {
    pub fn new(address: Address, address_type: AddressType, action: Action) -> Self {
        Self {
            address,
            address_type,
            action,
        }
    }
}

impl ManagementCommand for AddDeviceCommand {
    type Result = (Address, AddressType);

    fn into_mgmt(self, i: ControlIndex) -> MgmtCommand {
        MgmtCommand::AddDeviceCommand(i, self)
    }
}

impl CommandItem for AddDeviceCommand {
    const CODE: Code = Code(0x0033);
}

impl PacketData for AddDeviceCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let address = PacketData::unpack(buf)?;
        let address_type = PacketData::unpack(buf)?;
        let action = PacketData::unpack(buf)?;

        Ok(Self {
            address,
            address_type,
            action,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.address.pack(buf)?;
        self.address_type.pack(buf)?;
        self.action.pack(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = AddDeviceCommand::new(
            "00:11:22:33:44:55".parse().unwrap(),
            AddressType::LePublic,
            Action::AutoConnectRemoteDevice,
        );
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
