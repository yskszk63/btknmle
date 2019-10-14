use bytes::{Buf, BytesMut};

use super::{Codec, CodecError, Att, AttItem, Handle};

#[derive(Debug)]
pub struct ReadRequest {
    attribute_handle: Handle,
}

impl ReadRequest {
    pub fn attribute_handle(&self) -> Handle {
        self.attribute_handle.clone()
    }
}

impl AttItem for ReadRequest {
    const OPCODE: u8 = 0x0A;
}

impl Codec for ReadRequest {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        let attribute_handle = Handle::parse(buf)?;

        Ok(Self { attribute_handle })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        self.attribute_handle.write_to(buf)?;

        Ok(())
    }
}

impl From<ReadRequest> for Att {
    fn from(v: ReadRequest) -> Att {
        Att::ReadRequest(v)
    }
}
