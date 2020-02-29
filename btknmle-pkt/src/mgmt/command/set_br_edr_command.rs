use bytes::{Buf, BufMut};

use super::CurrentSettings;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct SetBrEdrCommand {
    ctrl_idx: u16,
    br_edr: bool,
}

impl SetBrEdrCommand {
    pub fn new(ctrl_idx: u16, br_edr: bool) -> Self {
        Self { ctrl_idx, br_edr }
    }
}

impl ManagementCommand for SetBrEdrCommand {
    type Result = CurrentSettings;
}

impl CommandItem for SetBrEdrCommand {
    const CODE: Code = Code(0x002A);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl PacketData for SetBrEdrCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let br_edr = u8::unpack(buf)? != 0;
        Ok(Self {
            ctrl_idx: Default::default(),
            br_edr,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        (self.br_edr as u8).pack(buf)
    }
}

impl From<SetBrEdrCommand> for MgmtCommand {
    fn from(v: SetBrEdrCommand) -> Self {
        Self::SetBrEdrCommand(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = SetBrEdrCommand::new(Default::default(), true);
        e.pack(&mut b).unwrap();
        let r = SetBrEdrCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
