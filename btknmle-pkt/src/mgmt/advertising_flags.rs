use bitflags::bitflags;
use bytes::{Buf, BufMut as _, BytesMut};

use super::{Codec, Result};

bitflags! {
    pub struct AdvertisingFlags: u32 {
        const SWITCH_INTO_CONNECTABLE_MODE = 0b0000_0000_0000_0001;
        const ADVERTISE_AS_DISCOVERABLE = 0b0000_0000_0000_0010;
        const ADVERTISE_AS_LIMITED_DISCOVERABLE = 0b0000_0000_0000_0100;
        const ADD_FLAGS_FIELD_TO_ADV_DATA = 0b0000_0000_0000_1000;
        const ADD_TX_POWER_FIELD_TO_ADV_DATA = 0b0000_0000_0001_0000;
        const ADD_APPEARANCE_FIELD_TO_SCAN_RSP = 0b0000_0000_0010_0000;
        const ADD_LOCAL_NAME_IN_SCAN_RSP = 0b0000_0000_0100_0000;
        const SECONDARY_CHANNEL_WITH_LE_1M = 0b0000_0000_1000_0000;
        const SECONDARY_CHANNEL_WITH_LE_2M = 0b0000_0001_0000_0000;
        const SECONDARY_CHANNEL_WITH_LE_CODED = 0b0000_0010_0000_0000;
    }
}

impl Codec for AdvertisingFlags {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        Ok(Self::from_bits_truncate(buf.get_u32_le()))
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        buf.put_u32_le(self.bits());
        Ok(())
    }
}
