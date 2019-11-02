use bytes::{Buf, BytesMut};

use super::{Codec, CodecError, Result};

#[derive(Debug, Clone)]
pub enum AddressType {
    BrEdr,
    LePublic,
    LeRandom,
}

impl Codec for AddressType {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        Ok(match buf.get_u8() {
            0 => Self::BrEdr,
            1 => Self::LePublic,
            2 => Self::LeRandom,
            _ => return Err(CodecError::Invalid),
        })
    }

    fn write_to(&self, _buf: &mut BytesMut) -> Result<()> {
        unimplemented!()
    }
}
