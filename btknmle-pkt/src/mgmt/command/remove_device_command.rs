use bytes::{Buf, BufMut};

use super::ManagementCommand;
use super::{Address, AddressType};
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct RemoveDeviceCommand {
    address: Address,
    address_type: AddressType,
}

impl RemoveDeviceCommand {
    pub fn new(address: Address, address_type: AddressType) -> Self {
        Self {
            address,
            address_type,
        }
    }
}

impl ManagementCommand for RemoveDeviceCommand {
    type Result = (Address, AddressType);

    fn into_mgmt(self, i: ControlIndex) -> MgmtCommand {
        MgmtCommand::RemoveDeviceCommand(i, self)
    }
}

impl CommandItem for RemoveDeviceCommand {
    const CODE: Code = Code(0x0034);
}

impl PacketData for RemoveDeviceCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let address = PacketData::unpack(buf)?;
        let address_type = PacketData::unpack(buf)?;
        Ok(Self {
            address,
            address_type,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.address.pack(buf)?;
        self.address_type.pack(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e =
            RemoveDeviceCommand::new("00:11:22:33:44:55".parse().unwrap(), AddressType::LeRandom);
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
