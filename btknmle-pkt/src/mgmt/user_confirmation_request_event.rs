use bytes::buf::BufExt;
use bytes::{Buf, Bytes, BytesMut};

use super::{Address, AddressType};
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use super::{Codec, Result};

#[derive(Debug)]
pub struct UserConfirmationRequestEvent {
    controller_index: ControlIndex,
    address: Address,
    address_type: AddressType,
    confirm_hint: bool,
    value: Bytes,
}

impl UserConfirmationRequestEvent {
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

    pub fn value(&self) -> Bytes {
        self.value.clone()
    }
}

impl EventItem for UserConfirmationRequestEvent {
    const CODE: Code = Code(0x000F);

    fn with_controller_index(mut self, idx: ControlIndex) -> Self {
        self.controller_index = idx;
        self
    }
}

impl Codec for UserConfirmationRequestEvent {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let controller_index = Default::default();
        let address = Address::parse(buf)?;
        let address_type = AddressType::parse(buf)?;
        let confirm_hint = buf.get_u8() != 0;
        let value = buf.take(4).to_bytes();
        Ok(Self {
            controller_index,
            address,
            address_type,
            confirm_hint,
            value,
        })
    }

    fn write_to(&self, _buf: &mut BytesMut) -> Result<()> {
        unimplemented!()
    }
}

impl From<UserConfirmationRequestEvent> for MgmtEvent {
    fn from(v: UserConfirmationRequestEvent) -> Self {
        Self::UserConfirmationRequestEvent(v)
    }
}
