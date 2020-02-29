use bytes::{Buf, BufMut};

use super::Address;
use super::IdentityResolvingKey;
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct NewIdentityResolvingKeyEvent {
    store_hint: bool,
    random_address: Address,
    key: IdentityResolvingKey,
}

impl NewIdentityResolvingKeyEvent {
    pub fn new(store_hint: bool, random_address: Address, key: IdentityResolvingKey) -> Self {
        Self {
            store_hint,
            random_address,
            key,
        }
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

    fn into_mgmt(self, index: ControlIndex) -> MgmtEvent {
        MgmtEvent::NewIdentityResolvingKeyEvent(index, self)
    }
}

impl PacketData for NewIdentityResolvingKeyEvent {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let store_hint = u8::unpack(buf)? != 0;
        let random_address = PacketData::unpack(buf)?;
        let key = PacketData::unpack(buf)?;
        Ok(Self {
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

#[cfg(test)]
mod tests {
    use super::super::AddressType;
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = NewIdentityResolvingKeyEvent::new(
            true,
            "00:11:22:33:44:55".parse().unwrap(),
            IdentityResolvingKey::new(
                "00:11:22:33:44:55".parse().unwrap(),
                AddressType::LeRandom,
                [1; 16],
            ),
        );
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtEvent::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
