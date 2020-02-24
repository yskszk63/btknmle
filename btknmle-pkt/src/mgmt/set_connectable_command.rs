use bytes::{Buf, BufMut};

use super::CurrentSettings;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct SetConnectableCommand {
    ctrl_idx: u16,
    connectable: bool,
}

impl SetConnectableCommand {
    pub fn new(ctrl_idx: u16, connectable: bool) -> Self {
        Self {
            ctrl_idx,
            connectable,
        }
    }
}

impl ManagementCommand<CurrentSettings> for SetConnectableCommand {
    fn parse_result(buf: &mut impl Buf) -> Result<CurrentSettings, crate::CodecError> {
        Ok(CurrentSettings::unpack(buf)?)
    }
}

impl CommandItem for SetConnectableCommand {
    const CODE: Code = Code(0x0007);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl PacketData for SetConnectableCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let connectable = u8::unpack(buf)? != 0;
        Ok(Self {
            ctrl_idx: Default::default(),
            connectable,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        (self.connectable as u8).pack(buf)
    }
}

impl From<SetConnectableCommand> for MgmtCommand {
    fn from(v: SetConnectableCommand) -> Self {
        Self::SetConnectableCommand(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = SetConnectableCommand::new(Default::default(), true);
        e.pack(&mut b).unwrap();
        let r = SetConnectableCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
