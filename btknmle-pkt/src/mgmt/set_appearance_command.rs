use bytes::{Buf, BufMut as _, BytesMut};

use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use super::{Codec, Result};

#[derive(Debug)]
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
    fn parse_result(_buf: &mut impl Buf) -> Result<()> {
        Ok(())
    }
}

impl CommandItem for SetAppearanceCommand {
    const CODE: Code = Code(0x0043);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl Codec for SetAppearanceCommand {
    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        buf.put_u16_le(self.appearance);
        Ok(())
    }

    fn parse(_buf: &mut impl Buf) -> Result<Self> {
        unimplemented!()
    }
}

impl From<SetAppearanceCommand> for MgmtCommand {
    fn from(v: SetAppearanceCommand) -> Self {
        Self::SetAppearanceCommand(v)
    }
}
