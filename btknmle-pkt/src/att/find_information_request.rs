use bytes::{Buf, BytesMut};

use super::{Att, AttItem, Codec, CodecError, Handle};

#[derive(Debug)]
pub struct FindInformationRequest {
    starting_handle: Handle,
    ending_handle: Handle,
}

impl FindInformationRequest {
    pub fn starting_handle(&self) -> Handle {
        self.starting_handle.clone()
    }

    pub fn ending_handle(&self) -> Handle {
        self.ending_handle.clone()
    }
}

impl AttItem for FindInformationRequest {
    const OPCODE: u8 = 0x04;
}

impl Codec for FindInformationRequest {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        let starting_handle = Handle::parse(buf)?;
        let ending_handle = Handle::parse(buf)?;

        Ok(Self {
            starting_handle,
            ending_handle,
        })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        self.starting_handle.write_to(buf)?;
        self.ending_handle.write_to(buf)?;

        Ok(())
    }
}

impl From<FindInformationRequest> for Att {
    fn from(v: FindInformationRequest) -> Att {
        Att::FindInformationRequest(v)
    }
}
