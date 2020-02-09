use bytes::{Buf, BufMut as _, BytesMut};
use std::convert::TryFrom;

use super::{Codec, CodecError, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AddressType {
    BrEdr,
    LePublic,
    LeRandom,
}

impl Codec for AddressType {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        Ok(match buf.get_u8() {
            0 => Self::BrEdr,
            1 => Self::LePublic,
            2 => Self::LeRandom,
            _ => return Err(CodecError::Invalid),
        })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        let v = match self {
            Self::BrEdr => 0x00,
            Self::LePublic => 0x01,
            Self::LeRandom => 0x02,
        };
        buf.put_u8(v);
        Ok(())
    }
}

impl From<AddressType> for u8 {
    fn from(v: AddressType) -> Self {
        match v {
            AddressType::BrEdr => 0x00,
            AddressType::LePublic => 0x01,
            AddressType::LeRandom => 0x02,
        }
    }
}

impl TryFrom<u8> for AddressType {
    type Error = u8;
    fn try_from(v: u8) -> std::result::Result<Self, Self::Error> {
        Ok(match v {
            0x00 => AddressType::BrEdr,
            0x01 => AddressType::LePublic,
            0x02 => AddressType::LeRandom,
            v => return Err(v),
        })
    }
}
