use bytes::{Buf, Bytes, BytesMut};

use super::Status;
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use super::{Codec, Result};
use crate::util::HexDisplay;

#[derive(Debug)]
pub struct CommandCompleteEvent {
    controller_index: ControlIndex,
    command_opcode: Code,
    status: Status,
    parameters: HexDisplay<Bytes>,
}

impl CommandCompleteEvent {
    pub fn controller_index(&self) -> ControlIndex {
        self.controller_index.clone()
    }

    pub fn command_opcode(&self) -> Code {
        self.command_opcode.clone()
    }

    pub fn status(&self) -> Status {
        self.status.clone()
    }

    pub fn parameters(&self) -> &[u8] {
        self.parameters.as_ref()
    }
}

impl EventItem for CommandCompleteEvent {
    const CODE: Code = Code(0x0001);

    fn with_controller_index(mut self, idx: ControlIndex) -> Self {
        self.controller_index = idx;
        self
    }
}

impl Codec for CommandCompleteEvent {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let controller_index = Default::default();
        let command_opcode = buf.get_u16_le().into();
        let status = buf.get_u8().into();
        let parameters = HexDisplay::new(buf.to_bytes());
        Ok(Self {
            controller_index,
            command_opcode,
            status,
            parameters,
        })
    }

    fn write_to(&self, _buf: &mut BytesMut) -> Result<()> {
        unimplemented!()
    }
}

impl From<CommandCompleteEvent> for MgmtEvent {
    fn from(v: CommandCompleteEvent) -> Self {
        Self::CommandCompleteEvent(v)
    }
}
