use bytes::{Buf, BufMut, Bytes};
use std::convert::{TryFrom, TryInto};

use super::{Address, AddressType};
use crate::util::HexDisplay;
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    UnauthenticatedLocalCsrk,
    UnauthenticatedRemoteCsrk,
    AuthenticatedLocalCsrk,
    AuthenticatedRemoteCsrk,
}

impl TryFrom<u8> for Type {
    type Error = u8;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Ok(match v {
            0x00 => Self::UnauthenticatedLocalCsrk,
            0x01 => Self::UnauthenticatedRemoteCsrk,
            0x02 => Self::AuthenticatedLocalCsrk,
            0x03 => Self::AuthenticatedRemoteCsrk,
            x => return Err(x),
        })
    }
}

impl From<Type> for u8 {
    fn from(v: Type) -> Self {
        match v {
            Type::UnauthenticatedLocalCsrk => 0x00,
            Type::UnauthenticatedRemoteCsrk => 0x01,
            Type::AuthenticatedLocalCsrk => 0x02,
            Type::AuthenticatedRemoteCsrk => 0x03,
        }
    }
}

impl PacketData for Type {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        u8::unpack(buf)?
            .try_into()
            .map_err(UnpackError::UnexpectedValue)
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        u8::from(self.clone()).pack(buf)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Key {
    address: Address,
    address_type: AddressType,
    r#type: Type,
    value: Bytes,
}

impl Key {
    pub fn new(address: Address, address_type: AddressType, r#type: Type, value: Bytes) -> Self {
        Self {
            address,
            address_type,
            r#type,
            value,
        }
    }

    pub fn address(&self) -> Address {
        self.address.clone()
    }

    pub fn address_type(&self) -> AddressType {
        self.address_type.clone()
    }

    pub fn r#type(&self) -> Type {
        self.r#type.clone()
    }

    pub fn value(&self) -> Bytes {
        self.value.clone()
    }
}

