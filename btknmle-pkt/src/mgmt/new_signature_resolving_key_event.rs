use bytes::{Buf, BytesMut};

use super::Key;
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use super::{Codec, Result};

#[derive(Debug)]
pub struct NewSignatureResolvingKeyEvent {
    controller_index: ControlIndex,
    store_hint: bool,
    key: Key,
}

impl NewSignatureResolvingKeyEvent {
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

impl Codec for NewSignatureResolvingKeyEvent {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let controller_index = Default::default();
        let store_hint = if buf.get_u8() == 0 { false } else { true };
        let key = Key::parse(buf)?;
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

impl From<NewSignatureResolvingKeyEvent> for MgmtEvent {
    fn from(v: NewSignatureResolvingKeyEvent) -> Self {
        Self::NewSignatureResolvingKeyEvent(v)
    }
}
