use bytes::{Buf, BufMut};

use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct SetAppearanceCommand {
    ctrl_idx: u16,
    appearance: u16,
}

impl SetAppearanceCommand {
    pub fn new(ctrl_idx: u16, appearance: u16) -> Self {
        Self {
            ctrl_idx,
            appearance,
        }
    }
}

impl ManagementCommand<()> for SetAppearanceCommand {
    fn parse_result(_buf: &mut impl Buf) -> Result<(), crate::CodecError> {
        Ok(())
    }
}

impl CommandItem for SetAppearanceCommand {
    const CODE: Code = Code(0x0043);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl PacketData for SetAppearanceCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let appearance = PacketData::unpack(buf)?;
        Ok(Self {
            ctrl_idx: Default::default(),
            appearance,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.appearance.pack(buf)
    }
}

impl From<SetAppearanceCommand> for MgmtCommand {
    fn from(v: SetAppearanceCommand) -> Self {
        Self::SetAppearanceCommand(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = SetAppearanceCommand::new(Default::default(), 3);
        e.pack(&mut b).unwrap();
        let r = SetAppearanceCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
