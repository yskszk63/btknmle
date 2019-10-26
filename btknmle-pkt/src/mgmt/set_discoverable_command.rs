use bytes::{Buf, BufMut as _, BytesMut};

use super::CurrentSettings;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use super::{Codec, Result};

#[derive(Debug)]
pub enum Discoverable {
    Disabled,
    General,
    Limited(u16),
}

#[derive(Debug)]
pub struct SetDiscoverableCommand {
    ctrl_idx: u16,
    discoverable: Discoverable,
}

impl SetDiscoverableCommand {
    pub fn new(ctrl_idx: u16, discoverable: Discoverable) -> Self {
        Self {
            ctrl_idx,
            discoverable,
        }
    }
}

impl ManagementCommand<CurrentSettings> for SetDiscoverableCommand {
    fn parse_result(buf: &mut impl Buf) -> Result<CurrentSettings> {
        Ok(CurrentSettings::parse(buf)?)
    }
}

impl CommandItem for SetDiscoverableCommand {
    const CODE: Code = Code(0x0006);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl Codec for SetDiscoverableCommand {
    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        let (v, t) = match self.discoverable {
            Discoverable::Disabled => (0x00, 0x0000),
            Discoverable::General => (0x01, 0x0000),
            Discoverable::Limited(t) => (0x02, t),
        };
        buf.put_u8(v);
        buf.put_u16_le(t);
        Ok(())
    }

    fn parse(_buf: &mut impl Buf) -> Result<Self> {
        unimplemented!()
    }
}

impl From<SetDiscoverableCommand> for MgmtCommand {
    fn from(v: SetDiscoverableCommand) -> Self {
        Self::SetDiscoverableCommand(v)
    }
}
