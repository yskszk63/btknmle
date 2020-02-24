use std::marker::PhantomData;

use bytes::buf::BufExt as _;
use bytes::{Buf, BufMut, Bytes};

use super::{Att, AttItem, Handle};
use crate::util::HexDisplay;
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug)]
pub struct ReadByTypeResponseBuilder<V> {
    length: Option<usize>,
    attribute_data_list: Vec<AttributeData>,
    _phantom: PhantomData<V>,
}

impl<V> ReadByTypeResponseBuilder<V>
where
    V: Into<Bytes>,
{
    pub fn add(&mut self, attribute_handle: impl Into<Handle>, attribute_value: V) -> &mut Self {
        let data = AttributeData {
            attribute_handle: attribute_handle.into(),
            attribute_value: attribute_value.into().into(),
        };
        if let Some(len) = self.length {
            if len != data.attribute_value.len() {
                panic!("attr value length not match")
            }
        } else {
            self.length = Some(data.attribute_value.len());
        };
        self.attribute_data_list.push(data);
        self
    }

    pub fn build(&mut self) -> ReadByTypeResponse {
        ReadByTypeResponse {
            length: (self.length.unwrap() + 2) as u8,
            attribute_data_list: self.attribute_data_list.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AttributeData {
    attribute_handle: Handle,
    attribute_value: HexDisplay<Bytes>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ReadByTypeResponse {
    length: u8,
    attribute_data_list: Vec<AttributeData>,
}

impl ReadByTypeResponse {
    pub fn builder<V>(
        attribute_handle: impl Into<Handle>,
        attribute_value: V,
    ) -> ReadByTypeResponseBuilder<V>
    where
        V: Into<Bytes>,
    {
        let mut builder = ReadByTypeResponseBuilder {
            length: None,
            attribute_data_list: vec![],
            _phantom: PhantomData,
        };
        builder.add(attribute_handle, attribute_value);
        builder
    }
}

impl AttItem for ReadByTypeResponse {
    const OPCODE: u8 = 0x09;
}

impl PacketData for ReadByTypeResponse {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let length = buf.get_u8();
        let len = length as usize;
        let mut attribute_data_list = vec![];

        if buf.remaining() % len != 0 {
            return Err(UnpackError::UnexpectedEof);
        }

        while buf.has_remaining() {
            let attribute_handle = PacketData::unpack(buf)?;
            let attribute_value = buf.take(len - 2).to_bytes().into();
            attribute_data_list.push(AttributeData {
                attribute_handle,
                attribute_value,
            });
        }

        Ok(Self {
            length,
            attribute_data_list,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        if buf.remaining_mut() < self.attribute_data_list.len() * (self.length as usize) {
            return Err(PackError::InsufficientBufLength);
        }

        self.length.pack(buf)?;
        for attr in &self.attribute_data_list {
            attr.attribute_handle.pack(buf)?;
            buf.put(attr.attribute_value.clone());
        }
        Ok(())
    }
}

impl From<ReadByTypeResponse> for Att {
    fn from(v: ReadByTypeResponse) -> Att {
        Att::ReadByTypeResponse(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = Att::from(
            ReadByTypeResponse::builder(Handle::from(0x0000), "aaa")
                .add(0x0000, "bbb")
                .build(),
        );
        e.pack(&mut b).unwrap();
        let r = Att::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
