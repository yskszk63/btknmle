use bytes::{Buf, BufMut};

use super::{Att, AttItem, Handle};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct ReadBlobRequest {
    attribute_handle: Handle,
    value_offset: u16,
}

impl ReadBlobRequest {
    pub fn new(attribute_handle: Handle, value_offset: u16) -> Self {
        Self {
            attribute_handle,
            value_offset,
        }
    }

    pub fn attribute_handle(&self) -> Handle {
        self.attribute_handle.clone()
    }

    pub fn value_offset(&self) -> u16 {
        self.value_offset
    }
}

impl AttItem for ReadBlobRequest {
    const OPCODE: u8 = 0x0C;
}

impl PacketData for ReadBlobRequest {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let attribute_handle = PacketData::unpack(buf)?;
        let value_offset = PacketData::unpack(buf)?;
        Ok(Self {
            attribute_handle,
            value_offset,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.attribute_handle.pack(buf)?;
        self.value_offset.pack(buf)
    }
}

impl From<ReadBlobRequest> for Att {
    fn from(v: ReadBlobRequest) -> Att {
        Att::ReadBlobRequest(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = Att::from(ReadBlobRequest::new(Handle::from(0x0000), 0xFFFF));
        e.pack(&mut b).unwrap();
        let r = Att::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
