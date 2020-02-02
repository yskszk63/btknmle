use std::fmt;

use bytes::{Buf, BytesMut};

use super::{Address, AddressType};
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use super::{Codec, Result};

pub struct Passkey(u32);

impl fmt::Display for Passkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:06}", self.0)
    }
}

impl fmt::Debug for Passkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub struct PasskeyNotifyEvent {
    controller_index: ControlIndex,
    address: Address,
    address_type: AddressType,
    passkey: Passkey,
    entered: bool,
}

impl PasskeyNotifyEvent {
    pub fn controller_index(&self) -> ControlIndex {
        self.controller_index.clone()
    }

    pub fn address(&self) -> Address {
        self.address.clone()
    }

    pub fn address_type(&self) -> AddressType {
        self.address_type.clone()
    }

    pub fn passkey(&self) -> &Passkey {
        &self.passkey
    }

    pub fn entered(&self) -> bool {
        self.entered
    }
}

impl EventItem for PasskeyNotifyEvent {
    const CODE: Code = Code(0x0017);

    fn with_controller_index(mut self, idx: ControlIndex) -> Self {
        self.controller_index = idx;
        self
    }
}

impl Codec for PasskeyNotifyEvent {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let controller_index = Default::default();
        let address = Address::parse(buf)?;
        let address_type = AddressType::parse(buf)?;
        let passkey = Passkey(buf.get_u32_le());
        let entered = buf.get_u8() != 0;
        Ok(Self {
            controller_index,
            address,
            address_type,
            passkey,
            entered,
        })
    }

    fn write_to(&self, _buf: &mut BytesMut) -> Result<()> {
        unimplemented!()
    }
}

impl From<PasskeyNotifyEvent> for MgmtEvent {
    fn from(v: PasskeyNotifyEvent) -> Self {
        Self::PasskeyNotifyEvent(v)
    }
}
