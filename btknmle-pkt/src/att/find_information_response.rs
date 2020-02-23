use std::marker::PhantomData;

use bytes::buf::BufExt as _;
use bytes::{Buf, BufMut, Bytes};

use super::{Att, AttItem, Handle};
use crate::{Uuid16, Uuid128};
use crate::{PackError, PacketData, UnpackError};
use crate::util::HexDisplay;


#[derive(Debug, PartialEq, Eq)]
pub enum Format {
    F2,
    F16,
}

impl Format {
    fn len(&self) -> usize {
        match self {
            Self::F2 => 2,
            Self::F16 => 16,
        }
    }
}

impl PacketData for Format {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        match u8::unpack(buf)? {
            0x01 => Ok(Self::F2),
            0x02 => Ok(Self::F16),
            x => Err(UnpackError::unexpected(format!("{:02x}", x))),
        }
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        let format = match self {
            Self::F2 => 0x01u8,
            Self::F16 => 0x02u8,
        };
        format.pack(buf)
    }
}

pub trait IntoAttributeDataValue: Into<Bytes> {
    const FORMAT: Format;
}

impl IntoAttributeDataValue for Uuid16 {
    const FORMAT: Format = Format::F2;
}

impl IntoAttributeDataValue for Uuid128 {
    const FORMAT: Format = Format::F16;
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AttributeData {
    attribute_handle: Handle,
    attribute_value: HexDisplay<Bytes>,
}

impl AttributeData {
    fn new<V>(attribute_handle: impl Into<Handle>, attribute_value: V) -> Self
    where V: IntoAttributeDataValue {
        let attribute_handle = attribute_handle.into();
        let attribute_value = attribute_value.into();

        if attribute_value.len() != (V::FORMAT.len()) {
            panic!("{} != {}", attribute_value.len(), V::FORMAT.len());
        }

        let attribute_value = attribute_value.into();
        Self {
            attribute_handle,
            attribute_value,
        }
    }
}

#[derive(Debug)]
pub struct FindInformationResponseBuilder<V> {
    attribute_data_list: Vec<AttributeData>,
    _phantom: PhantomData<V>,
}

impl<V> FindInformationResponseBuilder<V>
where
    V: IntoAttributeDataValue,
{
    pub fn add(&mut self, attribute_handle: impl Into<Handle>, attribute_value: V) -> &mut Self {
        let data = AttributeData::new(attribute_handle, attribute_value);
        self.attribute_data_list.push(data);
        self
    }

    pub fn build(&mut self) -> FindInformationResponse {
        FindInformationResponse::new::<V>(self.attribute_data_list.clone())
    }
}

// TODO implement iter iter_mut into_iter extend from_iter
#[derive(Debug, PartialEq, Eq)]
pub struct FindInformationResponse {
    format: Format,
    attribute_data_list: Vec<AttributeData>,
}

impl FindInformationResponse {
    pub fn builder<V>(
        attribute_handle: impl Into<Handle>,
        attribute_value: V,
    ) -> FindInformationResponseBuilder<V>
    where
        V: IntoAttributeDataValue,
    {
        let mut builder = FindInformationResponseBuilder {
            attribute_data_list: vec![],
            _phantom: PhantomData,
        };

        builder.add(attribute_handle, attribute_value);
        builder
    }

    fn new<V>(attribute_data_list: Vec<AttributeData>) -> Self
    where V: IntoAttributeDataValue {
        let format = V::FORMAT;
        Self {
            format,
            attribute_data_list,
        }
    }
}

impl AttItem for FindInformationResponse {
    const OPCODE: u8 = 0x05;
}

impl PacketData for FindInformationResponse {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let format = Format::unpack(buf)?;
        let len = format.len();
        let mut attribute_data_list = vec![];

        while buf.has_remaining() {
            let attribute_handle = PacketData::unpack(buf)?;
            if buf.remaining() < len {
                return Err(UnpackError::UnexpectedEof)
            }
            let attribute_value = buf.take(len).to_bytes().into();

            attribute_data_list.push(AttributeData {
                attribute_handle,
                attribute_value,
            });
        }

        Ok(Self {
            format,
            attribute_data_list,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.format.pack(buf)?;
        let len = self.format.len();

        for item in &self.attribute_data_list {
            item.attribute_handle.pack(buf)?;
            if buf.remaining_mut() < len {
                return Err(PackError::InsufficientBufLength)
            }
            buf.put(item.attribute_value.clone());
        }
        Ok(())
    }
}

impl From<FindInformationResponse> for Att {
    fn from(v: FindInformationResponse) -> Att {
        Att::FindInformationResponse(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid16() {
        let mut b = vec![];
        let e = Att::from(FindInformationResponse::builder(Handle::from(0x0000), Uuid16::from(0xFFFF)).build());
        e.pack(&mut b).unwrap();
        let r = Att::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }

    #[test]
    fn test_uuid128() {
        let mut b = vec![];
        let e = FindInformationResponse::builder(Handle::from(0x0000), Uuid128::from(0xFFFF)).build();
        e.pack(&mut b).unwrap();
        let r = FindInformationResponse::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
