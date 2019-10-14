use bytes::{Buf, BytesMut};

use super::{Codec, CodecError, Att, AttItem, Handle, Uuid};

#[derive(Debug)]
pub struct ReadByGroupTypeRequest {
    starting_handle: Handle,
    ending_handle: Handle,
    attribute_group_type: Uuid,
}

impl ReadByGroupTypeRequest {
    pub fn starting_handle(&self) -> Handle {
        self.starting_handle.clone()
    }

    pub fn ending_handle(&self) -> Handle {
        self.ending_handle.clone()
    }

    pub fn attribute_group_type(&self) -> Uuid {
        self.attribute_group_type.clone()
    }
}

impl AttItem for ReadByGroupTypeRequest {
    const OPCODE: u8 = 0x10;
}

impl Codec for ReadByGroupTypeRequest {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        let starting_handle = Handle::parse(buf)?;
        let ending_handle = Handle::parse(buf)?;
        let attribute_group_type = Uuid::parse(buf)?;

        Ok(Self { starting_handle, ending_handle, attribute_group_type })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        self.starting_handle.write_to(buf)?;
        self.ending_handle.write_to(buf)?;
        self.attribute_group_type.write_to(buf)?;

        Ok(())
    }
}

impl From<ReadByGroupTypeRequest> for Att {
    fn from(v: ReadByGroupTypeRequest) -> Att {
        Att::ReadByGroupTypeRequest(v)
    }
}
