use bytes::{Buf, BufMut};

use crate::{PackError, PacketData, UnpackError};

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

impl PacketData for CurrentSettings {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let b = PacketData::unpack(buf)?;
        Ok(CurrentSettings::from_bits_truncate(b))
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
        for n in 0..=0b1111_1111_1111_1111 {
            let n = u32::to_le_bytes(n).to_vec();
            let b = CurrentSettings::unpack(&mut n.as_ref()).unwrap();
            let mut r = vec![];
            b.pack(&mut r).unwrap();
            assert_eq!(n, r);
        }
    }
}
