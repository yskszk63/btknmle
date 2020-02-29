use bytes::buf::BufExt as _;
use bytes::{Buf, BufMut, Bytes};

use super::{Code, ControlIndex, EventItem, MgmtEvent};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct ExtendedControllerInformationChangedEvent {
    controller_index: ControlIndex,
    eir_data: Bytes,
}

impl ExtendedControllerInformationChangedEvent {
    pub fn new(controller_index: ControlIndex, eir_data: Bytes) -> Self {
        Self {
            controller_index,
            eir_data,
        }
    }

    pub fn controller_index(&self) -> ControlIndex {
        self.controller_index.clone()
    }

    pub fn eir_data(&self) -> Bytes {
        self.eir_data.clone()
    }
}

impl EventItem for ExtendedControllerInformationChangedEvent {
    const CODE: Code = Code(0x0025);

    fn with_controller_index(mut self, idx: ControlIndex) -> Self {
        self.controller_index = idx;
        self
    }
}

impl PacketData for ExtendedControllerInformationChangedEvent {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let len = u16::unpack(buf)? as usize;
        if buf.remaining() < len {
            return Err(UnpackError::UnexpectedEof);
        }
        let eir_data = buf.take(len).to_bytes();
        Ok(Self {
            controller_index: Default::default(),
            eir_data,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        (self.eir_data.len() as u16).pack(buf)?;
        if buf.remaining_mut() < self.eir_data.len() {
            return Err(PackError::InsufficientBufLength);
        }
        buf.put(self.eir_data.as_ref());
        Ok(())
    }
}

impl From<ExtendedControllerInformationChangedEvent> for MgmtEvent {
    fn from(v: ExtendedControllerInformationChangedEvent) -> Self {
        Self::ExtendedControllerInformationChangedEvent(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e =
            ExtendedControllerInformationChangedEvent::new(Default::default(), Bytes::from("ok"));
        e.pack(&mut b).unwrap();
        let r = ExtendedControllerInformationChangedEvent::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
