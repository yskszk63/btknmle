use bytes::{Buf, BufMut};

use super::{Att, AttItem, Handle};
use crate::Uuid;
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct ReadByTypeRequest {
    starting_handle: Handle,
    ending_handle: Handle,
    attribute_type: Uuid,
}

impl ReadByTypeRequest {
    pub fn new(starting_handle: Handle, ending_handle: Handle, attribute_type: Uuid) -> Self {
        Self {
            starting_handle,
            ending_handle,
            attribute_type,
        }
    }

    pub fn starting_handle(&self) -> Handle {
        self.starting_handle.clone()
    }

    pub fn ending_handle(&self) -> Handle {
        self.ending_handle.clone()
    }

    pub fn attribute_type(&self) -> Uuid {
        self.attribute_type.clone()
    }
}

impl AttItem for ReadByTypeRequest {
    const OPCODE: u8 = 0x08;
}

impl PacketData for ReadByTypeRequest {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let starting_handle = PacketData::unpack(buf)?;
        let ending_handle = PacketData::unpack(buf)?;
        let attribute_type = PacketData::unpack(buf)?;

        Ok(Self {
            starting_handle,
            ending_handle,
            attribute_type,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.starting_handle.pack(buf)?;
        self.ending_handle.pack(buf)?;
        self.attribute_type.pack(buf)
    }
}

impl From<ReadByTypeRequest> for Att {
    fn from(v: ReadByTypeRequest) -> Att {
        Att::ReadByTypeRequest(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Uuid16;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = Att::from(ReadByTypeRequest::new(Handle::from(0x0000), Handle::from(0xFFFF), Uuid16::from(0x01).into()));
        e.pack(&mut b).unwrap();
        let r = Att::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
