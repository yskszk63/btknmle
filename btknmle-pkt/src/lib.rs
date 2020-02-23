#![warn(clippy::all)]
use bytes::{Buf, BytesMut};
use failure::Fail;

pub use packet_data::*;
pub use uuid::*;

pub mod att;
pub mod mgmt;
mod packet_data;
mod util;
mod uuid;

// TODO delete after refactoring
#[derive(Debug, Fail)]
pub enum CodecError {
    #[fail(display = "Underflow")]
    Underflow,
    #[fail(display = "Unknown attribute {:x}", _0)]
    UnknownAtt(u8),
    #[fail(display = "Unknown management packet {:#06x}", _0)]
    UnknownMgmt(u16),
    #[fail(display = "invalid format")]
    Invalid,
}

pub type Result<T> = std::result::Result<T, CodecError>;

pub trait Codec: Sized {
    fn parse(data: &mut impl Buf) -> Result<Self>;
    fn write_to(&self, buf: &mut BytesMut) -> Result<()>;
}

impl<P> Codec for P
where
    P: PacketData,
{
    fn parse(data: &mut impl Buf) -> Result<Self> {
        Self::unpack(data).map_err(|_| CodecError::Invalid)
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        self.pack(buf).map_err(|_| CodecError::Invalid)
    }
}
