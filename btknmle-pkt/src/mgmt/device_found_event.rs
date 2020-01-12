use bytes::buf::BufExt as _;
use bytes::{Buf, Bytes, BytesMut};

use super::{Address, AddressType};
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use super::{Codec, Result};

#[derive(Debug)]
pub struct DeviceFoundEvent {
    controller_index: ControlIndex,
    address: Address,
    address_type: AddressType,
    rssi: u8,
    flags: u32,
    eir_data: Bytes,
}

impl DeviceFoundEvent {
    pub fn controller_index(&self) -> ControlIndex {
        self.controller_index.clone()
    }

    pub fn address(&self) -> Address {
        self.address.clone()
    }

    pub fn address_type(&self) -> AddressType {
        self.address_type.clone()
    }

    pub fn rssi(&self) -> u8 {
        self.rssi
    }

    pub fn flags(&self) -> u32 {
        self.flags
    }

    pub fn eir_data(&self) -> Bytes {
        self.eir_data.clone()
    }
}

impl EventItem for DeviceFoundEvent {
    const CODE: Code = Code(0x0012);

    fn with_controller_index(mut self, idx: ControlIndex) -> Self {
        self.controller_index = idx;
        self
    }
}

impl Codec for DeviceFoundEvent {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let controller_index = Default::default();
        let address = Address::parse(buf)?;
        let address_type = AddressType::parse(buf)?;
        let rssi = buf.get_u8();
        let flags = buf.get_u32_le();
        let len = buf.get_u16_le() as usize;
        let eir_data = buf.take(len).to_bytes();
        Ok(Self {
            controller_index,
            address,
            address_type,
            rssi,
            flags,
            eir_data,
        })
    }

    fn write_to(&self, _buf: &mut BytesMut) -> Result<()> {
        unimplemented!()
    }
}

impl From<DeviceFoundEvent> for MgmtEvent {
    fn from(v: DeviceFoundEvent) -> Self {
        Self::DeviceFoundEvent(v)
    }
}
