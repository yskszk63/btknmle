use bytes::{Buf, BufMut};

use super::CurrentSettings;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct SetBondableCommand {
    ctrl_idx: u16,
    bondable: bool,
}

impl SetBondableCommand {
    pub fn new(ctrl_idx: u16, bondable: bool) -> Self {
        Self { ctrl_idx, bondable }
    }
}

impl ManagementCommand for SetBondableCommand {
    type Result = CurrentSettings;
}

impl CommandItem for SetBondableCommand {
    const CODE: Code = Code(0x0009);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl PacketData for SetBondableCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let bondable = u8::unpack(buf)? != 0;
        Ok(Self {
            ctrl_idx: Default::default(),
            bondable,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        (self.bondable as u8).pack(buf)
    }
}

impl From<SetBondableCommand> for MgmtCommand {
    fn from(v: SetBondableCommand) -> Self {
        Self::SetBondableCommand(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = SetBondableCommand::new(Default::default(), true);
        e.pack(&mut b).unwrap();
        let r = SetBondableCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
