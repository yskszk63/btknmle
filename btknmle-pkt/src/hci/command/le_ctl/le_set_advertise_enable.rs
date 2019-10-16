use bytes::{Buf, BufMut as _, BytesMut};

use super::{Codec, CodecError, Command, CommandItem, OGF};

#[derive(Debug)]
pub struct LeSetAdvertiseEnable(bool);

impl LeSetAdvertiseEnable {
    pub fn new(v: bool) -> Self {
        Self(v)
    }
}

impl Codec for LeSetAdvertiseEnable {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        if !buf.has_remaining() {
            return Err(CodecError::Underflow);
        }

        let v = buf.get_u8();
        let v = v != 0;
        Ok(LeSetAdvertiseEnable(v))
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        buf.reserve(1);
        buf.put_u8(if self.0 { 1 } else { 0 });
        Ok(())
    }
}

impl CommandItem for LeSetAdvertiseEnable {
    const OPCODE: (u8, u16) = (OGF, 0x000A);
}

impl From<LeSetAdvertiseEnable> for Command {
    fn from(v: LeSetAdvertiseEnable) -> Self {
        Self::LeSetAdvertiseEnable(v)
    }
}
