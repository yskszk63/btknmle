use bytes::{Buf, BufMut as _, Bytes, BytesMut};

use super::{Att, AttItem, Codec, CodecError};

#[derive(Debug)]
pub struct ReadResponse {
    attribute_value: Bytes,
}

impl ReadResponse {
    pub fn new(attribute_value: impl Into<Bytes>) -> Self {
        Self {
            attribute_value: attribute_value.into(),
        }
    }
}

impl AttItem for ReadResponse {
    const OPCODE: u8 = 0x0B;
}

impl Codec for ReadResponse {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        let attribute_value = buf.take(usize::max_value()).collect();
        Ok(Self { attribute_value })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        buf.put(self.attribute_value.clone());
        Ok(())
    }
}

impl From<ReadResponse> for Att {
    fn from(v: ReadResponse) -> Att {
        Att::ReadResponse(v)
    }
}