impl PacketData for Key {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let address = PacketData::unpack(buf)?;
        let address_type = PacketData::unpack(buf)?;
        let r#type = PacketData::unpack(buf)?;
        let value = buf.to_bytes();
        Ok(Self {
            address,
            address_type,
            r#type,
            value,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.address.pack(buf)?;
        self.address_type.pack(buf)?;
        self.r#type.pack(buf)?;
        if buf.remaining_mut() < self.value.len() {
            return Err(PackError::InsufficientBufLength);
        }
        buf.put(self.value.as_ref());
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LongTermKey {
    address: Address,
    address_type: AddressType,
    key_type: u8,
    master: u8,
    encryption_size: u8,
    encryption_diversifier: HexDisplay<[u8; 2]>,
    random_number: HexDisplay<[u8; 8]>,
    value: HexDisplay<[u8; 16]>,
}

impl LongTermKey {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        address: Address,
        address_type: AddressType,
        key_type: u8,
        master: u8,
        encryption_size: u8,
        encryption_diversifier: [u8; 2],
        random_number: [u8; 8],
        value: [u8; 16],
    ) -> Self {
        let encryption_diversifier = encryption_diversifier.into();
        let random_number = random_number.into();
        let value = value.into();
        Self {
            address,
            address_type,
            key_type,
            master,
            encryption_size,
            encryption_diversifier,
            random_number,
            value,
        }
    }

    pub fn address(&self) -> Address {
        self.address.clone()
    }

    pub fn address_type(&self) -> AddressType {
        self.address_type.clone()
    }

    pub fn key_type(&self) -> u8 {
        self.key_type
    }

    pub fn master(&self) -> u8 {
        self.master
    }

    pub fn encryption_size(&self) -> u8 {
        self.encryption_size
    }

    pub fn encryption_diversifier(&self) -> &[u8] {
        self.encryption_diversifier.as_ref()
    }

    pub fn random_number(&self) -> &[u8] {
        self.random_number.as_ref()
    }

    pub fn value(&self) -> &[u8] {
        self.value.as_ref()
    }
}

impl PacketData for LongTermKey {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let address = PacketData::unpack(buf)?;
        let address_type = PacketData::unpack(buf)?;
        let key_type = PacketData::unpack(buf)?;
        let master = PacketData::unpack(buf)?;
        let encryption_size = PacketData::unpack(buf)?;
        if buf.remaining() < 2 + 8 + 16 {
            return Err(UnpackError::UnexpectedEof);
        }
        let mut encryption_diversifier = HexDisplay::new([0; 2]);
        buf.copy_to_slice(encryption_diversifier.as_mut());
        let mut random_number = HexDisplay::new([0; 8]);
        buf.copy_to_slice(random_number.as_mut());
        let mut value = HexDisplay::new([0; 16]);
        buf.copy_to_slice(value.as_mut());

        Ok(Self {
            address,
            address_type,
            key_type,
            master,
            encryption_size,
            encryption_diversifier,
            random_number,
            value,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.address.pack(buf)?;
        self.address_type.pack(buf)?;
        self.key_type.pack(buf)?;
        self.master.pack(buf)?;
        self.encryption_size.pack(buf)?;
        if buf.remaining_mut() < 2 + 8 + 16 {
            return Err(PackError::InsufficientBufLength);
        }
        buf.put(self.encryption_diversifier.as_ref());
        buf.put(self.random_number.as_ref());
        buf.put(self.value.as_ref());
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdentityResolvingKey {
    address: Address,
    address_type: AddressType,
    value: HexDisplay<[u8; 16]>,
}

impl IdentityResolvingKey {
    pub fn new(address: Address, address_type: AddressType, value: [u8; 16]) -> Self {
        let value = value.into();
        Self {
            address,
            address_type,
            value,
        }
    }

    pub fn address(&self) -> Address {
        self.address.clone()
    }

    pub fn address_type(&self) -> AddressType {
        self.address_type.clone()
    }

    pub fn value(&self) -> &[u8] {
        self.value.as_ref()
    }
}

impl PacketData for IdentityResolvingKey {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let address = PacketData::unpack(buf)?;
        let address_type = PacketData::unpack(buf)?;
        if buf.remaining() < 16 {
            return Err(UnpackError::UnexpectedEof);
        }
        let mut value = HexDisplay::new([0; 16]);
        buf.copy_to_slice(value.as_mut());

        Ok(Self {
            address,
            address_type,
            value,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.address.pack(buf)?;
        self.address_type.pack(buf)?;
        if buf.remaining_mut() < 16 {
            return Err(PackError::InsufficientBufLength);
        }
        buf.put(self.value.as_ref());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type() {
        for n in 0..=3 {
            let b = vec![n];
            let r = Type::unpack(&mut b.as_ref()).unwrap();
            let mut b2 = vec![];
            r.pack(&mut b2).unwrap();
            assert_eq!(b, b2);
        }

        assert_eq!(false, Type::unpack(&mut vec![4].as_ref()).is_ok());
    }

    #[test]
    fn test_key() {
        let mut b = vec![];
        let e = Key::new(
            "00:11:22:33:44:55".parse().unwrap(),
            AddressType::LeRandom,
            Type::AuthenticatedLocalCsrk,
            Bytes::from("ok"),
        );
        e.pack(&mut b).unwrap();
        let r = Key::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }

    #[test]
    fn test_ltks() {
        let mut b = vec![];
        let e = LongTermKey::new(
            "00:11:22:33:44:55".parse().unwrap(),
            AddressType::LeRandom,
            0,
            1,
            2,
            [3; 2],
            [4; 8],
            [5; 16],
        );
        e.pack(&mut b).unwrap();
        let r = LongTermKey::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }

    #[test]
    fn test_irks() {
        let mut b = vec![];
        let e = IdentityResolvingKey::new(
            "00:11:22:33:44:55".parse().unwrap(),
            AddressType::LeRandom,
            [1; 16],
        );
        e.pack(&mut b).unwrap();
        let r = IdentityResolvingKey::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
