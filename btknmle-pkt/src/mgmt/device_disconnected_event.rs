use bytes::{Buf, BytesMut};

use super::{Address, AddressType};
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use super::{Codec, Result};

#[derive(Debug)]
pub struct DeviceDisconnectedEvent {
    controller_index: ControlIndex,
    address: Address,
    address_type: AddressType,
    reason: u8,
}

impl DeviceDisconnectedEvent {
    pub fn controller_index(&self) -> ControlIndex {
        self.controller_index.clone()
    }

    pub fn address(&self) -> Address {
        self.address.clone()
    }

    pub fn address_type(&self) -> AddressType {
        self.address_type.clone()
    }

    pub fn reason(&self) -> u8 {
        self.reason
    }
}

impl EventItem for DeviceDisconnectedEvent {
    const CODE: Code = Code(0x000C);

    fn with_controller_index(mut self, idx: ControlIndex) -> Self {
        self.controller_index = idx;
        self
    }
}

impl Codec for DeviceDisconnectedEvent {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let controller_index = Default::default();
        let address = Address::parse(buf)?;
        let address_type = AddressType::parse(buf)?;
        let reason = buf.get_u8();
        Ok(Self {
            controller_index,
            address,
            address_type,
            reason,
        })
    }

    fn write_to(&self, _buf: &mut BytesMut) -> Result<()> {
        unimplemented!()
    }
}

impl From<DeviceDisconnectedEvent> for MgmtEvent {
    fn from(v: DeviceDisconnectedEvent) -> Self {
        Self::DeviceDisconnectedEvent(v)
    }
}
