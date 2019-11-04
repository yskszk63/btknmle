use bytes::{Buf, BufMut as _, BytesMut};

use super::IdentityResolvingKey;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use super::{Codec, Result};

#[derive(Debug)]
pub struct LoadIdentityResolvingKeysCommand {
    ctrl_idx: u16,
    keys: Vec<IdentityResolvingKey>,
}

impl LoadIdentityResolvingKeysCommand {
    pub fn new(ctrl_idx: u16, keys: Vec<IdentityResolvingKey>) -> Self {
        Self { ctrl_idx, keys }
    }
}

impl ManagementCommand<()> for LoadIdentityResolvingKeysCommand {
    fn parse_result(_buf: &mut impl Buf) -> Result<()> {
        Ok(())
    }
}

impl CommandItem for LoadIdentityResolvingKeysCommand {
    const CODE: Code = Code(0x0030);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl Codec for LoadIdentityResolvingKeysCommand {
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

impl From<LoadIdentityResolvingKeysCommand> for MgmtCommand {
    fn from(v: LoadIdentityResolvingKeysCommand) -> Self {
        Self::LoadIdentityResolvingKeysCommand(v)
    }
}
