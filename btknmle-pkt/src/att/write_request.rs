use bytes::buf::BufExt as _;
use bytes::{Buf, BufMut as _, Bytes, BytesMut};

use super::{Att, AttItem, Codec, CodecError, Handle};

#[derive(Debug)]
pub struct WriteRequest {
    attribute_handle: Handle,
    attribute_value: Bytes,
}

impl WriteRequest {
    pub fn attribute_handle(&self) -> Handle {
        self.attribute_handle.clone()
    }
    pub fn attribute_value(&self) -> Bytes {
        self.attribute_value.clone()
    }
}

impl AttItem for WriteRequest {
    const OPCODE: u8 = 0x12;
}

impl Codec for WriteRequest {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        let attribute_handle = Handle::parse(buf)?;
        let attribute_value = buf.take(usize::max_value()).to_bytes();

        Ok(Self {
            attribute_handle,
            attribute_value,
        })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        self.attribute_handle.write_to(buf)?;
        buf.put(self.attribute_value.clone());

        Ok(())
    }
}

impl From<WriteRequest> for Att {
    fn from(v: WriteRequest) -> Att {
        Att::WriteRequest(v)
    }
}
