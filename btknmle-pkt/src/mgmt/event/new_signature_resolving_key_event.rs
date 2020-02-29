use bytes::{Buf, BufMut};

use super::Key;
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct NewSignatureResolvingKeyEvent {
    controller_index: ControlIndex,
    store_hint: bool,
    key: Key,
}

impl NewSignatureResolvingKeyEvent {
    pub fn new(controller_index: ControlIndex, store_hint: bool, key: Key) -> Self {
        Self {
            controller_index,
            store_hint,
            key,
        }
    }

    pub fn controller_index(&self) -> ControlIndex {
        self.controller_index.clone()
    }

    pub fn store_hint(&self) -> bool {
        self.store_hint
    }

    pub fn key(&self) -> &Key {
        &self.key
    }
}

impl EventItem for NewSignatureResolvingKeyEvent {
    const CODE: Code = Code(0x0019);

    fn with_controller_index(mut self, idx: ControlIndex) -> Self {
        self.controller_index = idx;
        self
    }
}

impl PacketData for NewSignatureResolvingKeyEvent {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let store_hint = u8::unpack(buf)? != 0;
        let key = PacketData::unpack(buf)?;
        Ok(Self {
            controller_index: Default::default(),
            store_hint,
            key,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        (self.store_hint as u8).pack(buf)?;
        self.key.pack(buf)
    }
}

impl From<NewSignatureResolvingKeyEvent> for MgmtEvent {
    fn from(v: NewSignatureResolvingKeyEvent) -> Self {
        Self::NewSignatureResolvingKeyEvent(v)
    }
}

#[cfg(test)]
mod tests {
    use super::super::AddressType;
    use super::super::Type;
    use super::*;
    use bytes::Bytes;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = NewSignatureResolvingKeyEvent::new(
            Default::default(),
            true,
            Key::new(
                "00:11:22:33:44:55".parse().unwrap(),
                AddressType::LeRandom,
                Type::AuthenticatedLocalCsrk,
                Bytes::from("ok"),
            ),
        );
        e.pack(&mut b).unwrap();
        let r = NewSignatureResolvingKeyEvent::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
