use bytes::{Buf, BufMut as _, BytesMut};

use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use super::{Codec, Result};
use super::ManagementCommand;
use super::CurrentSettings;


#[derive(Debug)]
pub struct SetPoweredCommand {
    ctrl_idx: u16,
    powered: bool,
}

impl SetPoweredCommand {
    pub fn new(ctrl_idx: u16, powered: bool) -> Self {
        Self { ctrl_idx, powered }
    }
}

impl ManagementCommand<CurrentSettings> for SetPoweredCommand {
    fn parse_result(buf: &mut impl Buf) -> Result<CurrentSettings> {
        Ok(CurrentSettings::parse(buf)?)
    }
}

impl CommandItem for SetPoweredCommand {
    const CODE: Code = Code(0x0005);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl Codec for SetPoweredCommand {
    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        buf.put_u8(if self.powered { 0x01 } else { 0x00 });
        Ok(())
    }

    fn parse(_buf: &mut impl Buf) -> Result<Self> {
        unimplemented!()
    }
}

impl From<SetPoweredCommand> for MgmtCommand {
    fn from(v: SetPoweredCommand) -> Self {
        Self::SetPoweredCommand(v)
    }
}
