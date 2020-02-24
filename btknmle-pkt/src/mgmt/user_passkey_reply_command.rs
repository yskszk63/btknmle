use bytes::{Buf, BufMut};

use super::ManagementCommand;
use super::{Address, AddressType};
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct UserPasskeyReplyCommand {
    ctrl_idx: u16,
    address: Address,
    address_type: AddressType,
    passkey: u32,
}

impl UserPasskeyReplyCommand {
    pub fn new(ctrl_idx: u16, address: Address, address_type: AddressType, passkey: u32) -> Self {
        Self {
            ctrl_idx,
            address,
            address_type,
            passkey,
        }
    }
}

impl ManagementCommand<(Address, AddressType)> for UserPasskeyReplyCommand {
    fn parse_result(buf: &mut impl Buf) -> Result<(Address, AddressType), crate::CodecError> {
        let address = Address::unpack(buf)?;
        let address_type = AddressType::unpack(buf)?;
        Ok((address, address_type))
    }
}

impl CommandItem for UserPasskeyReplyCommand {
    const CODE: Code = Code(0x001E);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl PacketData for UserPasskeyReplyCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let address = PacketData::unpack(buf)?;
        let address_type = PacketData::unpack(buf)?;
        let passkey = PacketData::unpack(buf)?;
        Ok(Self {
            ctrl_idx: Default::default(),
            address,
            address_type,
            passkey,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.address.pack(buf)?;
        self.address_type.pack(buf)?;
        self.passkey.pack(buf)
    }
}

impl From<UserPasskeyReplyCommand> for MgmtCommand {
    fn from(v: UserPasskeyReplyCommand) -> Self {
        Self::UserPasskeyReplyCommand(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = UserPasskeyReplyCommand::new(
            Default::default(),
            "00:11:22:33:44:55".parse().unwrap(),
            AddressType::LeRandom,
            3,
        );
        e.pack(&mut b).unwrap();
        let r = UserPasskeyReplyCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
