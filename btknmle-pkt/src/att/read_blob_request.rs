use bytes::{Buf, BufMut as _, BytesMut};

use super::{Att, AttItem, Codec, CodecError, Handle};

#[derive(Debug)]
pub struct ReadBlobRequest {
    attribute_handle: Handle,
    value_offset: u16,
}

impl ReadBlobRequest {
    pub fn attribute_handle(&self) -> Handle {
        self.attribute_handle.clone()
    }

    pub fn value_offset(&self) -> u16 {
        self.value_offset
    }
}

impl AttItem for ReadBlobRequest {
    const OPCODE: u8 = 0x0C;
}

impl Codec for ReadBlobRequest {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        let attribute_handle = Handle::parse(buf)?;
        let value_offset = buf.get_u16_le();

        Ok(Self {
            attribute_handle,
            value_offset,
        })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        self.attribute_handle.write_to(buf)?;
        buf.put_u16_le(self.value_offset);

        Ok(())
    }
}

impl From<ReadBlobRequest> for Att {
    fn from(v: ReadBlobRequest) -> Att {
        Att::ReadBlobRequest(v)
    }
}
