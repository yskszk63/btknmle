use bytes::{Buf, BufMut as _, Bytes, BytesMut};

use super::{AdvItem, Advertise, Codec, Result};
use crate::att::Uuid128; // FIXME

#[derive(Debug)]
pub struct CompleteListUuid128(Vec<Uuid128>);

impl CompleteListUuid128 {
    pub fn new(v: impl IntoIterator<Item = impl Into<Uuid128>>) -> Self {
        Self(v.into_iter().map(|v| v.into()).collect())
    }
}

impl AdvItem for CompleteListUuid128 {
    const TYPE: u8 = 0x07;
}

impl Codec for CompleteListUuid128 {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let mut v = vec![];
        while buf.has_remaining() {
            let b = buf.get_u128_le();
            v.push(Uuid128::from(b));
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

impl From<CompleteListUuid128> for Advertise {
    fn from(v: CompleteListUuid128) -> Self {
        Self::CompleteListUuid128(v)
    }
}
