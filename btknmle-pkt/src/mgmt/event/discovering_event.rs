use bytes::{Buf, BufMut};

use super::AddressType;
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct DiscoveringEvent {
    address_type: AddressType,
    discovering: bool,
}

impl DiscoveringEvent {
    pub fn new(address_type: AddressType, discovering: bool) -> Self {
        Self {
            address_type,
            discovering,
        }
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

    fn into_mgmt(self, index: ControlIndex) -> MgmtEvent {
        MgmtEvent::DiscoveringEvent(index, self)
    }
}

impl PacketData for DiscoveringEvent {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let address_type = PacketData::unpack(buf)?;
        let discovering = u8::unpack(buf)? != 0;
        Ok(Self {
            address_type,
            discovering,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.address_type.pack(buf)?;
        (self.discovering as u8).pack(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = DiscoveringEvent::new(AddressType::LeRandom, true);
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtEvent::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
