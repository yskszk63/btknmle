use super::{Codec, CodecError};
use bytes::{Buf, BufMut as _, BytesMut};

pub use cmd_complete::*;
pub use disconn_complete::*;
pub use num_comp_pkts::*;

mod cmd_complete;
mod disconn_complete;
mod num_comp_pkts;

trait EventItem: Codec + Into<Event> {
    const ID: u8;
}

#[derive(Debug)]
pub enum Event {
    CmdComplete(CmdComplete),
    DisconnComplete(DisconnComplete),
    NumCompPkts(NumCompPkts),
}

impl Codec for Event {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        if buf.remaining() < 2 {
            return Err(CodecError::Underflow);
        }

        let evt = buf.get_u8();
        let plen = buf.get_u8() as usize;
        if buf.remaining() < plen {
            return Err(CodecError::Underflow);
        }

        Ok(match evt {
            CmdComplete::ID => CmdComplete::parse(buf)?.into(),
            DisconnComplete::ID => DisconnComplete::parse(buf)?.into(),
            NumCompPkts::ID => NumCompPkts::parse(buf)?.into(),
            x => return Err(CodecError::UnknownEvent(x)),
        })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        let evt = match self {
            Self::CmdComplete(..) => CmdComplete::ID,
            Self::DisconnComplete(..) => DisconnComplete::ID,
            Self::NumCompPkts(..) => NumCompPkts::ID,
        };

        let mut b = BytesMut::new();
        match self {
            Self::CmdComplete(item) => item.write_to(&mut b)?,
            Self::DisconnComplete(item) => item.write_to(&mut b)?,
            Self::NumCompPkts(item) => item.write_to(&mut b)?,
        }
        buf.reserve(b.len() + 2);
        buf.put_u8(evt);
        buf.put_u8(b.len() as u8);
        buf.put(b);
        Ok(())
    }
}
