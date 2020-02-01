use tokio::sync::broadcast;

use btknmle_input::event::keyboard::{KeyState, KeyboardEvent, KeyboardEventTrait as _};
use btknmle_input::KeyCodes;

#[derive(Debug)]
pub(crate) struct NotInterested(pub(crate) KeyboardEvent);

#[derive(Debug, Clone)]
pub struct PasskeyFilter {
    tx: broadcast::Sender<u8>,
}

impl PasskeyFilter {
    pub fn new() -> Self {
        let (tx, ..) = broadcast::channel(16);
        Self { tx }
    }

    pub(crate) fn subscribe(&self) -> broadcast::Receiver<u8> {
        self.tx.subscribe()
    }

    pub(crate) fn send(&mut self, evt: KeyboardEvent) -> Result<(), NotInterested> {
        if evt.key_state() != KeyState::Pressed {
            return Err(NotInterested(evt));
        }

        let keycode = KeyCodes::from(evt.key());
        let b = match keycode {
            KeyCodes::KEY_0 => b'0',
            KeyCodes::KEY_1 => b'1',
            KeyCodes::KEY_2 => b'2',
            KeyCodes::KEY_3 => b'3',
            KeyCodes::KEY_4 => b'4',
            KeyCodes::KEY_5 => b'5',
            KeyCodes::KEY_6 => b'6',
            KeyCodes::KEY_7 => b'7',
            KeyCodes::KEY_8 => b'7',
            KeyCodes::KEY_9 => b'9',
            KeyCodes::KEY_ENTER => b'\n',
            _ => return Err(NotInterested(evt)),
        };

        match self.tx.send(b) {
            Ok(..) => Ok(()),
            Err(..) => Err(NotInterested(evt)),
        }
    }
}
