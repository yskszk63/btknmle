use bytes::{Buf, BufMut};

use super::Status;
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct CommandStatusEvent {
    command_opcode: Code,
    status: Status,
}

impl CommandStatusEvent {
    pub fn new(command_opcode: Code, status: Status) -> Self {
        Self {
            command_opcode,
            status,
        }
    }

    pub fn command_opcode(&self) -> Code {
        self.command_opcode.clone()
    }

    pub fn status(&self) -> Status {
        self.status.clone()
    }
}

impl EventItem for CommandStatusEvent {
    const CODE: Code = Code(0x0002);

    fn into_mgmt(self, index: ControlIndex) -> MgmtEvent {
        MgmtEvent::CommandStatusEvent(index, self)
    }
}

impl PacketData for CommandStatusEvent {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let command_opcode = PacketData::unpack(buf)?;
        let status = PacketData::unpack(buf)?;
        Ok(Self {
            command_opcode,
            status,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.command_opcode.pack(buf)?;
        self.status.pack(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = CommandStatusEvent::new(Code(10), Status::Failed);
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtEvent::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
