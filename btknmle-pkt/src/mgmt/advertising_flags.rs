use bitflags::bitflags;
use bytes::{Buf, BufMut};

use crate::{PackError, PacketData, UnpackError};

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

impl PacketData for AdvertisingFlags {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        Ok(Self::from_bits_truncate(u32::unpack(buf)?))
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.bits().pack(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        for n in 0..=0b0000_0011_1111_1111 {
            let n = u32::to_le_bytes(n).to_vec();
            let b = AdvertisingFlags::unpack(&mut n.as_ref()).unwrap();
            let mut r = vec![];
            b.pack(&mut r).unwrap();
            assert_eq!(n, r);
        }
    }
}
