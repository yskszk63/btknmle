use bytes::{Buf, BufMut};

use super::CurrentSettings;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct SetLowEnergyCommand {
    low_energy: bool,
}

impl SetLowEnergyCommand {
    pub fn new(low_energy: bool) -> Self {
        Self { low_energy }
    }
}

impl ManagementCommand for SetLowEnergyCommand {
    type Result = CurrentSettings;

    fn into_mgmt(self, i: ControlIndex) -> MgmtCommand {
        MgmtCommand::SetLowEnergyCommand(i, self)
    }
}

impl CommandItem for SetLowEnergyCommand {
    const CODE: Code = Code(0x000D);
}

impl PacketData for SetLowEnergyCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let low_energy = u8::unpack(buf)? != 0;
        Ok(Self { low_energy })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        (self.low_energy as u8).pack(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = SetLowEnergyCommand::new(true);
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
