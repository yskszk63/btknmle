use bytes::{Buf, BufMut};

use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct SetAppearanceCommand {
    appearance: u16,
}

impl SetAppearanceCommand {
    pub fn new(appearance: u16) -> Self {
        Self { appearance }
    }
}

impl ManagementCommand for SetAppearanceCommand {
    type Result = ();

    fn into_mgmt(self, i: ControlIndex) -> MgmtCommand {
        MgmtCommand::SetAppearanceCommand(i, self)
    }
}

impl CommandItem for SetAppearanceCommand {
    const CODE: Code = Code(0x0043);
}

impl PacketData for SetAppearanceCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let appearance = PacketData::unpack(buf)?;
        Ok(Self { appearance })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.appearance.pack(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = SetAppearanceCommand::new(3);
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
