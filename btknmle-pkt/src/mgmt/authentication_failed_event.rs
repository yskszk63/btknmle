use bytes::{Buf, BytesMut};

use super::{Address, AddressType};
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use super::{Codec, Result};

#[derive(Debug)]
pub struct AuthenticationFailedEvent {
    controller_index: ControlIndex,
    address: Address,
    address_type: AddressType,
    status: u8,
}

impl AuthenticationFailedEvent {
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

impl Codec for AuthenticationFailedEvent {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let controller_index = Default::default();
        let address = Address::parse(buf)?;
        let address_type = AddressType::parse(buf)?;
        let status = buf.get_u8();
        Ok(Self {
            controller_index,
            address,
            address_type,
            status,
        })
    }

    fn write_to(&self, _buf: &mut BytesMut) -> Result<()> {
        unimplemented!()
    }
}

impl From<AuthenticationFailedEvent> for MgmtEvent {
    fn from(v: AuthenticationFailedEvent) -> Self {
        Self::AuthenticationFailedEvent(v)
    }
}
