use bytes::{Buf, BufMut, Bytes};

use super::{Att, AttItem};
use crate::{PackError, PacketData, UnpackError};
use crate::util::HexDisplay;

#[derive(Debug, PartialEq, Eq)]
pub struct ReadResponse {
    attribute_value: HexDisplay<Bytes>,
}

impl ReadResponse {
    pub fn new(attribute_value: impl Into<Bytes>) -> Self {
        Self {
            attribute_value: attribute_value.into().into(),
        }
    }
}

impl AttItem for ReadResponse {
    const OPCODE: u8 = 0x0B;
}

impl PacketData for ReadResponse {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let attribute_value = buf.to_bytes().into();
        Ok(Self { attribute_value })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        if buf.remaining_mut() < self.attribute_value.len() {
            return Err(PackError::InsufficientBufLength)
        }
        buf.put(self.attribute_value.clone());
        Ok(())
    }
}

impl From<ReadResponse> for Att {
    fn from(v: ReadResponse) -> Att {
        Att::ReadResponse(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = Att::from(ReadResponse::new(Bytes::from("abc")));
        e.pack(&mut b).unwrap();
        let r = Att::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
