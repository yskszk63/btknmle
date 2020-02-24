use bytes::{Buf, BufMut};

use super::{Att, AttItem, Handle};
use crate::Uuid;
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct ReadByGroupTypeRequest {
    starting_handle: Handle,
    ending_handle: Handle,
    attribute_group_type: Uuid,
}

impl ReadByGroupTypeRequest {
    pub fn new(starting_handle: Handle, ending_handle: Handle, attribute_group_type: Uuid) -> Self {
        Self {
            starting_handle,
            ending_handle,
            attribute_group_type,
        }
    }

    pub fn starting_handle(&self) -> Handle {
        self.starting_handle.clone()
    }

    pub fn ending_handle(&self) -> Handle {
        self.ending_handle.clone()
    }

    pub fn attribute_group_type(&self) -> Uuid {
        self.attribute_group_type.clone()
    }
}

impl AttItem for ReadByGroupTypeRequest {
    const OPCODE: u8 = 0x10;
}

impl PacketData for ReadByGroupTypeRequest {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let starting_handle = PacketData::unpack(buf)?;
        let ending_handle = PacketData::unpack(buf)?;
        let attribute_group_type = PacketData::unpack(buf)?;

        Ok(Self {
            starting_handle,
            ending_handle,
            attribute_group_type,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.starting_handle.pack(buf)?;
        self.ending_handle.pack(buf)?;
        self.attribute_group_type.pack(buf)
    }
}

impl From<ReadByGroupTypeRequest> for Att {
    fn from(v: ReadByGroupTypeRequest) -> Att {
        Att::ReadByGroupTypeRequest(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = Att::from(ReadByGroupTypeRequest::new(
            Handle::from(0x0000),
            Handle::from(0xFFFF),
            Uuid::Uuid128(0),
        ));
        e.pack(&mut b).unwrap();
        let r = Att::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
