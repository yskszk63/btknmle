use bytes::{Buf, BufMut as _, BytesMut};

use super::{Att, AttItem, Codec, CodecError, Handle};

#[derive(Debug, Clone)]
pub enum ErrorCode {
    InvalidHandle,
    ReadNotPermitted,
    WriteNotPermitted,
    InvalidPDU,
    InsufficientAuthentication,
    RequestNotSupported,
    InvalidOffset,
    InsufficientAuthorization,
    PrepareQueueFull,
    AttributeNotFound,
    AttributeNotLong,
    InsufficientEncryptionKeySize,
    InvalidAttributeValueLength,
    UnlikelyError,
    InsufficientEncryption,
    UnsupportedGroupType,
    InsufficientResource,
    DatabaseOutOfSync,
    ValueNotAllowed,
    ApplicationError(u8),
    CommonProfileAndServiceErrorCode(u8),
    ReservedForFutureUse(u8),
}

impl From<u8> for ErrorCode {
    fn from(v: u8) -> Self {
        match v {
            0x01 => ErrorCode::InvalidHandle,
            0x02 => ErrorCode::ReadNotPermitted,
            0x03 => ErrorCode::WriteNotPermitted,
            0x04 => ErrorCode::InvalidPDU,
            0x05 => ErrorCode::InsufficientAuthentication,
            0x06 => ErrorCode::RequestNotSupported,
            0x07 => ErrorCode::InvalidOffset,
            0x08 => ErrorCode::InsufficientAuthorization,
            0x09 => ErrorCode::PrepareQueueFull,
            0x0A => ErrorCode::AttributeNotFound,
            0x0B => ErrorCode::AttributeNotLong,
            0x0C => ErrorCode::InsufficientEncryptionKeySize,
            0x0D => ErrorCode::InvalidAttributeValueLength,
            0x0E => ErrorCode::UnlikelyError,
            0x0F => ErrorCode::InsufficientEncryption,
            0x10 => ErrorCode::UnsupportedGroupType,
            0x11 => ErrorCode::InsufficientResource,
            0x12 => ErrorCode::DatabaseOutOfSync,
            0x13 => ErrorCode::ValueNotAllowed,
            v if (0x80..=0x9F).contains(&v) => ErrorCode::ApplicationError(v),
            v if (0xE0..=0xFF).contains(&v) => ErrorCode::CommonProfileAndServiceErrorCode(v),
            v => ErrorCode::ReservedForFutureUse(v),
        }
    }
}

impl From<ErrorCode> for u8 {
    fn from(v: ErrorCode) -> u8 {
        match v {
            ErrorCode::InvalidHandle => 0x01,
            ErrorCode::ReadNotPermitted => 0x02,
            ErrorCode::WriteNotPermitted => 0x03,
            ErrorCode::InvalidPDU => 0x04,
            ErrorCode::InsufficientAuthentication => 0x05,
            ErrorCode::RequestNotSupported => 0x06,
            ErrorCode::InvalidOffset => 0x07,
            ErrorCode::InsufficientAuthorization => 0x08,
            ErrorCode::PrepareQueueFull => 0x09,
            ErrorCode::AttributeNotFound => 0x0A,
            ErrorCode::AttributeNotLong => 0x0B,
            ErrorCode::InsufficientEncryptionKeySize => 0x0C,
            ErrorCode::InvalidAttributeValueLength => 0x0D,
            ErrorCode::UnlikelyError => 0x0E,
            ErrorCode::InsufficientEncryption => 0x0F,
            ErrorCode::UnsupportedGroupType => 0x10,
            ErrorCode::InsufficientResource => 0x11,
            ErrorCode::DatabaseOutOfSync => 0x12,
            ErrorCode::ValueNotAllowed => 0x13,
            ErrorCode::ApplicationError(v)
            | ErrorCode::CommonProfileAndServiceErrorCode(v)
            | ErrorCode::ReservedForFutureUse(v) => v,
        }
    }
}

#[derive(Debug)]
pub struct ErrorResponse {
    request_opcode_in_error: u8,
    attribute_handle_in_error: Handle,
    error_code: ErrorCode,
}

impl ErrorResponse {
    pub fn new(
        request_opcode_in_error: u8,
        attribute_handle_in_error: Handle,
        error_code: ErrorCode,
    ) -> Self {
        Self {
            request_opcode_in_error,
            attribute_handle_in_error,
            error_code,
        }
    }
}

impl AttItem for ErrorResponse {
    const OPCODE: u8 = 0x01;
}

impl Codec for ErrorResponse {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        let request_opcode_in_error = buf.get_u8();
        let attribute_handle_in_error = Handle::parse(buf)?;
        let error_code = buf.get_u8().into();

        Ok(Self {
            request_opcode_in_error,
            attribute_handle_in_error,
            error_code,
        })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        buf.put_u8(self.request_opcode_in_error);
        self.attribute_handle_in_error.write_to(buf)?;
        buf.put_u8(self.error_code.clone().into());

        Ok(())
    }
}

impl From<ErrorResponse> for Att {
    fn from(v: ErrorResponse) -> Att {
        Att::ErrorResponse(v)
    }
}
