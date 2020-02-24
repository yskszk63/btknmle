use bytes::{Buf, BufMut};

use super::CurrentSettings;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct SetPoweredCommand {
    ctrl_idx: u16,
    powered: bool,
}

impl SetPoweredCommand {
    pub fn new(ctrl_idx: u16, powered: bool) -> Self {
        Self { ctrl_idx, powered }
    }
}

impl ManagementCommand for SetPoweredCommand {
    type Result = CurrentSettings;
}

impl CommandItem for SetPoweredCommand {
    const CODE: Code = Code(0x0005);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl PacketData for SetPoweredCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let powered = u8::unpack(buf)? != 0;
        Ok(Self {
            ctrl_idx: Default::default(),
            powered,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        (self.powered as u8).pack(buf)
    }
}

impl From<SetPoweredCommand> for MgmtCommand {
    fn from(v: SetPoweredCommand) -> Self {
        Self::SetPoweredCommand(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = SetPoweredCommand::new(Default::default(), true);
        e.pack(&mut b).unwrap();
        let r = SetPoweredCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
