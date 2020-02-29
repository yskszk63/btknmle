use bytes::{Buf, BufMut};

use super::LongTermKey;
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct NewLongTermKeyEvent {
    store_hint: bool,
    key: LongTermKey,
}

impl NewLongTermKeyEvent {
    pub fn new(store_hint: bool, key: LongTermKey) -> Self {
        Self { store_hint, key }
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

    fn into_mgmt(self, index: ControlIndex) -> MgmtEvent {
        MgmtEvent::NewLongTermKeyEvent(index, self)
    }
}

impl PacketData for NewLongTermKeyEvent {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let store_hint = u8::unpack(buf)? != 0;
        let key = PacketData::unpack(buf)?;
        Ok(Self { store_hint, key })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        (self.store_hint as u8).pack(buf)?;
        self.key.pack(buf)
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
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtEvent::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
