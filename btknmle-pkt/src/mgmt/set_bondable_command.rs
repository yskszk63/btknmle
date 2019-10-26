use bytes::{Buf, BufMut as _, BytesMut};

use super::CurrentSettings;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use super::{Codec, Result};

#[derive(Debug)]
pub struct SetBondableCommand {
    ctrl_idx: u16,
    bondable: bool,
}

impl SetBondableCommand {
    pub fn new(ctrl_idx: u16, bondable: bool) -> Self {
        Self { ctrl_idx, bondable }
    }
}

impl ManagementCommand<CurrentSettings> for SetBondableCommand {
    fn parse_result(buf: &mut impl Buf) -> Result<CurrentSettings> {
        Ok(CurrentSettings::parse(buf)?)
    }
}

impl CommandItem for SetBondableCommand {
    const CODE: Code = Code(0x0009);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl Codec for SetBondableCommand {
    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        buf.put_u8(if self.bondable { 0x01 } else { 0x00 });
        Ok(())
    }

    fn parse(_buf: &mut impl Buf) -> Result<Self> {
        unimplemented!()
    }
}

impl From<SetBondableCommand> for MgmtCommand {
    fn from(v: SetBondableCommand) -> Self {
        Self::SetBondableCommand(v)
    }
}
