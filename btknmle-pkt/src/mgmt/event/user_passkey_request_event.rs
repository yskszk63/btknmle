use bytes::{Buf, BufMut};

use super::{Address, AddressType};
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct UserPasskeyRequestEvent {
    controller_index: ControlIndex,
    address: Address,
    address_type: AddressType,
}

impl UserPasskeyRequestEvent {
    pub fn new(
        controller_index: ControlIndex,
        address: Address,
        address_type: AddressType,
    ) -> Self {
        Self {
            controller_index,
            address,
            address_type,
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
}

impl EventItem for UserPasskeyRequestEvent {
    const CODE: Code = Code(0x0010);

    fn with_controller_index(mut self, idx: ControlIndex) -> Self {
        self.controller_index = idx;
        self
    }
}

impl PacketData for UserPasskeyRequestEvent {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let address = PacketData::unpack(buf)?;
        let address_type = PacketData::unpack(buf)?;
        Ok(Self {
            controller_index: Default::default(),
            address,
            address_type,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.address.pack(buf)?;
        self.address_type.pack(buf)
    }
}

impl From<UserPasskeyRequestEvent> for MgmtEvent {
    fn from(v: UserPasskeyRequestEvent) -> Self {
        Self::UserPasskeyRequestEvent(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = UserPasskeyRequestEvent::new(
            Default::default(),
            "00:11:22:33:44:55".parse().unwrap(),
            AddressType::LeRandom,
        );
        e.pack(&mut b).unwrap();
        let r = UserPasskeyRequestEvent::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
