use bytes::{Buf, BytesMut};

use super::LongTermKey;
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use super::{Codec, Result};

#[derive(Debug)]
pub struct NewLongTermKeyEvent {
    controller_index: ControlIndex,
    store_hint: bool,
    key: LongTermKey,
}

impl NewLongTermKeyEvent {
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

impl Codec for NewLongTermKeyEvent {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let controller_index = Default::default();
        let store_hint = buf.get_u8() != 0;
        let key = LongTermKey::parse(buf)?;
        Ok(Self {
            controller_index,
            store_hint,
            key,
        })
    }

    fn write_to(&self, _buf: &mut BytesMut) -> Result<()> {
        unimplemented!()
    }
}

impl From<NewLongTermKeyEvent> for MgmtEvent {
    fn from(v: NewLongTermKeyEvent) -> Self {
        Self::NewLongTermKeyEvent(v)
    }
}
