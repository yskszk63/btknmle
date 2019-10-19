use std::fmt;

use bytes::{Buf, BufMut as _, BytesMut};

use super::{Codec, CodecError, Command, CommandItem, OGF};

pub struct LeSetAdvertisingData {
    len: u8,
    data: [u8; 31],
}

impl fmt::Debug for LeSetAdvertisingData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LeSetAdvertisingData(len={}, data=", self.len)?;
        for d in &self.data {
            write!(f, "{:02X}", d)?;
        }
        write!(f, ")")
    }
}

impl LeSetAdvertisingData {
    pub fn new(len: u8, data: [u8; 31]) -> Self {
        Self { len, data }
    }
}

impl Codec for LeSetAdvertisingData {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        if buf.remaining() < 32 {
            return Err(CodecError::Underflow);
        }

        let len = buf.get_u8();
        let mut data = [0; 31];
        buf.copy_to_slice(&mut data);
        Ok(LeSetAdvertisingData { len, data })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        buf.reserve(32);
        buf.put_u8(self.len);
        buf.put_slice(&self.data);
        Ok(())
    }
}

impl CommandItem for LeSetAdvertisingData {
    const OPCODE: (u8, u16) = (OGF, 0x0008);
}

impl From<LeSetAdvertisingData> for Command {
    fn from(v: LeSetAdvertisingData) -> Self {
        Self::LeSetAdvertisingData(v)
    }
}
