use bytes::{Buf, BufMut, Bytes};

use super::{Att, AttItem, Handle};
use crate::util::HexDisplay;
use crate::Uuid16;
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct FindByTypeValueRequest {
    starting_handle: Handle,
    ending_handle: Handle,
    attribute_type: Uuid16,
    attribute_value: HexDisplay<Bytes>,
}

impl FindByTypeValueRequest {
    pub fn new(
        starting_handle: Handle,
        ending_handle: Handle,
        attribute_type: Uuid16,
        attribute_value: Bytes,
    ) -> Self {
        let attribute_value = attribute_value.into();
        Self {
            starting_handle,
            ending_handle,
            attribute_type,
            attribute_value,
        }
    }

    pub fn starting_handle(&self) -> Handle {
        self.starting_handle.clone()
    }

    pub fn ending_handle(&self) -> Handle {
        self.ending_handle.clone()
    }

    pub fn attribute_type(&self) -> Uuid16 {
        self.attribute_type.clone()
    }

    pub fn attribute_value(&self) -> &Bytes {
        &self.attribute_value
    }
}

impl AttItem for FindByTypeValueRequest {
    const OPCODE: u8 = 0x06;
}

impl PacketData for FindByTypeValueRequest {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let starting_handle = PacketData::unpack(buf)?;
        let ending_handle = PacketData::unpack(buf)?;
        let attribute_type = PacketData::unpack(buf)?;
        let attribute_value = buf.to_bytes().into();

        Ok(Self {
            starting_handle,
            ending_handle,
            attribute_type,
            attribute_value,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.starting_handle.pack(buf)?;
        self.ending_handle.pack(buf)?;
        self.attribute_type.pack(buf)?;
        if buf.remaining_mut() < self.attribute_value.len() {
            Err(PackError::InsufficientBufLength)
        } else {
            buf.put(self.attribute_value.clone());
            Ok(())
        }
    }
}

impl From<FindByTypeValueRequest> for Att {
    fn from(v: FindByTypeValueRequest) -> Att {
        Att::FindByTypeValueRequest(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = Att::from(FindByTypeValueRequest::new(
            Handle::from(0x0000),
            Handle::from(0xFFFF),
            Uuid16::from(0x1234),
            Bytes::from("abc"),
        ));
        e.pack(&mut b).unwrap();
        let r = Att::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);

        let mut b = [0u8; 8]; // 8 = e.len - 1
        assert_eq!(
            Err(PackError::InsufficientBufLength),
            e.pack(&mut b.as_mut())
        );
    }
}
