use bytes::{Buf, BytesMut};

use super::Address;
use super::IdentityResolvingKey;
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use super::{Codec, Result};

#[derive(Debug)]
pub struct NewIdentityResolvingKeyEvent {
    controller_index: ControlIndex,
    store_hint: bool,
    random_address: Address,
    key: IdentityResolvingKey,
}

impl NewIdentityResolvingKeyEvent {
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

impl Codec for NewIdentityResolvingKeyEvent {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let controller_index = Default::default();
        let store_hint = buf.get_u8() != 0;
        let random_address = Address::parse(buf)?;
        let key = IdentityResolvingKey::parse(buf)?;
        Ok(Self {
            controller_index,
            store_hint,
            random_address,
            key,
        })
    }

    fn write_to(&self, _buf: &mut BytesMut) -> Result<()> {
        unimplemented!()
    }
}

impl From<NewIdentityResolvingKeyEvent> for MgmtEvent {
    fn from(v: NewIdentityResolvingKeyEvent) -> Self {
        Self::NewIdentityResolvingKeyEvent(v)
    }
}
