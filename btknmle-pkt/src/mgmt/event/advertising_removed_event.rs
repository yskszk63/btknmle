use bytes::{Buf, BufMut};

use super::{Code, ControlIndex, EventItem, MgmtEvent};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct AdvertisingRemovedEvent {
    instance: u8,
}

impl AdvertisingRemovedEvent {
    pub fn new(instance: u8) -> Self {
        Self { instance }
    }

    pub fn instance(&self) -> u8 {
        self.instance
    }
}

impl EventItem for AdvertisingRemovedEvent {
    const CODE: Code = Code(0x0024);

    fn into_mgmt(self, index: ControlIndex) -> MgmtEvent {
        MgmtEvent::AdvertisingRemovedEvent(index, self)
    }
}

impl PacketData for AdvertisingRemovedEvent {
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
        let e = AdvertisingRemovedEvent::new(1);
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtEvent::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
