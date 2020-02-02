use std::marker::PhantomData;

use bytes::buf::BufExt as _;
use bytes::{Buf, BufMut as _, Bytes, BytesMut};

use super::{Att, AttItem, Codec, CodecError, Handle};

#[derive(Debug)]
pub struct ReadByTypeResponseBuilder<V> {
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
            attribute_value: attribute_value.into(),
        };
        self.attribute_data_list.push(data);
        self
    }

    pub fn build(&mut self) -> ReadByTypeResponse {
        ReadByTypeResponse {
            attribute_data_list: self.attribute_data_list.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct AttributeData {
    attribute_handle: Handle,
    attribute_value: Bytes,
}

#[derive(Debug)]
pub struct ReadByTypeResponse {
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
        let data = AttributeData {
            attribute_handle: attribute_handle.into(),
            attribute_value: attribute_value.into(),
        };

        ReadByTypeResponseBuilder {
            attribute_data_list: vec![data],
            _phantom: PhantomData,
        }
    }
}

impl AttItem for ReadByTypeResponse {
    const OPCODE: u8 = 0x09;
}

impl Codec for ReadByTypeResponse {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        let len = buf.get_u8() as usize;
        let mut attribute_data_list = vec![];
        while buf.has_remaining() {
            let attribute_handle = Handle::parse(buf)?;
            let attribute_value = buf.take(len - 4).to_bytes();
            attribute_data_list.push(AttributeData {
                attribute_handle,
                attribute_value,
            });
        }
        Ok(Self {
            attribute_data_list,
        })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        let mut iter = self.attribute_data_list.iter();
        let head = match iter.next() {
            Some(e) => e,
            None => panic!(), // TODO
        };

        let len = (head.attribute_value.len() + 2) as u8;
        buf.put_u8(len);

        head.attribute_handle.write_to(buf)?;
        buf.put(head.attribute_value.clone());

        for item in iter {
            item.attribute_handle.write_to(buf)?;
            buf.put(item.attribute_value.clone());
        }
        Ok(())
    }
}

impl From<ReadByTypeResponse> for Att {
    fn from(v: ReadByTypeResponse) -> Att {
        Att::ReadByTypeResponse(v)
    }
}
