use bytes::{Buf, BufMut, Bytes};

use super::{Att, AttItem};
use crate::util::HexDisplay;
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct ReadBlobResponse {
    attribute_value: HexDisplay<Bytes>,
}

impl ReadBlobResponse {
    pub fn new(attribute_value: impl Into<Bytes>) -> Self {
        Self {
            attribute_value: attribute_value.into().into(),
        }
    }
}

impl AttItem for ReadBlobResponse {
    const OPCODE: u8 = 0x0D;
}

impl PacketData for ReadBlobResponse {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let attribute_value = buf.to_bytes().into();

        Ok(Self { attribute_value })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        if buf.remaining_mut() < self.attribute_value.len() {
            return Err(PackError::InsufficientBufLength);
        }

        buf.put(self.attribute_value.clone());
        Ok(())
    }
}

impl From<ReadBlobResponse> for Att {
    fn from(v: ReadBlobResponse) -> Att {
        Att::ReadBlobResponse(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = Att::from(ReadBlobResponse::new("abc"));
        e.pack(&mut b).unwrap();
        let r = Att::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
