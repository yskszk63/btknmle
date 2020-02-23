use bytes::{Buf, BufMut};

use super::{Att, AttItem, Handle};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct ReadRequest {
    attribute_handle: Handle,
}

impl ReadRequest {
    pub fn new(attribute_handle: Handle) -> Self {
        Self {
            attribute_handle,
        }
    }

    pub fn attribute_handle(&self) -> Handle {
        self.attribute_handle.clone()
    }
}

impl AttItem for ReadRequest {
    const OPCODE: u8 = 0x0A;
}

impl PacketData for ReadRequest {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let attribute_handle = PacketData::unpack(buf)?;

        Ok(Self { attribute_handle })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.attribute_handle.pack(buf)
    }
}

impl From<ReadRequest> for Att {
    fn from(v: ReadRequest) -> Att {
        Att::ReadRequest(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = Att::from(ReadRequest::new(Handle::from(0x0000)));
        e.pack(&mut b).unwrap();
        let r = Att::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
