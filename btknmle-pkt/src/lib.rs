use bytes::{Buf, BytesMut};

pub mod att;
pub mod hci;

#[derive(Debug)]
pub enum CodecError {
    Underflow,
    UnknownPkt(u8),
    UnknownEvent(u8),
    UnknownAtt(u8),
    Invalid,
}

pub trait Codec: Sized {
    fn parse(data: &mut impl Buf) -> Result<Self, CodecError>;
    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError>;
}
