use bytes::{Buf, BufMut as _, BytesMut};

use super::{AdvItem, Advertise, Codec, Result};

#[derive(Debug)]
pub struct TxPower(i8);

impl TxPower {
    pub fn new(v: i8) -> Self {
        Self(v)
    }
}

impl AdvItem for TxPower {
    const TYPE: u8 = 0x0A;
}

impl Codec for TxPower {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        Ok(Self(buf.get_i8()))
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        buf.put_i8(self.0);
        Ok(())
    }
}

impl From<TxPower> for Advertise {
    fn from(v: TxPower) -> Self {
        Self::TxPower(v)
    }
}
