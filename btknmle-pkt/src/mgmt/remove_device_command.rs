use bytes::{Buf, BytesMut};

use super::ManagementCommand;
use super::{Address, AddressType};
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use super::{Codec, Result};

#[derive(Debug)]
pub struct RemoveDeviceCommand {
    ctrl_idx: u16,
    address: Address,
    address_type: AddressType,
}

impl RemoveDeviceCommand {
    pub fn new(ctrl_idx: u16, address: Address, address_type: AddressType) -> Self {
        Self {
            ctrl_idx,
            address,
            address_type,
        }
    }
}

impl ManagementCommand<(Address, AddressType)> for RemoveDeviceCommand {
    fn parse_result(buf: &mut impl Buf) -> Result<(Address, AddressType)> {
        let address = Address::parse(buf)?;
        let address_type = AddressType::parse(buf)?;
        Ok((address, address_type))
    }
}

impl CommandItem for RemoveDeviceCommand {
    const CODE: Code = Code(0x0034);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl Codec for RemoveDeviceCommand {
    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        self.address.write_to(buf)?;
        self.address_type.write_to(buf)?;
        Ok(())
    }

    fn parse(_buf: &mut impl Buf) -> Result<Self> {
        unimplemented!()
    }
}

impl From<RemoveDeviceCommand> for MgmtCommand {
    fn from(v: RemoveDeviceCommand) -> Self {
        Self::RemoveDeviceCommand(v)
    }
}
