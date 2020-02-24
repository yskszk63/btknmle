use bytes::{Buf, BufMut};

use super::ManagementCommand;
use super::{Address, AddressType};
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct UserConfirmationReplyCommand {
    ctrl_idx: u16,
    address: Address,
    address_type: AddressType,
}

impl UserConfirmationReplyCommand {
    pub fn new(ctrl_idx: u16, address: Address, address_type: AddressType) -> Self {
        Self {
            ctrl_idx,
            address,
            address_type,
        }
    }
}

impl ManagementCommand for UserConfirmationReplyCommand {
    type Result = (Address, AddressType);
}

impl CommandItem for UserConfirmationReplyCommand {
    const CODE: Code = Code(0x001C);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl PacketData for UserConfirmationReplyCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let address = PacketData::unpack(buf)?;
        let address_type = PacketData::unpack(buf)?;
        Ok(Self {
            ctrl_idx: Default::default(),
            address,
            address_type,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.address.pack(buf)?;
        self.address_type.pack(buf)
    }
}

impl From<UserConfirmationReplyCommand> for MgmtCommand {
    fn from(v: UserConfirmationReplyCommand) -> Self {
        Self::UserConfirmationReplyCommand(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = UserConfirmationReplyCommand::new(
            Default::default(),
            "00:11:22:33:44:55".parse().unwrap(),
            AddressType::LeRandom,
        );
        e.pack(&mut b).unwrap();
        let r = UserConfirmationReplyCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
