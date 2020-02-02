use bytes::buf::BufExt as _;
use bytes::{Buf, BufMut as _, Bytes, BytesMut};
use std::convert::TryFrom;

use super::{Address, AddressType};
use super::{Codec, CodecError, Result};
use crate::util::HexDisplay;

#[derive(Debug, Clone)]
pub enum Type {
    UnauthenticatedLocalCsrk,
    UnauthenticatedRemoteCsrk,
    AuthenticatedLocalCsrk,
    AuthenticatedRemoteCsrk,
}

impl Codec for Type {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        Ok(match buf.get_u8() {
            0x00 => Self::UnauthenticatedLocalCsrk,
            0x01 => Self::UnauthenticatedRemoteCsrk,
            0x02 => Self::AuthenticatedLocalCsrk,
            0x03 => Self::AuthenticatedRemoteCsrk,
            _ => return Err(CodecError::Invalid),
        })
    }

    fn write_to(&self, _buf: &mut BytesMut) -> Result<()> {
        unimplemented!()
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

impl TryFrom<u8> for Type {
    type Error = u8;
    fn try_from(v: u8) -> std::result::Result<Self, Self::Error> {
        Ok(match v {
            0x00 => Type::UnauthenticatedLocalCsrk,
            0x01 => Type::UnauthenticatedRemoteCsrk,
            0x02 => Type::AuthenticatedLocalCsrk,
            0x03 => Type::AuthenticatedRemoteCsrk,
            v => return Err(v),
        })
    }
}

#[derive(Debug)]
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

impl Codec for Key {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let address = Address::parse(buf)?;
        let address_type = AddressType::parse(buf)?;
        let r#type = Type::parse(buf)?;
        let value = buf.take(usize::max_value()).to_bytes();
        Ok(Self {
            address,
            address_type,
            r#type,
            value,
        })
    }

    fn write_to(&self, _buf: &mut BytesMut) -> Result<()> {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
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
        let encryption_diversifier = HexDisplay::new(encryption_diversifier);
        let random_number = HexDisplay::new(random_number);
        let value = HexDisplay::new(value);
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

impl Codec for LongTermKey {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let address = Address::parse(buf)?;
        let address_type = AddressType::parse(buf)?;
        let key_type = buf.get_u8();
        let master = buf.get_u8();
        let encryption_size = buf.get_u8();
        let mut encryption_diversifier = [0; 2];
        buf.copy_to_slice(&mut encryption_diversifier);
        let encryption_diversifier = HexDisplay::new(encryption_diversifier);
        let mut random_number = [0; 8];
        buf.copy_to_slice(&mut random_number);
        let random_number = HexDisplay::new(random_number);
        let mut value = [0; 16];
        buf.copy_to_slice(&mut value);
        let value = HexDisplay::new(value);

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

    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        self.address.write_to(buf)?;
        self.address_type.write_to(buf)?;
        buf.put_u8(self.key_type);
        buf.put_u8(self.master);
        buf.put_u8(self.encryption_size);
        buf.extend_from_slice(self.encryption_diversifier.as_ref());
        buf.extend_from_slice(self.random_number.as_ref());
        buf.extend_from_slice(self.value.as_ref());
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct IdentityResolvingKey {
    address: Address,
    address_type: AddressType,
    value: HexDisplay<[u8; 16]>,
}

impl IdentityResolvingKey {
    pub fn new(address: Address, address_type: AddressType, value: [u8; 16]) -> Self {
        let value = HexDisplay::new(value);
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

impl Codec for IdentityResolvingKey {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let address = Address::parse(buf)?;
        let address_type = AddressType::parse(buf)?;
        let mut value = [0; 16];
        buf.copy_to_slice(&mut value);
        let value = HexDisplay::new(value);

        Ok(Self {
            address,
            address_type,
            value,
        })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        self.address.write_to(buf)?;
        self.address_type.write_to(buf)?;
        buf.extend_from_slice(self.value.as_ref());
        Ok(())
    }
}
