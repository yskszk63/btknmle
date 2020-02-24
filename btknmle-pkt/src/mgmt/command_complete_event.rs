use bytes::{Buf, BufMut, Bytes};

use super::Status;
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use crate::util::HexDisplay;
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct CommandCompleteEvent {
    controller_index: ControlIndex,
    command_opcode: Code,
    status: Status,
    parameters: HexDisplay<Bytes>,
}

impl CommandCompleteEvent {
    pub fn new(
        controller_index: ControlIndex,
        command_opcode: Code,
        status: Status,
        parameters: Bytes,
    ) -> Self {
        let parameters = parameters.into();
        Self {
            controller_index,
            command_opcode,
            status,
            parameters,
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

impl PacketData for CommandCompleteEvent {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let command_opcode = PacketData::unpack(buf)?;
        let status = PacketData::unpack(buf)?;
        let parameters = buf.to_bytes().into();
        Ok(Self {
            controller_index: Default::default(),
            command_opcode,
            status,
            parameters,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.command_opcode.pack(buf)?;
        self.status.pack(buf)?;
        if buf.remaining_mut() < self.parameters.len() {
            return Err(PackError::InsufficientBufLength);
        }
        buf.put(self.parameters.as_ref());
        Ok(())
    }
}

impl From<CommandCompleteEvent> for MgmtEvent {
    fn from(v: CommandCompleteEvent) -> Self {
        Self::CommandCompleteEvent(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = CommandCompleteEvent::new(
            Default::default(),
            Code(10),
            Status::Busy,
            Bytes::from("ok"),
        );
        e.pack(&mut b).unwrap();
        let r = CommandCompleteEvent::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
