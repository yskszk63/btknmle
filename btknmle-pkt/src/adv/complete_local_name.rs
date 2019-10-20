use bytes::{Buf, BufMut as _, BytesMut};

use super::{AdvItem, Advertise, Codec, Result};

#[derive(Debug)]
pub struct CompleteLocalName(String);

impl CompleteLocalName {
    pub fn new(v: impl Into<String>) -> Self {
        Self(v.into())
    }
}

impl AdvItem for CompleteLocalName {
    const TYPE: u8 = 0x09;
}

impl Codec for CompleteLocalName {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let v = String::from_utf8_lossy(buf.take(usize::max_value()).bytes()).to_string();
        Ok(Self(v))
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        buf.put(self.0.as_bytes());
        Ok(())
    }
}

impl From<CompleteLocalName> for Advertise {
    fn from(v: CompleteLocalName) -> Self {
        Self::CompleteLocalName(v)
    }
}
