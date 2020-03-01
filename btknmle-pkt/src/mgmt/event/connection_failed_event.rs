use bytes::{Buf, BufMut};

use super::{Address, AddressType};
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct ConnectionFailedEvent {
    address: Address,
    address_type: AddressType,
    status: u8,
}

impl ConnectionFailedEvent {
    pub fn new(address: Address, address_type: AddressType, status: u8) -> Self {
        Self {
            address,
            address_type,
            status,
        }
    }

    pub fn address(&self) -> Address {
        self.address.clone()
    }

    pub fn address_type(&self) -> AddressType {
        self.address_type.clone()
    }

    pub fn status(&self) -> u8 {
        self.status
    }
}

impl EventItem for ConnectionFailedEvent {
    const CODE: Code = Code(0x000D);

    fn into_mgmt(self, index: ControlIndex) -> MgmtEvent {
        MgmtEvent::ConnectionFailedEvent(index, self)
    }
}

impl PacketData for ConnectionFailedEvent {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let address = PacketData::unpack(buf)?;
        let address_type = PacketData::unpack(buf)?;
        let status = PacketData::unpack(buf)?;
        Ok(Self {
            address,
            address_type,
            status,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.address.pack(buf)?;
        self.address_type.pack(buf)?;
        self.status.pack(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = ConnectionFailedEvent::new(
            "00:11:22:33:44:55".parse().unwrap(),
            AddressType::LeRandom,
            3,
        );
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtEvent::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
