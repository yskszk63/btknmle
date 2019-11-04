use bytes::{Buf, BufMut as _, BytesMut};

use super::LongTermKey;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use super::{Codec, Result};

#[derive(Debug)]
pub struct LoadLongTermKeysCommand {
    ctrl_idx: u16,
    keys: Vec<LongTermKey>,
}

impl LoadLongTermKeysCommand {
    pub fn new(ctrl_idx: u16, keys: Vec<LongTermKey>) -> Self {
        Self { ctrl_idx, keys }
    }
}

impl ManagementCommand<()> for LoadLongTermKeysCommand {
    fn parse_result(_buf: &mut impl Buf) -> Result<()> {
        Ok(())
    }
}

impl CommandItem for LoadLongTermKeysCommand {
    const CODE: Code = Code(0x0013);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl Codec for LoadLongTermKeysCommand {
    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        buf.put_u16_le(self.keys.len() as u16);
        for key in &self.keys {
            key.write_to(buf)?;
        }
        Ok(())
    }

    fn parse(_buf: &mut impl Buf) -> Result<Self> {
        unimplemented!()
    }
}

impl From<LoadLongTermKeysCommand> for MgmtCommand {
    fn from(v: LoadLongTermKeysCommand) -> Self {
        Self::LoadLongTermKeysCommand(v)
    }
}
