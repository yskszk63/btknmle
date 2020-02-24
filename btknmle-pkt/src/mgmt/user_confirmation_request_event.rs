use bytes::{Buf, BufMut};

use super::{Address, AddressType};
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct UserConfirmationRequestEvent {
    controller_index: ControlIndex,
    address: Address,
    address_type: AddressType,
    confirm_hint: bool,
    value: u32,
}

impl UserConfirmationRequestEvent {
    pub fn new(
        controller_index: ControlIndex,
        address: Address,
        address_type: AddressType,
        confirm_hint: bool,
        value: u32,
    ) -> Self {
        Self {
            controller_index,
            address,
            address_type,
            confirm_hint,
            value,
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

    pub fn confirm_hint(&self) -> bool {
        self.confirm_hint
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

impl EventItem for UserConfirmationRequestEvent {
    const CODE: Code = Code(0x000F);

    fn with_controller_index(mut self, idx: ControlIndex) -> Self {
        self.controller_index = idx;
        self
    }
}

impl PacketData for UserConfirmationRequestEvent {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let address = PacketData::unpack(buf)?;
        let address_type = PacketData::unpack(buf)?;
        let confirm_hint = u8::unpack(buf)? != 0;
        let value = PacketData::unpack(buf)?;
        Ok(Self {
            controller_index: Default::default(),
            address,
            address_type,
            confirm_hint,
            value,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.address.pack(buf)?;
        self.address_type.pack(buf)?;
        (self.confirm_hint as u8).pack(buf)?;
        self.value.pack(buf)
    }
}

impl From<UserConfirmationRequestEvent> for MgmtEvent {
    fn from(v: UserConfirmationRequestEvent) -> Self {
        Self::UserConfirmationRequestEvent(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = UserConfirmationRequestEvent::new(
            Default::default(),
            "00:11:22:33:44:55".parse().unwrap(),
            AddressType::LeRandom,
            true,
            3,
        );
        e.pack(&mut b).unwrap();
        let r = UserConfirmationRequestEvent::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
