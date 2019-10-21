use bytes::{Buf, BufMut as _, Bytes, BytesMut};

use super::{Codec, Result, Smp, SmpItem};

#[derive(Debug)]
pub struct PairingConfirm {
    value: [u8; 2],
}

impl PairingConfirm {
    pub fn new(value: [u8; 2]) -> Self {
        Self { value }
    }
}

impl SmpItem for PairingConfirm {
    const CODE: u8 = 0x03;
}

impl Codec for PairingConfirm {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let mut value = [0; 2];
        value.copy_from_slice(&buf.take(2).collect::<Bytes>());
        Ok(Self { value })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        buf.put(&self.value[..]);
        Ok(())
    }
}

impl From<PairingConfirm> for Smp {
    fn from(v: PairingConfirm) -> Self {
        Self::PairingConfirm(v)
    }
}
