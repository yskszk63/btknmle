use std::num::NonZeroU8;

use bytes::{Buf, BufMut as _, BytesMut};

use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use super::{Codec, Result};

#[derive(Debug)]
pub struct RemoveAdvertisingCommand {
    ctrl_idx: u16,
    instance: u8,
}

impl RemoveAdvertisingCommand {
    pub fn new(
        ctrl_idx: u16,
        instance: Option<NonZeroU8>,
    ) -> Self {
        let instance = instance.map(NonZeroU8::get).unwrap_or_else(|| 0);
        Self {
            ctrl_idx,
            instance,
        }
    }
}

impl ManagementCommand<u8> for RemoveAdvertisingCommand {
    fn parse_result(buf: &mut impl Buf) -> Result<u8> {
        Ok(buf.get_u8())
    }
}

impl CommandItem for RemoveAdvertisingCommand {
    const CODE: Code = Code(0x003f);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl Codec for RemoveAdvertisingCommand {
    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        buf.put_u8(self.instance);
        Ok(())
    }

    fn parse(_buf: &mut impl Buf) -> Result<Self> {
        unimplemented!()
    }
}

impl From<RemoveAdvertisingCommand> for MgmtCommand {
    fn from(v: RemoveAdvertisingCommand) -> Self {
        Self::RemoveAdvertisingCommand(v)
    }
}
