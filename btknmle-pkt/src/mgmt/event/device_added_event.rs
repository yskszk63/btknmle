use bytes::{Buf, BufMut};

use super::Action;
use super::{Address, AddressType};
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct DeviceAddedEvent {
    address: Address,
    address_type: AddressType,
    action: Action,
}

impl DeviceAddedEvent {
    pub fn new(address: Address, address_type: AddressType, action: Action) -> Self {
        Self {
            address,
            address_type,
            action,
        }
    }

    pub fn address(&self) -> Address {
        self.address.clone()
    }

    pub fn address_type(&self) -> AddressType {
        self.address_type.clone()
    }

    pub fn action(&self) -> &Action {
        &self.action
    }
}

impl EventItem for DeviceAddedEvent {
    const CODE: Code = Code(0x001A);

    fn into_mgmt(self, index: ControlIndex) -> MgmtEvent {
        MgmtEvent::DeviceAddedEvent(index, self)
    }
}

impl PacketData for DeviceAddedEvent {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let address = PacketData::unpack(buf)?;
        let address_type = PacketData::unpack(buf)?;
        let action = PacketData::unpack(buf)?;

        Ok(Self {
            address,
            address_type,
            action,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.address.pack(buf)?;
        self.address_type.pack(buf)?;
        self.action.pack(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = DeviceAddedEvent::new(
            "00:11:22:33:44:55".parse().unwrap(),
            AddressType::LeRandom,
            Action::AutoConnectRemoteDevice,
        );
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtEvent::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
