use bytes::buf::BufExt as _;
use bytes::{Buf, BufMut, Bytes};

use super::{Code, ControlIndex, EventItem, MgmtEvent};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct ExtendedControllerInformationChangedEvent {
    eir_data: Bytes,
}

impl ExtendedControllerInformationChangedEvent {
    pub fn new(eir_data: Bytes) -> Self {
        Self { eir_data }
    }

    pub fn eir_data(&self) -> Bytes {
        self.eir_data.clone()
    }
}

impl EventItem for ExtendedControllerInformationChangedEvent {
    const CODE: Code = Code(0x0025);

    fn into_mgmt(self, index: ControlIndex) -> MgmtEvent {
        MgmtEvent::ExtendedControllerInformationChangedEvent(index, self)
    }
}

impl PacketData for ExtendedControllerInformationChangedEvent {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let len = u16::unpack(buf)? as usize;
        if buf.remaining() < len {
            return Err(UnpackError::UnexpectedEof);
        }
        let eir_data = buf.take(len).to_bytes();
        Ok(Self { eir_data })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = ExtendedControllerInformationChangedEvent::new(Bytes::from("ok"));
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtEvent::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
