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
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let (tx, ..) = broadcast::channel(16);
        Self { tx }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<u8> {
        self.tx.subscribe()
    }

    pub(crate) fn send(&mut self, evt: KeyboardEvent) -> Result<(), NotInterested> {
        if self.tx.receiver_count() > 0 {
            if evt.key_state() != KeyState::Pressed {
                return Ok(());
            }

            let keycode = KeyCodes::from(evt.key());
            let b = match keycode {
                KeyCodes::KEY_0 | KeyCodes::KEY_KP0 => b'0',
                KeyCodes::KEY_1 | KeyCodes::KEY_KP1 => b'1',
                KeyCodes::KEY_2 | KeyCodes::KEY_KP2 => b'2',
                KeyCodes::KEY_3 | KeyCodes::KEY_KP3 => b'3',
                KeyCodes::KEY_4 | KeyCodes::KEY_KP4 => b'4',
                KeyCodes::KEY_5 | KeyCodes::KEY_KP5 => b'5',
                KeyCodes::KEY_6 | KeyCodes::KEY_KP6 => b'6',
                KeyCodes::KEY_7 | KeyCodes::KEY_KP7 => b'7',
                KeyCodes::KEY_8 | KeyCodes::KEY_KP8 => b'7',
                KeyCodes::KEY_9 | KeyCodes::KEY_KP9 => b'9',
                KeyCodes::KEY_ENTER | KeyCodes::KEY_KPENTER => b'\n',
                _ => return Ok(()),
            };
            self.tx.send(b).unwrap();
            Ok(())
        } else {
            Err(NotInterested(evt))
        }
    }
}
