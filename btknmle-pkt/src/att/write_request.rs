use bytes::{Buf, BufMut, Bytes};

use super::{Att, AttItem, Handle};
use crate::{PackError, PacketData, UnpackError};
use crate::util::HexDisplay;

#[derive(Debug, PartialEq, Eq)]
pub struct WriteRequest {
    attribute_handle: Handle,
    attribute_value: HexDisplay<Bytes>,
}

impl WriteRequest {
    pub fn new(attribute_handle: Handle, attribute_value: Bytes) -> Self {
        let attribute_value = attribute_value.into();
        Self {
            attribute_handle,
            attribute_value,
        }
    }

    pub fn attribute_handle(&self) -> Handle {
        self.attribute_handle.clone()
    }
    pub fn attribute_value(&self) -> &Bytes {
        &self.attribute_value
    }
}

impl AttItem for WriteRequest {
    const OPCODE: u8 = 0x12;
}

impl PacketData for WriteRequest {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let attribute_handle = PacketData::unpack(buf)?;
        let attribute_value = buf.to_bytes().into();

        Ok(Self {
            attribute_handle,
            attribute_value,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.attribute_handle.pack(buf)?;
        if buf.remaining_mut() < self.attribute_value.len() {
            return Err(PackError::InsufficientBufLength)
        }
        buf.put(self.attribute_value.clone());
        Ok(())
    }
}

impl From<WriteRequest> for Att {
    fn from(v: WriteRequest) -> Att {
        Att::WriteRequest(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = Att::from(WriteRequest::new(Handle::from(0x0000), Bytes::from("abc")));
        e.pack(&mut b).unwrap();
        let r = Att::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
