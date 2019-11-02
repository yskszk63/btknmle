use bytes::{Buf, BytesMut};

use super::{Address, AddressType};
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use super::{Codec, Result};

#[derive(Debug)]
pub struct UserPasskeyRequestEvent {
    controller_index: ControlIndex,
    address: Address,
    address_type: AddressType,
}

impl UserPasskeyRequestEvent {
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

impl Codec for UserPasskeyRequestEvent {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let controller_index = Default::default();
        let address = Address::parse(buf)?;
        let address_type = AddressType::parse(buf)?;
        Ok(Self {
            controller_index,
            address,
            address_type,
        })
    }

    fn write_to(&self, _buf: &mut BytesMut) -> Result<()> {
        unimplemented!()
    }
}

impl From<UserPasskeyRequestEvent> for MgmtEvent {
    fn from(v: UserPasskeyRequestEvent) -> Self {
        Self::UserPasskeyRequestEvent(v)
    }
}
