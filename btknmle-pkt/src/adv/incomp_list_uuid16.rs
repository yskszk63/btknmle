use bytes::{Buf, BufMut as _, Bytes, BytesMut};

use super::{Advertise, AdvItem, Codec, Result};
use crate::att::Uuid16; // FIXME

#[derive(Debug)]
pub struct IncompleteListUuid16(Vec<Uuid16>);

impl IncompleteListUuid16 {
    pub fn new(v: impl IntoIterator<Item=impl Into<Uuid16>>) -> Self {
        Self(v.into_iter().map(|v| v.into()).collect())
    }
}

impl AdvItem for IncompleteListUuid16 {
    const TYPE: u8 = 0x02;
}

impl Codec for IncompleteListUuid16 {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let mut v = vec![];
        while buf.has_remaining() {
            let b = buf.get_u16_le();
            v.push(Uuid16::from(b));
        }
        Ok(Self(v))
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        for v in &self.0 {
            buf.put(Bytes::from(v.clone()));
        }
        Ok(())
    }
}

impl From<IncompleteListUuid16> for Advertise {
    fn from(v: IncompleteListUuid16) -> Self {
        Self::IncompleteListUuid16(v)
    }
}
