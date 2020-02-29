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
            x => return Err(UnpackError::UnexpectedValue(x)),
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
    io_capability: IoCapability,
}

impl SetIoCapabilityCommand {
    pub fn new(io_capability: IoCapability) -> Self {
        Self { io_capability }
    }
}

impl ManagementCommand for SetIoCapabilityCommand {
    type Result = ();

    fn into_mgmt(self, i: ControlIndex) -> MgmtCommand {
        MgmtCommand::SetIoCapabilityCommand(i, self)
    }
}

impl CommandItem for SetIoCapabilityCommand {
    const CODE: Code = Code(0x0018);
}

impl PacketData for SetIoCapabilityCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let io_capability = PacketData::unpack(buf)?;
        Ok(Self { io_capability })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.io_capability.pack(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = SetIoCapabilityCommand::new(IoCapability::KeyboardDisplay);
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
