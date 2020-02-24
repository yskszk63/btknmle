use bytes::{Buf, BufMut};
use std::convert::{TryFrom, TryInto};

use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AddressType {
    BrEdr,
    LePublic,
    LeRandom,
}

impl PacketData for AddressType {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        u8::unpack(buf)?
            .try_into()
            .map_err(UnpackError::UnexpectedValue)
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        u8::from(self.clone()).pack(buf)
    }
}

impl From<AddressType> for u8 {
    fn from(v: AddressType) -> Self {
        match v {
            AddressType::BrEdr => 0x00,
            AddressType::LePublic => 0x01,
            AddressType::LeRandom => 0x02,
        }
    }
}

impl TryFrom<u8> for AddressType {
    type Error = u8;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(match v {
            0x00 => AddressType::BrEdr,
            0x01 => AddressType::LePublic,
            0x02 => AddressType::LeRandom,
            v => return Err(v),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = AddressType::BrEdr;
        e.pack(&mut b).unwrap();
        let r = AddressType::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
