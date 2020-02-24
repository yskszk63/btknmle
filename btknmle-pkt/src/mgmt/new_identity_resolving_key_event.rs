use bytes::{Buf, BufMut};

use super::Address;
use super::IdentityResolvingKey;
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct NewIdentityResolvingKeyEvent {
    controller_index: ControlIndex,
    store_hint: bool,
    random_address: Address,
    key: IdentityResolvingKey,
}

impl NewIdentityResolvingKeyEvent {
    pub fn new(
        controller_index: ControlIndex,
        store_hint: bool,
        random_address: Address,
        key: IdentityResolvingKey,
    ) -> Self {
        Self {
            controller_index,
            store_hint,
            random_address,
            key,
        }
    }

    pub fn controller_index(&self) -> ControlIndex {
        self.controller_index.clone()
    }

    pub fn store_hint(&self) -> bool {
        self.store_hint
    }

    pub fn random_address(&self) -> Address {
        self.random_address.clone()
    }

    pub fn key(&self) -> &IdentityResolvingKey {
        &self.key
    }
}

impl EventItem for NewIdentityResolvingKeyEvent {
    const CODE: Code = Code(0x0018);

    fn with_controller_index(mut self, idx: ControlIndex) -> Self {
        self.controller_index = idx;
        self
    }
}

impl PacketData for NewIdentityResolvingKeyEvent {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let store_hint = u8::unpack(buf)? != 0;
        let random_address = PacketData::unpack(buf)?;
        let key = PacketData::unpack(buf)?;
        Ok(Self {
            controller_index: Default::default(),
            store_hint,
            random_address,
            key,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        (self.store_hint as u8).pack(buf)?;
        self.random_address.pack(buf)?;
        self.key.pack(buf)
    }
}

impl From<NewIdentityResolvingKeyEvent> for MgmtEvent {
    fn from(v: NewIdentityResolvingKeyEvent) -> Self {
        Self::NewIdentityResolvingKeyEvent(v)
    }
}

#[cfg(test)]
mod tests {
    use super::super::AddressType;
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = NewIdentityResolvingKeyEvent::new(
            Default::default(),
            true,
            "00:11:22:33:44:55".parse().unwrap(),
            IdentityResolvingKey::new(
                "00:11:22:33:44:55".parse().unwrap(),
                AddressType::LeRandom,
                [1; 16],
            ),
        );
        e.pack(&mut b).unwrap();
        let r = NewIdentityResolvingKeyEvent::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
