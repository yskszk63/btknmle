use std::fmt;

use super::{Codec, CodecError, Event, EventItem};
use bytes::{Buf, BufMut as _, BytesMut};

pub struct DisconnComplete {
    status: u8,
    handle: u16,
    reason: u8,
}

impl fmt::Debug for DisconnComplete {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DisconnComplete(status={}, handle=0x{:04X}, reason={})",
            self.status, self.handle, self.reason
        )
    }
}

impl EventItem for DisconnComplete {
    const ID: u8 = 0x05;
}

impl Codec for DisconnComplete {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        if buf.remaining() < 4 {
            return Err(CodecError::Underflow);
        }

        let status = buf.get_u8();
        let handle = buf.get_u16_le();
        let reason = buf.get_u8();
        Ok(Self {
            status,
            handle,
            reason,
        })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        buf.reserve(4);

        buf.put_u8(self.status);
        buf.put_u16_le(self.handle);
        buf.put_u8(self.reason);
        Ok(())
    }
}

impl From<DisconnComplete> for Event {
    fn from(v: DisconnComplete) -> Self {
        Self::DisconnComplete(v)
    }
}
