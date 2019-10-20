use bitflags::bitflags;
use bytes::{Buf, BufMut as _, BytesMut};

use super::{AdvItem, Advertise, Codec, Result};

bitflags! {
    pub struct Flags: u8 {
        const LE_LIMITED_DISCOVERABLE_MODE = 0b0000_0001;
        const LE_GENERAL_DISCOVERABLE_MODE = 0b0000_0010;
        const BR_EDR_NOT_SUPPORTED = 0b0000_0100;
        const SIMULATANEOUS_LE_AND_BR_EDR_TO_SAME_DEVICE_CAPABLE_CONTROLLER = 0b0000_1000;
        const SIMULATANEOUS_LE_AND_BR_EDR_TO_SAME_DEVICE_CAPABLE_HOST = 0b0001_0000;
    }
}

impl AdvItem for Flags {
    const TYPE: u8 = 0x01;
}

impl Codec for Flags {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        Ok(Self::from_bits_truncate(buf.get_u8()))
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        buf.put_u8(self.bits());
        Ok(())
    }
}

impl From<Flags> for Advertise {
    fn from(v: Flags) -> Self {
        Self::Flags(v)
    }
}
