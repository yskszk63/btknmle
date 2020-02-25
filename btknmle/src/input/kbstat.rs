use std::collections::HashSet;
use std::convert::TryFrom;

use bitflags::bitflags;
use bytes::{BufMut as _, Bytes, BytesMut};

use btknmle_hid::KeyboardUsageId;
use btknmle_input::event::keyboard::KeyState;
use btknmle_input::event::keyboard::KeyboardEventTrait as _;
use btknmle_input::event::KeyboardEvent;
use btknmle_input::KeyCodes;

bitflags! {
    pub struct MetaKeys: u8 {
        const LEFT_CTRL = 0b0000_0001;
        const LEFT_SHIFT = 0b0000_0010;
        const LEFT_ALT = 0b0000_0100;
        const LEFT_GUI = 0b0000_1000;
        const RIGHT_CTRL = 0b0001_0000;
        const RIGHT_SHIFT = 0b0010_0000;
        const RIGHT_ALT = 0b0100_0000;
        const RIGHT_GUI = 0b1000_0000;
    }
}

impl MetaKeys {
    fn from_keycodes(k: &KeyboardUsageId) -> Option<Self> {
        Some(match k {
            KeyboardUsageId::KEY_LEFT_CTRL => Self::LEFT_CTRL,
            KeyboardUsageId::KEY_LEFT_SHIFT => Self::LEFT_SHIFT,
            KeyboardUsageId::KEY_LEFT_ALT => Self::LEFT_ALT,
            KeyboardUsageId::KEY_LEFT_GUI => Self::LEFT_GUI,
            KeyboardUsageId::KEY_RIGHT_CTRL => Self::RIGHT_CTRL,
            KeyboardUsageId::KEY_RIGHT_SHIFT => Self::RIGHT_SHIFT,
            KeyboardUsageId::KEY_RIGHT_ALT => Self::RIGHT_ALT,
            KeyboardUsageId::KEY_RIGHT_GUI => Self::RIGHT_GUI,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone)]
pub struct KbStat {
    meta: MetaKeys,
    keys: HashSet<KeyboardUsageId>,
}

impl KbStat {
    pub fn new() -> Self {
        Self {
            meta: MetaKeys::empty(),
            keys: Default::default(),
        }
    }

    pub fn keys(&self) -> &HashSet<KeyboardUsageId> {
        &self.keys
    }

    pub fn recv(&mut self, evt: &KeyboardEvent) {
        let code = KeyCodes::from(evt.key());
        let code = KeyboardUsageId::try_from(code);
        if let Ok(code) = code {
            match evt.key_state() {
                KeyState::Pressed => {
                    if let Some(meta) = MetaKeys::from_keycodes(&code) {
                        self.meta |= meta;
                    } else {
                        self.keys.insert(code);
                    }
                }
                KeyState::Released => {
                    if let Some(meta) = MetaKeys::from_keycodes(&code) {
                        self.meta -= meta;
                    } else {
                        self.keys.remove(&code);
                    }
                }
            }
        }
    }

    pub fn to_bytes(&self) -> Bytes {
        let mut b = BytesMut::new();
        b.put_u8(self.meta.bits());
        b.put_u8(0x00);
        for v in &self.keys {
            b.put_u8(v.clone().into())
        }
        b.put_u8(0x00);
        b.put_u8(0x00);
        b.put_u8(0x00);
        b.put_u8(0x00);
        b.put_u8(0x00);
        b.put_u8(0x00);

        b.freeze().split_to(8)
    }
}
