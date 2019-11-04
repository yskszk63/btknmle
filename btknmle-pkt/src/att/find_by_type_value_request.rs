use bytes::{Buf, BufMut as _, Bytes, BytesMut};

use super::Uuid16;
use super::{Att, AttItem, Codec, CodecError, Handle};

#[derive(Debug)]
pub struct FindByTypeValueRequest {
    starting_handle: Handle,
    ending_handle: Handle,
    attribute_type: Uuid16,
    attribute_value: Bytes,
}

impl FindByTypeValueRequest {
    pub fn starting_handle(&self) -> Handle {
        self.starting_handle.clone()
    }

    pub fn ending_handle(&self) -> Handle {
        self.ending_handle.clone()
    }

    pub fn attribute_type(&self) -> Uuid16 {
        self.attribute_type.clone()
    }

    pub fn attribute_value(&self) -> Bytes {
        self.attribute_value.clone()
    }
}

impl AttItem for FindByTypeValueRequest {
    const OPCODE: u8 = 0x06;
}

impl Codec for FindByTypeValueRequest {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        let starting_handle = Handle::parse(buf)?;
        let ending_handle = Handle::parse(buf)?;
        let attribute_type = Uuid16(buf.get_u16_le());
        let attribute_value = buf.take(usize::max_value()).collect();

        Ok(Self {
            starting_handle,
            ending_handle,
            attribute_type,
            attribute_value,
        })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        self.starting_handle.write_to(buf)?;
        self.ending_handle.write_to(buf)?;
        buf.put_u16_le(self.attribute_type.0);
        buf.extend_from_slice(&self.attribute_value);

        Ok(())
    }
}

impl From<FindByTypeValueRequest> for Att {
    fn from(v: FindByTypeValueRequest) -> Att {
        Att::FindByTypeValueRequest(v)
    }
}
