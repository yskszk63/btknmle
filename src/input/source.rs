use std::future::Future;
use std::io;
use std::sync::Arc;

use futures_channel::mpsc;
use futures_util::lock::{Mutex, MutexGuard};
use futures_util::{select, FutureExt as _, StreamExt as _};

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
pub(crate) struct InputStream<'a> {
    _guard: MutexGuard<'a, ()>,
    rx: mpsc::UnboundedReceiver<InputEvent>,
    control_tx: mpsc::UnboundedSender<Control>,
}

impl<'a> InputStream<'a> {
    pub(crate) async fn next(&mut self) -> Option<InputEvent> {
        self.rx.next().await
    }
}

impl<'a> Drop for InputStream<'a> {
    fn drop(&mut self) {
        self.control_tx.unbounded_send(Control::EndSubscribe).ok();
    }
}

#[derive(Debug)]
enum Control {
    BeginSubscribe(mpsc::UnboundedSender<InputEvent>),
    EndSubscribe,
}

async fn input_loop(
    mut control_rx: mpsc::UnboundedReceiver<Control>,
    grab: bool,
) -> anyhow::Result<()> {
    let mut libinput = LibinputStream::new_from_udev("seat0")?; // TODO seat name
    let mut kbstat = KbStat::new();
    let mut mousestat = MouseStat::new();

    let mut stream_tx = Option::<mpsc::UnboundedSender<InputEvent>>::None;
    loop {
        select! {
            event = libinput.next().fuse() => {
                let event = if let Some(event) = event {
                    event?
                } else {
                    return Ok(())
                };

                let event = match event {
                    LibinputEvent::Device(DeviceEvent::Added(evt)) => {
                        let mut device = evt.device();
                        configure_device(&mut device);
                        None
                    }
                    LibinputEvent::Keyboard(kbd) => {
                        kbstat.recv(&kbd);
                        Some(InputEvent::from(kbstat.clone()))
                    }
                    LibinputEvent::Pointer(PointerEvent::Motion(motion)) => {
                        mousestat.recv_motion(&motion);
                        Some(InputEvent::from(mousestat.clone()))
                    }
                    LibinputEvent::Pointer(PointerEvent::Button(button)) => {
                        mousestat.recv_button(&button);
                        Some(InputEvent::from(mousestat.clone()))
                    }
                    LibinputEvent::Pointer(PointerEvent::Axis(axis)) => {
                        mousestat.recv_axis(&axis);
                        Some(InputEvent::from(mousestat.clone()))
                    }
                    _ => None,
                };
                if let (Some(event), Some(tx)) = (event, stream_tx.as_mut()) {
                    if let Err(err) = tx.unbounded_send(event) {
                        if err.is_disconnected() {
                            stream_tx = None;
                        }
                    }
                }
            }

            control = control_rx.next().fuse() => {
                match control {
                    Some(Control::BeginSubscribe(new_subscribe)) => {
                        log::debug!("begin capture input.");
                        if grab {
                            libinput.grab()?;
                        }
                        stream_tx = Some(new_subscribe);
                    }
                    Some(Control::EndSubscribe) => {
                        log::debug!("end capture input.");
                        if grab {
                            libinput.ungrab()?;
                        }
                        stream_tx = None;
                    }
                    None => return Ok(()),
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct InputSource {
    stream_lock: Arc<Mutex<()>>,
    control_tx: mpsc::UnboundedSender<Control>,
}

impl InputSource {
    pub(crate) fn new(grab: bool) -> io::Result<(Self, impl Future<Output = anyhow::Result<()>>)> {
        let (control_tx, control_rx) = mpsc::unbounded();

        let me = Self {
            stream_lock: Arc::new(Mutex::new(())),
            control_tx,
        };
        Ok((me, input_loop(control_rx, grab)))
    }

    pub(crate) async fn use_stream(&self) -> anyhow::Result<InputStream<'_>> {
        let guard = self.stream_lock.lock().await;
        let (tx, rx) = mpsc::unbounded();
        self.control_tx
            .clone()
            .unbounded_send(Control::BeginSubscribe(tx))?;
        Ok(InputStream {
            _guard: guard,
            rx,
            control_tx: self.control_tx.clone(),
        })
    }
}
