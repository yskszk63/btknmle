use std::fmt;

use bytes::{Buf, BufMut as _, Bytes, BytesMut};

use super::{Codec, CodecError};

pub use error_response::*;
pub use exchange_mtu_request::*;
pub use exchange_mtu_response::*;
pub use find_information_request::*;
pub use find_information_response::*;
pub use read_by_type_request::*;
pub use read_by_type_response::*;
pub use read_by_group_type_request::*;
pub use read_by_group_type_response::*;
pub use read_request::*;
pub use read_response::*;
pub use read_blob_request::*;
pub use read_blob_response::*;
pub use write_request::*;
pub use write_response::*;
pub use handle_value_notification::*;

mod error_response;
mod exchange_mtu_request;
mod exchange_mtu_response;
mod find_information_request;
mod find_information_response;
mod read_by_type_request;
mod read_by_type_response;
mod read_by_group_type_request;
mod read_by_group_type_response;
mod read_request;
mod read_response;
mod read_blob_request;
mod read_blob_response;
mod write_request;
mod write_response;
mod handle_value_notification;

trait AttItem: Codec + Into<Att> {
    const OPCODE: u8;
}

#[derive(Clone, PartialEq, Eq)]
pub enum Uuid {
    Uuid16(u16),
    Uuid128(u128),
}

impl Codec for Uuid {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        Ok(match buf.remaining() {
            2 => Uuid16(buf.get_u16_le()).into(),
            16 => Uuid128(buf.get_u128_le()).into(),
            _ => return Err(CodecError::Invalid),
        })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        match self {
            Self::Uuid16(v) => buf.put_u16_le(*v),
            Self::Uuid128(v) => buf.put_u128_le(*v),
        }
        Ok(())
    }
}

impl From<Uuid> for Bytes {
    fn from(v: Uuid) -> Self {
        let mut b = BytesMut::new();
        v.write_to(&mut b).unwrap();
        b.freeze()
    }
}

impl fmt::Debug for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uuid16(v) => fmt::Display::fmt(&Uuid16(*v), f),
            Self::Uuid128(v) => fmt::Display::fmt(&Uuid128(*v), f),
        }
    }
}

#[derive(Clone)]
pub struct Uuid16(u16);

impl From<u16> for Uuid16 {
    fn from(v: u16) -> Self {
        Uuid16(v)
    }
}

impl From<Uuid16> for Bytes {
    fn from(v: Uuid16) -> Self {
        let mut b = BytesMut::with_capacity(2);
        b.put_u16_le(v.0);
        b.freeze()
    }
}

impl From<Uuid16> for Uuid {
    fn from(v: Uuid16) -> Self {
        Uuid::Uuid16(v.0)
    }
}

impl fmt::Debug for Uuid16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Uuid16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:X}", self.0)
    }
}

#[derive(Clone)]
pub struct Uuid128(u128);

impl From<u128> for Uuid128 {
    fn from(v: u128) -> Self {
        Uuid128(v)
    }
}

impl From<Uuid128> for Bytes {
    fn from(v: Uuid128) -> Self {
        let mut b = BytesMut::with_capacity(16);
        b.put_u128_le(v.0);
        b.freeze()
    }
}

impl From<Uuid128> for Uuid {
    fn from(v: Uuid128) -> Self {
        Uuid::Uuid128(v.0)
    }
}

impl fmt::Debug for Uuid128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Uuid128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:X}", self.0) // FIXME
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Handle(pub u16);

impl Codec for Handle {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        if buf.remaining() < 2 {
            return Err(CodecError::Underflow)
        }
        Ok(Handle(buf.get_u16_le()))
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        buf.reserve(2);
        buf.put_u16_le(self.0);
        Ok(())
    }
}

impl From<u16> for Handle {
    fn from(v: u16) -> Self {
        Self(v)
    }
}

#[derive(Debug)]
pub enum Att {
    ErrorResponse(ErrorResponse),
    ExchangeMtuRequest(ExchangeMtuRequest),
    ExchangeMtuResponse(ExchangeMtuResponse),
    FindInformationRequest(FindInformationRequest),
    FindInformationResponse(FindInformationResponse),
    ReadByTypeRequest(ReadByTypeRequest),
    ReadByTypeResponse(ReadByTypeResponse),
    ReadByGroupTypeRequest(ReadByGroupTypeRequest),
    ReadByGroupTypeResponse(ReadByGroupTypeResponse),
    ReadRequest(ReadRequest),
    ReadResponse(ReadResponse),
    ReadBlobRequest(ReadBlobRequest),
    ReadBlobResponse(ReadBlobResponse),
    WriteRequest(WriteRequest),
    WriteResponse(WriteResponse),
    HandleValueNotification(HandleValueNotification),
}

