use bytes::{Buf, BufMut};

use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub enum IoCapability {
    DisplayOnly,
    DisplayYesNo,
    KeyboardOnly,
    NoInputNoOutput,
    KeyboardDisplay,
}

impl PacketData for IoCapability {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let v = u8::unpack(buf)?;
        Ok(match v {
            0x00 => IoCapability::DisplayOnly,
            0x01 => IoCapability::DisplayYesNo,
            0x02 => IoCapability::KeyboardOnly,
            0x03 => IoCapability::NoInputNoOutput,
            0x04 => IoCapability::KeyboardDisplay,
            x => return Err(UnpackError::unexpected(format!("value {}", x))),
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        let v = match self {
            IoCapability::DisplayOnly => 0x00,
            IoCapability::DisplayYesNo => 0x01,
            IoCapability::KeyboardOnly => 0x02,
            IoCapability::NoInputNoOutput => 0x03,
            IoCapability::KeyboardDisplay => 0x04,
        };
        u8::pack(&v, buf)
    }
}
#[derive(Debug, PartialEq, Eq)]
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

impl ManagementCommand for SetIoCapabilityCommand {
    type Result = ();
}

impl CommandItem for SetIoCapabilityCommand {
    const CODE: Code = Code(0x0018);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl PacketData for SetIoCapabilityCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let io_capability = PacketData::unpack(buf)?;
        Ok(Self {
            ctrl_idx: Default::default(),
            io_capability,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.io_capability.pack(buf)
    }
}

impl From<SetIoCapabilityCommand> for MgmtCommand {
    fn from(v: SetIoCapabilityCommand) -> Self {
        Self::SetIoCapabilityCommand(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = SetIoCapabilityCommand::new(Default::default(), IoCapability::KeyboardDisplay);
        e.pack(&mut b).unwrap();
        let r = SetIoCapabilityCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
