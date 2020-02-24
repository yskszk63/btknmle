use bytes::{Buf, BufMut};

use super::CurrentSettings;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
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
    fn parse_result(buf: &mut impl Buf) -> Result<CurrentSettings, crate::CodecError> {
        Ok(CurrentSettings::unpack(buf)?)
    }
}

impl CommandItem for SetLowEnergyCommand {
    const CODE: Code = Code(0x000D);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl PacketData for SetLowEnergyCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let low_energy = u8::unpack(buf)? != 0;
        Ok(Self {
            ctrl_idx: Default::default(),
            low_energy,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        (self.low_energy as u8).pack(buf)
    }
}

impl From<SetLowEnergyCommand> for MgmtCommand {
    fn from(v: SetLowEnergyCommand) -> Self {
        Self::SetLowEnergyCommand(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = SetLowEnergyCommand::new(Default::default(), true);
        e.pack(&mut b).unwrap();
        let r = SetLowEnergyCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
