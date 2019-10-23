use bytes::{Buf, BytesMut};

use super::{Code, ControlIndex, EventItem, MgmtEvent};
use super::{Codec, Result};
use super::Status;

#[derive(Debug)]
pub struct CommandStatusEvent {
    controller_index: ControlIndex,
    command_opcode: Code,
    status: Status,
}

impl CommandStatusEvent {
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

impl Codec for CommandStatusEvent {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let controller_index = Default::default();
        let command_opcode = buf.get_u16_le().into();
        let status = buf.get_u8().into();
        Ok(Self {
            controller_index,
            command_opcode,
            status,
        })
    }

    fn write_to(&self, _buf: &mut BytesMut) -> Result<()> {
        unimplemented!()
    }
}

impl From<CommandStatusEvent> for MgmtEvent {
    fn from(v: CommandStatusEvent) -> Self {
        Self::CommandStatusEvent(v)
    }
}
