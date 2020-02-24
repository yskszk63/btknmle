use bytes::{Buf, BufMut};

use super::Status;
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct CommandStatusEvent {
    controller_index: ControlIndex,
    command_opcode: Code,
    status: Status,
}

impl CommandStatusEvent {
    pub fn new(controller_index: ControlIndex, command_opcode: Code, status: Status) -> Self {
        Self {
            controller_index,
            command_opcode,
            status,
        }
    }

    pub fn controller_index(&self) -> ControlIndex {
        self.controller_index.clone()
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

    fn with_controller_index(mut self, idx: ControlIndex) -> Self {
        self.controller_index = idx;
        self
    }
}

impl PacketData for CommandStatusEvent {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let command_opcode = PacketData::unpack(buf)?;
        let status = PacketData::unpack(buf)?;
        Ok(Self {
            controller_index: Default::default(),
            command_opcode,
            status,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.command_opcode.pack(buf)?;
        self.status.pack(buf)
    }
}

impl From<CommandStatusEvent> for MgmtEvent {
    fn from(v: CommandStatusEvent) -> Self {
        Self::CommandStatusEvent(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = CommandStatusEvent::new(Default::default(), Code(10), Status::Failed);
        e.pack(&mut b).unwrap();
        let r = CommandStatusEvent::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
