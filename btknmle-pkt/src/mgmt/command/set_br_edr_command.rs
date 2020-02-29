use bytes::{Buf, BufMut};

use super::CurrentSettings;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct SetBrEdrCommand {
    br_edr: bool,
}

impl SetBrEdrCommand {
    pub fn new(br_edr: bool) -> Self {
        Self { br_edr }
    }
}

impl ManagementCommand for SetBrEdrCommand {
    type Result = CurrentSettings;

    fn into_mgmt(self, i: ControlIndex) -> MgmtCommand {
        MgmtCommand::SetBrEdrCommand(i, self)
    }
}

impl CommandItem for SetBrEdrCommand {
    const CODE: Code = Code(0x002A);
}

impl PacketData for SetBrEdrCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let br_edr = u8::unpack(buf)? != 0;
        Ok(Self { br_edr })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        (self.br_edr as u8).pack(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = SetBrEdrCommand::new(true);
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
