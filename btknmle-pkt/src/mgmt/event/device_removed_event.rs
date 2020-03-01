use bytes::{Buf, BufMut};

use super::{Address, AddressType};
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct DeviceRemovedEvent {
    address: Address,
    address_type: AddressType,
}

impl DeviceRemovedEvent {
    pub fn new(address: Address, address_type: AddressType) -> Self {
        Self {
            address,
            address_type,
        }
    }

    pub fn address(&self) -> Address {
        self.address.clone()
    }

    pub fn address_type(&self) -> AddressType {
        self.address_type.clone()
    }
}

impl EventItem for DeviceRemovedEvent {
    const CODE: Code = Code(0x001B);

    fn into_mgmt(self, index: ControlIndex) -> MgmtEvent {
        MgmtEvent::DeviceRemovedEvent(index, self)
    }
}

impl PacketData for DeviceRemovedEvent {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let address = PacketData::unpack(buf)?;
        let address_type = PacketData::unpack(buf)?;

        Ok(Self {
            address,
            address_type,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.address.pack(buf)?;
        self.address_type.pack(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e =
            DeviceRemovedEvent::new("00:11:22:33:44:55".parse().unwrap(), AddressType::LeRandom);
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtEvent::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
