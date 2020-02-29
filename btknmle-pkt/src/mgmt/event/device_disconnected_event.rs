use bytes::{Buf, BufMut};

use super::{Address, AddressType};
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct DeviceDisconnectedEvent {
    address: Address,
    address_type: AddressType,
    reason: u8,
}

impl DeviceDisconnectedEvent {
    pub fn new(address: Address, address_type: AddressType, reason: u8) -> Self {
        Self {
            address,
            address_type,
            reason,
        }
    }

    pub fn address(&self) -> Address {
        self.address.clone()
    }

    pub fn address_type(&self) -> AddressType {
        self.address_type.clone()
    }

    pub fn reason(&self) -> u8 {
        self.reason
    }
}

impl EventItem for DeviceDisconnectedEvent {
    const CODE: Code = Code(0x000C);

    fn into_mgmt(self, index: ControlIndex) -> MgmtEvent {
        MgmtEvent::DeviceDisconnectedEvent(index, self)
    }
}

impl PacketData for DeviceDisconnectedEvent {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let address = PacketData::unpack(buf)?;
        let address_type = PacketData::unpack(buf)?;
        let reason = PacketData::unpack(buf)?;
        Ok(Self {
            address,
            address_type,
            reason,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.address.pack(buf)?;
        self.address_type.pack(buf)?;
        self.reason.pack(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = DeviceDisconnectedEvent::new(
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
