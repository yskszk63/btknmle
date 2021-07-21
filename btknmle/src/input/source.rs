use std::io;

use tokio_stream::StreamExt as _;

use btknmle_input::event::DeviceEvent;
use btknmle_input::event::Event as LibinputEvent;
use btknmle_input::event::EventTrait as _;
use btknmle_input::event::PointerEvent;
use btknmle_input::model::{Device, DeviceCapability};
use btknmle_input::LibinputStream;

use super::kbstat::KbStat;
use super::mousestat::MouseStat;

#[derive(Debug, Clone)]
pub enum InputEvent {
    Keyboard(KbStat),
    Mouse(MouseStat),
}

impl From<KbStat> for InputEvent {
    fn from(v: KbStat) -> Self {
        Self::Keyboard(v)
    }
}

impl From<MouseStat> for InputEvent {
    fn from(v: MouseStat) -> Self {
        Self::Mouse(v)
    }
}

fn configure_device(device: &mut Device) {
    if device.has_capability(DeviceCapability::Gesture) {
        if let Err(e) = device.config_tap_set_enabled(true) {
            log::warn!("failed to set clickfinger {:?}", e);
        }
    }
}

#[derive(Debug)]
pub(crate) struct InputSource {
    libinput: LibinputStream,
    kbstate: KbStat,
    mousestate: MouseStat,
}

impl InputSource {
    pub(crate) fn new() -> io::Result<Self> {
        let libinput = LibinputStream::new_from_udev("seat0")?; // TODO seat name
        let kbstate = KbStat::new();
        let mousestate = MouseStat::new();
        Ok(Self {
            libinput,
            kbstate,
            mousestate,
        })
    }

    pub(crate) async fn next(&mut self) -> Option<io::Result<InputEvent>> {
        while let Some(event) = self.libinput.next().await {
            let event = match event {
                Ok(event) => event,
                Err(err) => return Some(Err(err)),
            };

            match event {
                LibinputEvent::Device(DeviceEvent::Added(evt)) => {
                    let mut device = evt.device();
                    configure_device(&mut device);
                }
                LibinputEvent::Keyboard(kbd) => {
                    self.kbstate.recv(&kbd);
                    return Some(Ok(self.kbstate.clone().into()));
                }
                LibinputEvent::Pointer(PointerEvent::Motion(motion)) => {
                    self.mousestate.recv_motion(&motion);
                    return Some(Ok(self.mousestate.clone().into()));
                }
                LibinputEvent::Pointer(PointerEvent::Button(button)) => {
                    self.mousestate.recv_button(&button);
                    return Some(Ok(self.mousestate.clone().into()));
                }
                LibinputEvent::Pointer(PointerEvent::Axis(axis)) => {
                    self.mousestate.recv_axis(&axis);
                    return Some(Ok(self.mousestate.clone().into()));
                }
                _ => {}
            }
        }
        None
    }

    pub(crate) fn grab(&mut self) -> io::Result<()> {
        self.libinput.grab()
    }

    pub(crate) fn ungrab(&mut self) -> io::Result<()> {
        self.libinput.ungrab()
    }
}
