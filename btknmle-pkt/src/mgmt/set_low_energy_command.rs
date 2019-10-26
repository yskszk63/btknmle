use bytes::{Buf, BufMut as _, BytesMut};

use super::CurrentSettings;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use super::{Codec, Result};

#[derive(Debug)]
pub struct SetLowEnergyCommand {
    ctrl_idx: u16,
    low_energy: bool,
}

impl SetLowEnergyCommand {
    pub fn new(ctrl_idx: u16, low_energy: bool) -> Self {
        Self {
            ctrl_idx,
            low_energy,
        }
    }
}

impl ManagementCommand<CurrentSettings> for SetLowEnergyCommand {
    fn parse_result(buf: &mut impl Buf) -> Result<CurrentSettings> {
        Ok(CurrentSettings::parse(buf)?)
    }
}

impl CommandItem for SetLowEnergyCommand {
    const CODE: Code = Code(0x000D);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl Codec for SetLowEnergyCommand {
    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        buf.put_u8(if self.low_energy { 0x01 } else { 0x00 });
        Ok(())
    }

    fn parse(_buf: &mut impl Buf) -> Result<Self> {
        unimplemented!()
    }
}

impl From<SetLowEnergyCommand> for MgmtCommand {
    fn from(v: SetLowEnergyCommand) -> Self {
        Self::SetLowEnergyCommand(v)
    }
}
