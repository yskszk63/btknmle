use bytes::buf::BufExt as _;
use bytes::{Buf, BufMut, Bytes};

use super::{Address, AddressType};
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct DeviceFoundEvent {
    controller_index: ControlIndex,
    address: Address,
    address_type: AddressType,
    rssi: u8,
    flags: u32,
    eir_data: Bytes,
}

impl DeviceFoundEvent {
    pub fn new(
        controller_index: ControlIndex,
        address: Address,
        address_type: AddressType,
        rssi: u8,
        flags: u32,
        eir_data: Bytes,
    ) -> Self {
        Self {
            controller_index,
            address,
            address_type,
            rssi,
            flags,
            eir_data,
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

impl PacketData for DeviceFoundEvent {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let address = PacketData::unpack(buf)?;
        let address_type = PacketData::unpack(buf)?;
        let rssi = PacketData::unpack(buf)?;
        let flags = PacketData::unpack(buf)?;
        let len = u16::unpack(buf)? as usize;
        if buf.remaining() < len {
            return Err(UnpackError::UnexpectedEof);
        }
        let eir_data = buf.take(len).to_bytes();
        Ok(Self {
            controller_index: Default::default(),
            address,
            address_type,
            rssi,
            flags,
            eir_data,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.address.pack(buf)?;
        self.address_type.pack(buf)?;
        self.rssi.pack(buf)?;
        self.flags.pack(buf)?;
        (self.eir_data.len() as u16).pack(buf)?;
        if buf.remaining_mut() < self.eir_data.len() {
            return Err(PackError::InsufficientBufLength);
        }
        buf.put(self.eir_data.as_ref());
        Ok(())
    }
}

impl From<DeviceFoundEvent> for MgmtEvent {
    fn from(v: DeviceFoundEvent) -> Self {
        Self::DeviceFoundEvent(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = DeviceFoundEvent::new(
            Default::default(),
            "00:11:22:33:44:55".parse().unwrap(),
            AddressType::LeRandom,
            3,
            4,
            Bytes::from("ok"),
        );
        e.pack(&mut b).unwrap();
        let r = DeviceFoundEvent::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
