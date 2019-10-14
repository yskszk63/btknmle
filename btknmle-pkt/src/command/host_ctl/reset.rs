use bytes::{Buf, BytesMut};

use super::{Codec, CodecError, Command, CommandItem, OGF};

#[derive(Debug)]
pub struct Reset {}

impl Reset {
    pub fn new() -> Self {
        Self {}
    }
}

impl Codec for Reset {
    fn parse(_buf: &mut impl Buf) -> Result<Self, CodecError> {
        Ok(Reset {})
    }

    fn write_to(&self, _buf: &mut BytesMut) -> Result<(), CodecError> {
        Ok(())
    }
}

impl CommandItem for Reset {
    const OPCODE: (u8, u16) = (OGF, 0x0003);
}

impl From<Reset> for Command {
    fn from(v: Reset) -> Self {
        Self::Reset(v)
    }
}
