use super::{Codec, CodecError, Event, EventItem};
use bytes::{Buf, BufMut as _, BytesMut};

#[derive(Debug)]
pub struct NumCompPkts {
    num_hndl: u8,
}

impl EventItem for NumCompPkts {
    const ID: u8 = 0x13;
}

impl Codec for NumCompPkts {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        if buf.remaining() < 1 {
            return Err(CodecError::Underflow);
        }

        let num_hndl = buf.get_u8();
        Ok(Self { num_hndl })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        buf.reserve(1);

        buf.put_u8(self.num_hndl);
        Ok(())
    }
}

impl From<NumCompPkts> for Event {
    fn from(v: NumCompPkts) -> Self {
        Self::NumCompPkts(v)
    }
}
