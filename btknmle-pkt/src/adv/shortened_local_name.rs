use bytes::{Buf, BufMut as _, BytesMut};

use super::{AdvItem, Advertise, Codec, Result};

#[derive(Debug)]
pub struct ShortenedLocalName(String);

impl ShortenedLocalName {
    pub fn new(v: impl Into<String>) -> Self {
        Self(v.into())
    }
}

impl AdvItem for ShortenedLocalName {
    const TYPE: u8 = 0x08;
}

impl Codec for ShortenedLocalName {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let v = String::from_utf8_lossy(buf.take(usize::max_value()).bytes()).to_string();
        Ok(Self(v))
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        buf.put(self.0.as_bytes());
        Ok(())
    }
}

impl From<ShortenedLocalName> for Advertise {
    fn from(v: ShortenedLocalName) -> Self {
        Self::ShortenedLocalName(v)
    }
}
