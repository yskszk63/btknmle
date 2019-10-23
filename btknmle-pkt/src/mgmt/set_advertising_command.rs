use bytes::{Buf, BufMut as _, BytesMut};

use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use super::{Codec, Result};
use super::ManagementCommand;
use super::CurrentSettings;

#[derive(Debug)]
pub enum Advertising {
    Disabled,
    Enabled,
    Connectable,
}

#[derive(Debug)]
pub struct SetAdvertisingCommand {
    ctrl_idx: u16,
    advertising: Advertising,
}

impl SetAdvertisingCommand {
    pub fn new(ctrl_idx: u16, advertising: Advertising) -> Self {
        Self {
            ctrl_idx,
            advertising,
        }
    }
}

impl ManagementCommand<CurrentSettings> for SetAdvertisingCommand {
    fn parse_result(buf: &mut impl Buf) -> Result<CurrentSettings> {
        Ok(CurrentSettings::parse(buf)?)
    }
}

impl CommandItem for SetAdvertisingCommand {
    const CODE: Code = Code(0x0029);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl Codec for SetAdvertisingCommand {
    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        let v = match self.advertising {
            Advertising::Disabled => 0x00,
            Advertising::Enabled => 0x01,
            Advertising::Connectable => 0x02,
        };
        buf.put_u8(v);
        Ok(())
    }

    fn parse(_buf: &mut impl Buf) -> Result<Self> {
        unimplemented!()
    }
}

impl From<SetAdvertisingCommand> for MgmtCommand {
    fn from(v: SetAdvertisingCommand) -> Self {
        Self::SetAdvertisingCommand(v)
    }
}
