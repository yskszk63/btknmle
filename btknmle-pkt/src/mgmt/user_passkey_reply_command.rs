use bytes::{Buf, BufMut as _, BytesMut};

use super::ManagementCommand;
use super::{Address, AddressType};
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use super::{Codec, Result};

#[derive(Debug)]
pub struct UserPasskeyReplyCommand {
    ctrl_idx: u16,
    address: Address,
    address_type: AddressType,
    passkey: [u8; 4],
}

impl UserPasskeyReplyCommand {
    pub fn new(ctrl_idx: u16, address: Address, address_type: AddressType, passkey: u32) -> Self {
        let passkey = passkey.to_le_bytes();
        Self {
            ctrl_idx,
            address,
            address_type,
            passkey,
        }
    }
}

impl ManagementCommand<(Address, AddressType)> for UserPasskeyReplyCommand {
    fn parse_result(buf: &mut impl Buf) -> Result<(Address, AddressType)> {
        let address = Address::parse(buf)?;
        let address_type = AddressType::parse(buf)?;
        Ok((address, address_type))
    }
}

impl CommandItem for UserPasskeyReplyCommand {
    const CODE: Code = Code(0x001E);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl Codec for UserPasskeyReplyCommand {
    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        self.address.write_to(buf)?;
        self.address_type.write_to(buf)?;
        buf.put(self.passkey.as_ref());
        Ok(())
    }

    fn parse(_buf: &mut impl Buf) -> Result<Self> {
        unimplemented!()
    }
}

impl From<UserPasskeyReplyCommand> for MgmtCommand {
    fn from(v: UserPasskeyReplyCommand) -> Self {
        Self::UserPasskeyReplyCommand(v)
    }
}
