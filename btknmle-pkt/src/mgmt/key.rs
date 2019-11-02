use bytes::{Buf, Bytes, BytesMut};

use super::{Address, AddressType};
use super::{Codec, CodecError, Result};

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
            0x01 => Self::UnauthenticatedLocalCsrk,
            0x02 => Self::UnauthenticatedLocalCsrk,
            0x03 => Self::UnauthenticatedLocalCsrk,
            _ => return Err(CodecError::Invalid),
        })
    }

    fn write_to(&self, _buf: &mut BytesMut) -> Result<()> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct Key {
    address: Address,
    address_type: AddressType,
    r#type: Type,
    value: Bytes,
}

impl Codec for Key {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let address = Address::parse(buf)?;
        let address_type = AddressType::parse(buf)?;
        let r#type = Type::parse(buf)?;
        let value = buf.take(usize::max_value()).collect();
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

#[derive(Debug)]
pub struct LongTermKey {
    address: Address,
    address_type: AddressType,
    key_type: u8,
    master: u8,
    encryption_size: u8,
    encryption_diversifier: [u8; 2],
    random_number: [u8; 8],
    value: [u8; 16],
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
        let mut random_number = [0; 8];
        buf.copy_to_slice(&mut random_number);
        let mut value = [0; 16];
        buf.copy_to_slice(&mut value);

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

    fn write_to(&self, _buf: &mut BytesMut) -> Result<()> {
        unimplemented!()
    }
}