impl Codec for Att {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        if !buf.has_remaining() {
            return Err(CodecError::Underflow);
        }

        let opcode = buf.get_u8();
        let result = match opcode {
            ErrorResponse::OPCODE => ErrorResponse::parse(buf)?.into(),
            ExchangeMtuRequest::OPCODE => ExchangeMtuRequest::parse(buf)?.into(),
            ExchangeMtuResponse::OPCODE => ExchangeMtuResponse::parse(buf)?.into(),
            FindInformationRequest::OPCODE => FindInformationRequest::parse(buf)?.into(),
            FindInformationResponse::OPCODE => FindInformationResponse::parse(buf)?.into(),
            ReadByTypeRequest::OPCODE => ReadByTypeRequest::parse(buf)?.into(),
            ReadByTypeResponse::OPCODE => ReadByTypeResponse::parse(buf)?.into(),
            ReadByGroupTypeRequest::OPCODE => ReadByGroupTypeRequest::parse(buf)?.into(),
            ReadByGroupTypeResponse::OPCODE => ReadByGroupTypeResponse::parse(buf)?.into(),
            ReadRequest::OPCODE => ReadRequest::parse(buf)?.into(),
            ReadResponse::OPCODE => ReadResponse::parse(buf)?.into(),
            ReadBlobRequest::OPCODE => ReadBlobRequest::parse(buf)?.into(),
            ReadBlobResponse::OPCODE => ReadBlobResponse::parse(buf)?.into(),
            WriteRequest::OPCODE => WriteRequest::parse(buf)?.into(),
            WriteResponse::OPCODE => WriteResponse::parse(buf)?.into(),
            HandleValueNotification::OPCODE => HandleValueNotification::parse(buf)?.into(),
            x => return Err(CodecError::UnknownAtt(x)),
        };
        Ok(result)
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        match self {
            Att::ErrorResponse(..) => buf.put_u8(ErrorResponse::OPCODE),
            Att::ExchangeMtuRequest(..) => buf.put_u8(ExchangeMtuRequest::OPCODE),
            Att::ExchangeMtuResponse(..) => buf.put_u8(ExchangeMtuResponse::OPCODE),
            Att::FindInformationRequest(..) => buf.put_u8(FindInformationRequest::OPCODE),
            Att::FindInformationResponse(..) => buf.put_u8(FindInformationResponse::OPCODE),
            Att::ReadByTypeRequest(..) => buf.put_u8(ReadByTypeRequest::OPCODE),
            Att::ReadByTypeResponse(..) => buf.put_u8(ReadByTypeResponse::OPCODE),
            Att::ReadByGroupTypeRequest(..) => buf.put_u8(ReadByGroupTypeRequest::OPCODE),
            Att::ReadByGroupTypeResponse(..) => buf.put_u8(ReadByGroupTypeResponse::OPCODE),
            Att::ReadRequest(..) => buf.put_u8(ReadRequest::OPCODE),
            Att::ReadResponse(..) => buf.put_u8(ReadResponse::OPCODE),
            Att::ReadBlobRequest(..) => buf.put_u8(ReadBlobRequest::OPCODE),
            Att::ReadBlobResponse(..) => buf.put_u8(ReadBlobResponse::OPCODE),
            Att::WriteRequest(..) => buf.put_u8(WriteRequest::OPCODE),
            Att::WriteResponse(..) => buf.put_u8(WriteResponse::OPCODE),
            Att::HandleValueNotification(..) => buf.put_u8(HandleValueNotification::OPCODE),
        }

        match self {
            Att::ErrorResponse(item) => item.write_to(buf)?,
            Att::ExchangeMtuRequest(item) => item.write_to(buf)?,
            Att::ExchangeMtuResponse(item) => item.write_to(buf)?,
            Att::FindInformationRequest(item) => item.write_to(buf)?,
            Att::FindInformationResponse(item) => item.write_to(buf)?,
            Att::ReadByTypeRequest(item) => item.write_to(buf)?,
            Att::ReadByTypeResponse(item) => item.write_to(buf)?,
            Att::ReadByGroupTypeRequest(item) => item.write_to(buf)?,
            Att::ReadByGroupTypeResponse(item) => item.write_to(buf)?,
            Att::ReadRequest(item) => item.write_to(buf)?,
            Att::ReadResponse(item) => item.write_to(buf)?,
            Att::ReadBlobRequest(item) => item.write_to(buf)?,
            Att::ReadBlobResponse(item) => item.write_to(buf)?,
            Att::WriteRequest(item) => item.write_to(buf)?,
            Att::WriteResponse(item) => item.write_to(buf)?,
            Att::HandleValueNotification(item) => item.write_to(buf)?,
        }

        Ok(())
    }
}
