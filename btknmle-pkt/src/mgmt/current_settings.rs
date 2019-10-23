use bytes::{Buf, BufMut as _, BytesMut};

use super::{Codec, Result};

bitflags::bitflags! {
    pub struct CurrentSettings: u32 {
        const POWERED = 0b0000_0000_0000_0001;
        const CONNECTABLE = 0b0000_0000_0000_0010;
        const FAST_CONNECTABLE = 0b0000_0000_0000_0100;
        const DISCOVERABLE = 0b0000_0000_0000_1000;
        const BONDABLE = 0b0000_0000_0001_0000;
        const LINK_LEVEL_SECURITY = 0b0000_0000_0010_0000;
        const SECURE_SIMPLE_PAIRING = 0b0000_0000_0100_0000;
        const BASIC_RATE_ENHANCED_DATA_RATE = 0b0000_0000_1000_0000;
        const HIGH_SPEED = 0b0000_0001_0000_0000;
        const LOW_ENERGY = 0b0000_0010_0000_0000;
        const ADVERTISING = 0b0000_0100_0000_0000;
        const SECURE_CONNECTION = 0b0000_1000_0000_0000;
        const DEBUG_KEYS = 0b0001_0000_0000_0000;
        const PRIVACY = 0b0010_0000_0000_0000;
        const CONTROLLER_CONFIGURATION = 0b0100_0000_0000_0000;
        const STATIC_ADDRESS = 0b1000_0000_0000_0000;
    }
}

impl Codec for CurrentSettings {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let b = buf.get_u32_le();
        Ok(CurrentSettings::from_bits_truncate(b))
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        buf.put_u32_le(self.bits());
        Ok(())
    }
}
