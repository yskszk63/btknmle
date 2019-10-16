use bytes::{Buf, BytesMut};

use super::{Att, AttItem, Codec, CodecError, Handle, Uuid};

#[derive(Debug)]
pub struct ReadByTypeRequest {
    starting_handle: Handle,
    ending_handle: Handle,
    attribute_type: Uuid,
}

impl ReadByTypeRequest {
    pub fn starting_handle(&self) -> Handle {
        self.starting_handle.clone()
    }

    pub fn ending_handle(&self) -> Handle {
        self.ending_handle.clone()
    }

    pub fn attribute_type(&self) -> Uuid {
        self.attribute_type.clone()
    }
}

impl AttItem for ReadByTypeRequest {
    const OPCODE: u8 = 0x08;
}

impl Codec for ReadByTypeRequest {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        let starting_handle = Handle::parse(buf)?;
        let ending_handle = Handle::parse(buf)?;
        let attribute_type = Uuid::parse(buf)?;

        Ok(Self {
            starting_handle,
            ending_handle,
            attribute_type,
        })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        self.starting_handle.write_to(buf)?;
        self.ending_handle.write_to(buf)?;
        self.attribute_type.write_to(buf)?;

        Ok(())
    }
}

impl From<ReadByTypeRequest> for Att {
    fn from(v: ReadByTypeRequest) -> Att {
        Att::ReadByTypeRequest(v)
    }
}
