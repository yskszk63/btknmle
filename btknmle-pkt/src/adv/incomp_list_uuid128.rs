use bytes::{Buf, BufMut as _, Bytes, BytesMut};

use super::{AdvItem, Advertise, Codec, Result};
use crate::att::Uuid128; // FIXME

#[derive(Debug)]
pub struct IncompleteListUuid128(Vec<Uuid128>);

impl IncompleteListUuid128 {
    pub fn new(v: impl IntoIterator<Item = impl Into<Uuid128>>) -> Self {
        Self(v.into_iter().map(|v| v.into()).collect())
    }
}

impl AdvItem for IncompleteListUuid128 {
    const TYPE: u8 = 0x06;
}

impl Codec for IncompleteListUuid128 {
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

impl From<IncompleteListUuid128> for Advertise {
    fn from(v: IncompleteListUuid128) -> Self {
        Self::IncompleteListUuid128(v)
    }
}
