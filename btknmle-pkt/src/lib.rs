use bytes::{Buf, BytesMut};
use failure::Fail;

pub mod adv;
pub mod att;
pub mod hci;
pub mod smp;

#[derive(Debug, Fail)]
pub enum CodecError {
    #[fail(display = "Underflow")]
    Underflow,
    #[fail(display = "Unknown packet {:x}", _0)]
    UnknownPkt(u8),
    #[fail(display = "Unknown event {:x}", _0)]
    UnknownEvent(u8),
    #[fail(display = "Unknown attribute {:x}", _0)]
    UnknownAtt(u8),
    #[fail(display = "invalid format")]
    Invalid,
}

pub type Result<T> = std::result::Result<T, CodecError>;

pub trait Codec: Sized {
    fn parse(data: &mut impl Buf) -> Result<Self>;
    fn write_to(&self, buf: &mut BytesMut) -> Result<()>;
}
