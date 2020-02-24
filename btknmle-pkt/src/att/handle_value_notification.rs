use bytes::{Buf, BufMut, Bytes};

use super::{Att, AttItem, Handle};
use crate::util::HexDisplay;
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct HandleValueNotification {
    attribute_handle: Handle,
    attribute_value: HexDisplay<Bytes>,
}

impl HandleValueNotification {
    pub fn new(attribute_handle: impl Into<Handle>, attribute_value: impl Into<Bytes>) -> Self {
        Self {
            attribute_handle: attribute_handle.into(),
            attribute_value: attribute_value.into().into(),
        }
    }
}

impl AttItem for HandleValueNotification {
    const OPCODE: u8 = 0x1B;
}

impl PacketData for HandleValueNotification {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let attribute_handle = PacketData::unpack(buf)?;
        let attribute_value = buf.to_bytes().into();

        Ok(Self {
            attribute_handle,
            attribute_value,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.attribute_handle.pack(buf)?;
        if buf.remaining_mut() < self.attribute_value.len() {
            return Err(PackError::InsufficientBufLength);
        }
        buf.put(self.attribute_value.clone());
        Ok(())
    }
}

impl From<HandleValueNotification> for Att {
    fn from(v: HandleValueNotification) -> Att {
        Att::HandleValueNotification(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = Att::from(HandleValueNotification::new(Handle::from(0x0000), "abc"));
        e.pack(&mut b).unwrap();
        let r = Att::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
