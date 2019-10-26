use bytes::{Buf, BufMut as _, BytesMut};

use super::CurrentSettings;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use super::{Codec, Result};

#[derive(Debug)]
pub struct SetConnectableCommand {
    ctrl_idx: u16,
    connectable: bool,
}

impl SetConnectableCommand {
    pub fn new(ctrl_idx: u16, connectable: bool) -> Self {
        Self {
            ctrl_idx,
            connectable,
        }
    }
}

impl ManagementCommand<CurrentSettings> for SetConnectableCommand {
    fn parse_result(buf: &mut impl Buf) -> Result<CurrentSettings> {
        Ok(CurrentSettings::parse(buf)?)
    }
}

impl CommandItem for SetConnectableCommand {
    const CODE: Code = Code(0x0007);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl Codec for SetConnectableCommand {
    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        buf.put_u8(if self.connectable { 0x01 } else { 0x00 });
        Ok(())
    }

    fn parse(_buf: &mut impl Buf) -> Result<Self> {
        unimplemented!()
    }
}

impl From<SetConnectableCommand> for MgmtCommand {
    fn from(v: SetConnectableCommand) -> Self {
        Self::SetConnectableCommand(v)
    }
}
