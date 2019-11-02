use bytes::{Buf, BytesMut};

use super::{Codec, Result};

#[derive(Debug, Clone)]
pub struct Address([u8; 6]);

impl Codec for Address {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let mut b = [0; 6];
        buf.copy_to_slice(&mut b);
        Ok(Address(b))
    }

    fn write_to(&self, _buf: &mut BytesMut) -> Result<()> {
        unimplemented!()
    }
}
