use bytes::{Buf, BufMut};

use super::{Code, ControlIndex, EventItem, MgmtEvent};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct AdvertisingAddedEvent {
    instance: u8,
}

impl AdvertisingAddedEvent {
    pub fn new(instance: u8) -> Self {
        Self { instance }
    }

    pub fn instance(&self) -> u8 {
        self.instance
    }
}

impl EventItem for AdvertisingAddedEvent {
    const CODE: Code = Code(0x0023);

    fn into_mgmt(self, index: ControlIndex) -> MgmtEvent {
        MgmtEvent::AdvertisingAddedEvent(index, self)
    }
}

impl PacketData for AdvertisingAddedEvent {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let instance = PacketData::unpack(buf)?;

        Ok(Self { instance })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.instance.pack(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = AdvertisingAddedEvent::new(1);
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtEvent::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
