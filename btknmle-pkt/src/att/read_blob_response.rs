use bytes::{Buf, BufMut as _, BytesMut, Bytes};

use super::{Codec, CodecError, Att, AttItem};

#[derive(Debug)]
pub struct ReadBlobResponse {
    attribute_value: Bytes,
}

impl ReadBlobResponse {
    pub fn new(attribute_value: impl Into<Bytes>) -> Self {
        Self {
            attribute_value: attribute_value.into(),
        }
    }
}

impl AttItem for ReadBlobResponse {
    const OPCODE: u8 = 0x0D;
}

impl Codec for ReadBlobResponse {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        let attribute_value = buf.take(usize::max_value()).collect();
        Ok(Self { attribute_value })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        buf.put(self.attribute_value.clone());
        Ok(())
    }
}

impl From<ReadBlobResponse> for Att {
    fn from(v: ReadBlobResponse) -> Att {
        Att::ReadBlobResponse(v)
    }
}
