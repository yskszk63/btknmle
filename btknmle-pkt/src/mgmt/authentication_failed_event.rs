use bytes::{Buf, BufMut};

use super::{Address, AddressType};
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct AuthenticationFailedEvent {
    controller_index: ControlIndex,
    address: Address,
    address_type: AddressType,
    status: u8,
}

impl AuthenticationFailedEvent {
    pub fn new(
        controller_index: ControlIndex,
        address: Address,
        address_type: AddressType,
        status: u8,
    ) -> Self {
        Self {
            controller_index,
            address,
            address_type,
            status,
        }
    }

    pub fn controller_index(&self) -> ControlIndex {
        self.controller_index.clone()
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

impl EventItem for AuthenticationFailedEvent {
    const CODE: Code = Code(0x0011);

    fn with_controller_index(mut self, idx: ControlIndex) -> Self {
        self.controller_index = idx;
        self
    }
}

impl PacketData for AuthenticationFailedEvent {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let address = PacketData::unpack(buf)?;
        let address_type = PacketData::unpack(buf)?;
        let status = PacketData::unpack(buf)?;
        Ok(Self {
            controller_index: Default::default(),
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

impl From<AuthenticationFailedEvent> for MgmtEvent {
    fn from(v: AuthenticationFailedEvent) -> Self {
        Self::AuthenticationFailedEvent(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = AuthenticationFailedEvent::new(
            Default::default(),
            "00:11:22:33:44:55".parse().unwrap(),
            AddressType::LeRandom,
            3,
        );
        e.pack(&mut b).unwrap();
        let r = AuthenticationFailedEvent::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
