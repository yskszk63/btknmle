use bytes::{Buf, BufMut};

use super::CurrentSettings;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct SetConnectableCommand {
    connectable: bool,
}

impl SetConnectableCommand {
    pub fn new(connectable: bool) -> Self {
        Self { connectable }
    }
}

impl ManagementCommand for SetConnectableCommand {
    type Result = CurrentSettings;

    fn into_mgmt(self, i: ControlIndex) -> MgmtCommand {
        MgmtCommand::SetConnectableCommand(i, self)
    }
}

impl CommandItem for SetConnectableCommand {
    const CODE: Code = Code(0x0007);
}

impl PacketData for SetConnectableCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let connectable = u8::unpack(buf)? != 0;
        Ok(Self { connectable })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        (self.connectable as u8).pack(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = SetConnectableCommand::new(true);
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}