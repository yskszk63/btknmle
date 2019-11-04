use bytes::{Buf, BufMut as _, BytesMut};

use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use super::{Codec, Result};

#[derive(Debug)]
pub enum IoCapability {
    DisplayOnly,
    DisplayYesNo,
    KeyboardOnly,
    NoInputNoOutput,
    KeyboardDisplay,
}

#[derive(Debug)]
pub struct SetIoCapabilityCommand {
    ctrl_idx: u16,
    io_capability: IoCapability,
}

impl SetIoCapabilityCommand {
    pub fn new(ctrl_idx: u16, io_capability: IoCapability) -> Self {
        Self {
            ctrl_idx,
            io_capability,
        }
    }
}

impl ManagementCommand<()> for SetIoCapabilityCommand {
    fn parse_result(_buf: &mut impl Buf) -> Result<()> {
        Ok(())
    }
}

impl CommandItem for SetIoCapabilityCommand {
    const CODE: Code = Code(0x0018);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl Codec for SetIoCapabilityCommand {
    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        let v = match self.io_capability {
            IoCapability::DisplayOnly => 0x00,
            IoCapability::DisplayYesNo => 0x01,
            IoCapability::KeyboardOnly => 0x02,
            IoCapability::NoInputNoOutput => 0x03,
            IoCapability::KeyboardDisplay => 0x04,
        };
        buf.put_u8(v);
        Ok(())
    }

    fn parse(_buf: &mut impl Buf) -> Result<Self> {
        unimplemented!()
    }
}

impl From<SetIoCapabilityCommand> for MgmtCommand {
    fn from(v: SetIoCapabilityCommand) -> Self {
        Self::SetIoCapabilityCommand(v)
    }
}
