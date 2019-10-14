use bytes::{Buf, BytesMut};

use super::{Codec, CodecError, Att, AttItem};

#[derive(Debug)]
pub struct WriteResponse {
}

impl WriteResponse {
    pub fn new() -> Self {
        Self { }
    }
}

impl AttItem for WriteResponse {
    const OPCODE: u8 = 0x13;
}

impl Codec for WriteResponse {
    fn parse(_buf: &mut impl Buf) -> Result<Self, CodecError> {
        Ok(Self { })
    }

    fn write_to(&self, _buf: &mut BytesMut) -> Result<(), CodecError> {
        Ok(())
    }
}

impl From<WriteResponse> for Att {
    fn from(v: WriteResponse) -> Att {
        Att::WriteResponse(v)
    }
}
