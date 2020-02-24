use std::fmt;

use bytes::{Buf, BufMut};

use crate::{PackError, PacketData, UnpackError};

pub use error_response::*;
pub use exchange_mtu_request::*;
pub use exchange_mtu_response::*;
pub use find_by_type_value_request::*;
pub use find_by_type_value_response::*;
pub use find_information_request::*;
pub use find_information_response::*;
pub use handle_value_notification::*;
pub use read_blob_request::*;
pub use read_blob_response::*;
pub use read_by_group_type_request::*;
pub use read_by_group_type_response::*;
pub use read_by_type_request::*;
pub use read_by_type_response::*;
pub use read_request::*;
pub use read_response::*;
pub use write_request::*;
pub use write_response::*;

mod error_response;
mod exchange_mtu_request;
mod exchange_mtu_response;
mod find_by_type_value_request;
mod find_by_type_value_response;
mod find_information_request;
mod find_information_response;
mod handle_value_notification;
mod read_blob_request;
mod read_blob_response;
mod read_by_group_type_request;
mod read_by_group_type_response;
mod read_by_type_request;
mod read_by_type_response;
mod read_request;
mod read_response;
mod write_request;
mod write_response;

trait AttItem: PacketData + Into<Att> {
    const OPCODE: u8;

    fn unpack_into(buf: &mut impl Buf) -> Result<Att, UnpackError> {
        let item = Self::unpack(buf)?;
        Ok(item.into())
    }

    fn pack_attr(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        Self::OPCODE.pack(buf)?;
        self.pack(buf)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Handle(u16);

impl PacketData for Handle {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        Ok(u16::unpack(buf)?.into())
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        u16::from(self.clone()).pack(buf)
    }
}

impl From<u16> for Handle {
    fn from(v: u16) -> Self {
        Self(v)
    }
}

impl From<Handle> for u16 {
    fn from(v: Handle) -> Self {
        v.0
    }
}

impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:04X}", self.0)
    }
}

#[derive(PartialEq, Eq)]
pub enum Att {
    ErrorResponse(ErrorResponse),
    ExchangeMtuRequest(ExchangeMtuRequest),
    ExchangeMtuResponse(ExchangeMtuResponse),
    FindInformationRequest(FindInformationRequest),
    FindInformationResponse(FindInformationResponse),
    FindByTypeValueRequest(FindByTypeValueRequest),
    FindByTypeValueResponse(FindByTypeValueResponse),
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

impl PacketData for Att {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let opcode = u8::unpack(buf)?;
        match opcode {
            ErrorResponse::OPCODE => ErrorResponse::unpack_into(buf),
            ExchangeMtuRequest::OPCODE => ExchangeMtuRequest::unpack_into(buf),
            ExchangeMtuResponse::OPCODE => ExchangeMtuResponse::unpack_into(buf),
            FindInformationRequest::OPCODE => FindInformationRequest::unpack_into(buf),
            FindInformationResponse::OPCODE => FindInformationResponse::unpack_into(buf),
            FindByTypeValueRequest::OPCODE => FindByTypeValueRequest::unpack_into(buf),
            FindByTypeValueResponse::OPCODE => FindByTypeValueResponse::unpack_into(buf),
            ReadByTypeRequest::OPCODE => ReadByTypeRequest::unpack_into(buf),
            ReadByTypeResponse::OPCODE => ReadByTypeResponse::unpack_into(buf),
            ReadByGroupTypeRequest::OPCODE => ReadByGroupTypeRequest::unpack_into(buf),
            ReadByGroupTypeResponse::OPCODE => ReadByGroupTypeResponse::unpack_into(buf),
            ReadRequest::OPCODE => ReadRequest::unpack_into(buf),
            ReadResponse::OPCODE => ReadResponse::unpack_into(buf),
            ReadBlobRequest::OPCODE => ReadBlobRequest::unpack_into(buf),
            ReadBlobResponse::OPCODE => ReadBlobResponse::unpack_into(buf),
            WriteRequest::OPCODE => WriteRequest::unpack_into(buf),
            WriteResponse::OPCODE => WriteResponse::unpack_into(buf),
            HandleValueNotification::OPCODE => HandleValueNotification::unpack_into(buf),
            x => Err(UnpackError::UnknownOpcode(x.into())),
        }
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        match self {
            Att::ErrorResponse(item) => item.pack_attr(buf),
            Att::ExchangeMtuRequest(item) => item.pack_attr(buf),
            Att::ExchangeMtuResponse(item) => item.pack_attr(buf),
            Att::FindInformationRequest(item) => item.pack_attr(buf),
            Att::FindInformationResponse(item) => item.pack_attr(buf),
            Att::FindByTypeValueRequest(item) => item.pack_attr(buf),
            Att::FindByTypeValueResponse(item) => item.pack_attr(buf),
            Att::ReadByTypeRequest(item) => item.pack_attr(buf),
            Att::ReadByTypeResponse(item) => item.pack_attr(buf),
            Att::ReadByGroupTypeRequest(item) => item.pack_attr(buf),
            Att::ReadByGroupTypeResponse(item) => item.pack_attr(buf),
            Att::ReadRequest(item) => item.pack_attr(buf),
            Att::ReadResponse(item) => item.pack_attr(buf),
            Att::ReadBlobRequest(item) => item.pack_attr(buf),
            Att::ReadBlobResponse(item) => item.pack_attr(buf),
            Att::WriteRequest(item) => item.pack_attr(buf),
            Att::WriteResponse(item) => item.pack_attr(buf),
            Att::HandleValueNotification(item) => item.pack_attr(buf),
        }
    }
}

impl fmt::Debug for Att {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Att::ErrorResponse(item) => item.fmt(f),
            Att::ExchangeMtuRequest(item) => item.fmt(f),
            Att::ExchangeMtuResponse(item) => item.fmt(f),
            Att::FindInformationRequest(item) => item.fmt(f),
            Att::FindInformationResponse(item) => item.fmt(f),
            Att::FindByTypeValueRequest(item) => item.fmt(f),
            Att::FindByTypeValueResponse(item) => item.fmt(f),
            Att::ReadByTypeRequest(item) => item.fmt(f),
            Att::ReadByTypeResponse(item) => item.fmt(f),
            Att::ReadByGroupTypeRequest(item) => item.fmt(f),
            Att::ReadByGroupTypeResponse(item) => item.fmt(f),
            Att::ReadRequest(item) => item.fmt(f),
            Att::ReadResponse(item) => item.fmt(f),
            Att::ReadBlobRequest(item) => item.fmt(f),
            Att::ReadBlobResponse(item) => item.fmt(f),
            Att::WriteRequest(item) => item.fmt(f),
            Att::WriteResponse(item) => item.fmt(f),
            Att::HandleValueNotification(item) => item.fmt(f),
        }
    }
}
