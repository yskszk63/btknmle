use bytes::{Buf, BufMut};

use super::CurrentSettings;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct SetBondableCommand {
    bondable: bool,
}

impl SetBondableCommand {
    pub fn new(bondable: bool) -> Self {
        Self { bondable }
    }
}

impl ManagementCommand for SetBondableCommand {
    type Result = CurrentSettings;

    fn into_mgmt(self, i: ControlIndex) -> MgmtCommand {
        MgmtCommand::SetBondableCommand(i, self)
    }
}

impl CommandItem for SetBondableCommand {
    const CODE: Code = Code(0x0009);
}

impl PacketData for SetBondableCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let bondable = u8::unpack(buf)? != 0;
        Ok(Self { bondable })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        (self.bondable as u8).pack(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = SetBondableCommand::new(true);
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
