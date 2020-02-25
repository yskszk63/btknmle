use bitflags::bitflags;
use bytes::{BufMut, Bytes, BytesMut};

use btknmle_input::event::pointer::{
    Axis, ButtonState, PointerAxisEvent, PointerButtonEvent, PointerMotionEvent,
};
use btknmle_input::ButtonCodes;

bitflags! {
    pub struct Button: u8 {
        const LEFT = 0b0000_0001;
        const RIGHT = 0b0000_0010;
        const MIDDLE = 0b0000_0100;
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    None,
    Move(f64, f64),
    Wheel(f64),
}

#[derive(Debug, Clone)]
pub struct MouseStat {
    button: Button,
    value: Value,
}

impl MouseStat {
    pub fn new() -> Self {
        Self {
            button: Button::empty(),
            value: Value::None,
        }
    }

    pub fn recv_motion(&mut self, evt: &PointerMotionEvent) {
        self.value = Value::Move(evt.dx(), evt.dy())
    }

    pub fn recv_button(&mut self, evt: &PointerButtonEvent) {
        let button = match ButtonCodes::from(evt.button()) {
            ButtonCodes::BTN_LEFT => Button::LEFT,
            ButtonCodes::BTN_RIGHT => Button::RIGHT,
            ButtonCodes::BTN_MIDDLE => Button::MIDDLE,
            _ => return,
        };

        match evt.button_state() {
            ButtonState::Pressed => self.button |= button,
            ButtonState::Released => self.button -= button,
        }
    }

    pub fn recv_axis(&mut self, evt: &PointerAxisEvent) {
        if evt.has_axis(Axis::Vertical) {
            self.value = Value::Wheel(evt.axis_value(Axis::Vertical) * 0.1)
        }
    }

    pub fn to_bytes(&self) -> Bytes {
        let mut b = BytesMut::new();
        b.put_u8(self.button.bits());

        match self.value {
            Value::None => {
                b.put_i8(0x00);
                b.put_i8(0x00);
                b.put_i8(0x00);
            }
            Value::Move(dx, dy) => {
                let dx = dx as i8;
                let dy = dy as i8;

                b.put_i8(dx);
                b.put_i8(dy);
                b.put_i8(0x00);
            }
            Value::Wheel(z) => {
                b.put_i8(0x00);
                b.put_i8(0x00);
                b.put_i8(z as i8);
            }
        }

        b.freeze()
    }
}
