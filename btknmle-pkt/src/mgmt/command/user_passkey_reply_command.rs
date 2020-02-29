use bytes::{Buf, BufMut};

use super::ManagementCommand;
use super::{Address, AddressType};
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct UserPasskeyReplyCommand {
    address: Address,
    address_type: AddressType,
    passkey: u32,
}

impl UserPasskeyReplyCommand {
    pub fn new(address: Address, address_type: AddressType, passkey: u32) -> Self {
        Self {
            address,
            address_type,
            passkey,
        }
    }
}

impl ManagementCommand for UserPasskeyReplyCommand {
    type Result = (Address, AddressType);

    fn into_mgmt(self, i: ControlIndex) -> MgmtCommand {
        MgmtCommand::UserPasskeyReplyCommand(i, self)
    }
}

impl CommandItem for UserPasskeyReplyCommand {
    const CODE: Code = Code(0x001E);
}

impl PacketData for UserPasskeyReplyCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let address = PacketData::unpack(buf)?;
        let address_type = PacketData::unpack(buf)?;
        let passkey = PacketData::unpack(buf)?;
        Ok(Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = UserPasskeyReplyCommand::new(
            "00:11:22:33:44:55".parse().unwrap(),
            AddressType::LeRandom,
            3,
        );
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
