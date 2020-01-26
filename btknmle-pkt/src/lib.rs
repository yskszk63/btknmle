use bytes::{Buf, BytesMut};
use failure::Fail;

pub mod att;
pub mod mgmt;
mod util;

#[derive(Debug, Fail)]
pub enum CodecError {
    #[fail(display = "Underflow")]
    Underflow,
    #[fail(display = "Unknown attribute {:x}", _0)]
    UnknownAtt(u8),
    #[fail(display = "Unknown management packet {:x}", _0)]
    UnknownMgmt(u16),
    #[fail(display = "invalid format")]
    Invalid,
}

pub type Result<T> = std::result::Result<T, CodecError>;

pub trait Codec: Sized {
    fn parse(data: &mut impl Buf) -> Result<Self>;
    fn write_to(&self, buf: &mut BytesMut) -> Result<()>;
}
