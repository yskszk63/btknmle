use bytes::buf::BufExt as _;
use bytes::{Buf, BufMut as _, Bytes, BytesMut};

use super::{Att, AttItem, Codec, CodecError, Handle};

#[derive(Debug)]
pub struct HandleValueNotification {
    attribute_handle: Handle,
    attribute_value: Bytes,
}

impl HandleValueNotification {
    pub fn new(attribute_handle: impl Into<Handle>, attribute_value: impl Into<Bytes>) -> Self {
        Self {
            attribute_handle: attribute_handle.into(),
            attribute_value: attribute_value.into(),
        }
    }
}

impl AttItem for HandleValueNotification {
    const OPCODE: u8 = 0x1B;
}

impl Codec for HandleValueNotification {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        let attribute_handle = Handle::parse(buf)?;
        let attribute_value = buf.take(usize::max_value()).to_bytes();
        Ok(Self {
            attribute_handle,
            attribute_value,
        })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        self.attribute_handle.write_to(buf)?;
        buf.put(self.attribute_value.clone());
        Ok(())
    }
}

impl From<HandleValueNotification> for Att {
    fn from(v: HandleValueNotification) -> Att {
        Att::HandleValueNotification(v)
    }
}
