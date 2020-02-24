use bytes::{Buf, BufMut};

use super::LongTermKey;
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct NewLongTermKeyEvent {
    controller_index: ControlIndex,
    store_hint: bool,
    key: LongTermKey,
}

impl NewLongTermKeyEvent {
    pub fn new(controller_index: ControlIndex, store_hint: bool, key: LongTermKey) -> Self {
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

    pub fn key(&self) -> &LongTermKey {
        &self.key
    }
}

impl EventItem for NewLongTermKeyEvent {
    const CODE: Code = Code(0x000A);

    fn with_controller_index(mut self, idx: ControlIndex) -> Self {
        self.controller_index = idx;
        self
    }
}

impl PacketData for NewLongTermKeyEvent {
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

impl From<NewLongTermKeyEvent> for MgmtEvent {
    fn from(v: NewLongTermKeyEvent) -> Self {
        Self::NewLongTermKeyEvent(v)
    }
}

#[cfg(test)]
mod tests {
    use super::super::AddressType;
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = NewLongTermKeyEvent::new(
            Default::default(),
            true,
            LongTermKey::new(
                "00:11:22:33:44:55".parse().unwrap(),
                AddressType::LeRandom,
                0,
                1,
                2,
                [3; 2],
                [4; 8],
                [5; 16],
            ),
        );
        e.pack(&mut b).unwrap();
        let r = NewLongTermKeyEvent::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
