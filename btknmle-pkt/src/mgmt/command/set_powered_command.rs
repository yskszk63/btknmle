use bytes::{Buf, BufMut};

use super::CurrentSettings;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct SetPoweredCommand {
    powered: bool,
}

impl SetPoweredCommand {
    pub fn new(powered: bool) -> Self {
        Self { powered }
    }
}

impl ManagementCommand for SetPoweredCommand {
    type Result = CurrentSettings;

    fn into_mgmt(self, i: ControlIndex) -> MgmtCommand {
        MgmtCommand::SetPoweredCommand(i, self)
    }
}

impl CommandItem for SetPoweredCommand {
    const CODE: Code = Code(0x0005);
}

impl PacketData for SetPoweredCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let powered = u8::unpack(buf)? != 0;
        Ok(Self { powered })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        (self.powered as u8).pack(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = SetPoweredCommand::new(true);
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}