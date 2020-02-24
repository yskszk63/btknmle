use std::convert::TryFrom;
use std::fmt;

use crate::{PackError, PacketData, UnpackError};
use bytes::{Buf, BufMut, Bytes, BytesMut};

#[derive(failure::Fail, Debug, Clone, PartialEq, Eq)]
#[fail(display = "try from uuid error")]
pub struct TryFromUuidError;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Uuid {
    Uuid16(u16),
    Uuid128(u128),
}

impl PacketData for Uuid {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        match buf.remaining() {
            2 => Ok(Uuid16::unpack(buf)?.into()),
            16 => Ok(Uuid128::unpack(buf)?.into()),
            x => Err(UnpackError::unexpected(format!("data length {}", x))),
        }
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        match self {
            Uuid::Uuid16(e) => e.pack(buf),
            Uuid::Uuid128(e) => e.pack(buf),
        }
    }
}

impl From<Uuid> for Bytes {
    fn from(v: Uuid) -> Self {
        let mut b = BytesMut::new();
        v.pack(&mut b).unwrap();
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

#[derive(Clone, PartialEq, Eq, Hash)]
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

impl TryFrom<Uuid> for Uuid16 {
    type Error = TryFromUuidError;
    fn try_from(v: Uuid) -> Result<Self, Self::Error> {
        match v {
            Uuid::Uuid16(v) => Ok(Self(v)),
            _ => Err(TryFromUuidError),
        }
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

impl PacketData for Uuid16 {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        Ok(u16::unpack(buf)?.into())
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.0.pack(buf)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
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

impl TryFrom<Uuid> for Uuid128 {
    type Error = TryFromUuidError;
    fn try_from(v: Uuid) -> Result<Self, Self::Error> {
        match v {
            Uuid::Uuid128(v) => Ok(Self(v)),
            _ => Err(TryFromUuidError),
        }
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

impl PacketData for Uuid128 {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        Ok(u128::unpack(buf)?.into())
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.0.pack(buf)
    }
}
