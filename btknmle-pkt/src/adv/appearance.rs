use bytes::{Buf, BufMut as _, BytesMut};

use super::{Advertise, AdvItem, Codec, Result};

#[derive(Debug)]
pub struct Appearance(u16);

impl Appearance {
    pub fn new(v: u16) -> Self {
        Self(v)
    }
}

impl AdvItem for Appearance {
    const TYPE: u8 = 0x19;
}

impl Codec for Appearance {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        Ok(Self(buf.get_u16_le()))
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        buf.put_u16_le(self.0);
        Ok(())
    }
}

impl From<Appearance> for Advertise {
    fn from(v: Appearance) -> Self {
        Self::Appearance(v)
    }
}
