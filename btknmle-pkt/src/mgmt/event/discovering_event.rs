use bytes::{Buf, BufMut};

use super::AddressType;
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct DiscoveringEvent {
    controller_index: ControlIndex,
    address_type: AddressType,
    discovering: bool,
}

impl DiscoveringEvent {
    pub fn new(
        controller_index: ControlIndex,
        address_type: AddressType,
        discovering: bool,
    ) -> Self {
        Self {
            controller_index,
            address_type,
            discovering,
        }
    }

    pub fn controller_index(&self) -> ControlIndex {
        self.controller_index.clone()
    }

    pub fn address_type(&self) -> AddressType {
        self.address_type.clone()
    }

    pub fn discovering(&self) -> bool {
        self.discovering
    }
}

impl EventItem for DiscoveringEvent {
    const CODE: Code = Code(0x0013);

    fn with_controller_index(mut self, idx: ControlIndex) -> Self {
        self.controller_index = idx;
        self
    }
}

impl PacketData for DiscoveringEvent {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let address_type = PacketData::unpack(buf)?;
        let discovering = u8::unpack(buf)? != 0;
        Ok(Self {
            controller_index: Default::default(),
            address_type,
            discovering,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.address_type.pack(buf)?;
        (self.discovering as u8).pack(buf)
    }
}

impl From<DiscoveringEvent> for MgmtEvent {
    fn from(v: DiscoveringEvent) -> Self {
        Self::DiscoveringEvent(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = DiscoveringEvent::new(Default::default(), AddressType::LeRandom, true);
        e.pack(&mut b).unwrap();
        let r = DiscoveringEvent::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
