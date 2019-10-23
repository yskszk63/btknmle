use bytes::{Buf, BufMut as _, BytesMut};

use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use super::{Codec, Result};
use super::ManagementCommand;
use super::CurrentSettings;


#[derive(Debug)]
pub struct SetBrEdrCommand {
    ctrl_idx: u16,
    br_edr: bool,
}

impl SetBrEdrCommand {
    pub fn new(ctrl_idx: u16, br_edr: bool) -> Self {
        Self {
            ctrl_idx,
            br_edr,
        }
    }
}

impl ManagementCommand<CurrentSettings> for SetBrEdrCommand {
    fn parse_result(buf: &mut impl Buf) -> Result<CurrentSettings> {
        Ok(CurrentSettings::parse(buf)?)
    }
}

impl CommandItem for SetBrEdrCommand {
    const CODE: Code = Code(0x002A);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl Codec for SetBrEdrCommand {
    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        buf.put_u8(if self.br_edr { 0x01 } else { 0x00 });
        Ok(())
    }

    fn parse(_buf: &mut impl Buf) -> Result<Self> {
        unimplemented!()
    }
}

impl From<SetBrEdrCommand> for MgmtCommand {
    fn from(v: SetBrEdrCommand) -> Self {
        Self::SetBrEdrCommand(v)
    }
}
