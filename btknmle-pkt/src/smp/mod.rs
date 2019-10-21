use bytes::{Buf, BufMut as _, BytesMut};

use crate::{Codec, CodecError, Result};

pub use pairing_confirm::*;
pub use pairing_failed::*;
pub use pairing_request::*;
pub use pairing_response::*;

mod pairing_confirm;
mod pairing_failed;
mod pairing_request;
mod pairing_response;

trait SmpItem: Codec + Into<Smp> {
    const CODE: u8;
}

#[derive(Debug)]
pub enum Smp {
    PairingRequest(PairingRequest),
    PairingResponse(PairingResponse),
    PairingConfirm(PairingConfirm),
    PairingFailed(PairingFailed),
}

impl Codec for Smp {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        if !buf.has_remaining() {
            return Err(CodecError::Underflow);
        }

        let code = buf.get_u8();
        let result = match code {
            PairingRequest::CODE => PairingRequest::parse(buf)?.into(),
            PairingResponse::CODE => PairingResponse::parse(buf)?.into(),
            PairingConfirm::CODE => PairingConfirm::parse(buf)?.into(),
            PairingFailed::CODE => PairingFailed::parse(buf)?.into(),
            x => return Err(CodecError::UnknownPkt(x)),
        };
        Ok(result)
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        let code = match self {
            Self::PairingRequest(..) => PairingRequest::CODE,
            Self::PairingResponse(..) => PairingResponse::CODE,
            Self::PairingConfirm(..) => PairingConfirm::CODE,
            Self::PairingFailed(..) => PairingFailed::CODE,
        };
        buf.put_u8(code);

        match self {
            Self::PairingRequest(v) => v.write_to(buf)?,
            Self::PairingResponse(v) => v.write_to(buf)?,
            Self::PairingConfirm(v) => v.write_to(buf)?,
            Self::PairingFailed(v) => v.write_to(buf)?,
        };
        Ok(())
    }
}
