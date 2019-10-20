use std::fmt;

use super::{Codec, CodecError, Event, EventItem};
use bytes::{Buf, BufMut as _, Bytes, BytesMut};

pub struct CmdComplete {
    ncmd: u8,
    opcode: u16,
    data: Bytes,
}

impl CmdComplete {
    pub fn ncmd(&self) -> u8 {
        self.ncmd
    }

    pub fn opcode(&self) -> u16 {
        self.opcode
    }

    pub fn data(&self) -> Bytes {
        self.data.clone()
    }
}

impl fmt::Debug for CmdComplete {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CmdComplete(ncmd={}, opcode=0x{:04X}, data={:?})",
            self.ncmd, self.opcode, self.data
        )
    }
}

impl EventItem for CmdComplete {
    const ID: u8 = 0x0E;
}

impl Codec for CmdComplete {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        if buf.remaining() < 4 {
            return Err(CodecError::Underflow);
        }

        let ncmd = buf.get_u8();
        let opcode = buf.get_u16_le();
        let plen = buf.get_u8() as usize;
        if buf.remaining() < plen {
            return Err(CodecError::Underflow);
        }
        let data = buf.take(plen).iter().collect();
        Ok(CmdComplete { ncmd, opcode, data })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        buf.reserve(4);

        buf.put_u8(self.ncmd);
        buf.put_u16_le(self.opcode);
        buf.put_u8(self.data.len() as u8);
        buf.put(&self.data);
        Ok(())
    }
}

impl From<CmdComplete> for Event {
    fn from(v: CmdComplete) -> Self {
        Self::CmdComplete(v)
    }
}
