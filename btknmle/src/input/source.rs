use std::future::Future;
use std::io;

use futures::stream::TryStreamExt as _;
use thiserror::Error;
use tokio::stream::Stream;
use tokio::stream::StreamExt as _;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use btknmle_input::event::DeviceEvent;
use btknmle_input::event::Event as LibinputEvent;
use btknmle_input::event::EventTrait as _;
use btknmle_input::event::PointerEvent;
use btknmle_input::model::{Device, DeviceCapability};
use btknmle_input::LibinputStream;

use super::kbstat::KbStat;
use super::mousestat::MouseStat;

#[derive(Debug)]
enum GrabRequest {
    Grab,
    Ungrab,
}

#[derive(Error, Debug)]
#[error("failed to request grab")]
pub struct GrabRequestError;

#[derive(Error, Debug)]
pub enum SubscribeError {
    #[error("subscription error")]
    Recv(#[from] broadcast::RecvError),
}

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

#[derive(Debug)]
pub struct InputSource {
    input_channel_tx: broadcast::Sender<InputEvent>,
    grab_channel_tx: mpsc::Sender<GrabRequest>,
    grab_channel_rx: Option<mpsc::Receiver<GrabRequest>>,
}

#[derive(Error, Debug)]
pub enum RunError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("invalid state")]
    InvalidState,
}

fn configure_device(device: &mut Device) {
    if device.has_capability(DeviceCapability::Gesture) {
        if let Err(e) = device.config_tap_set_enabled(true) {
            log::warn!("failed to set clickfinger {:?}", e);
        }
    }
}

fn process_event(
    event: LibinputEvent,
    input_channel_tx: &mut broadcast::Sender<InputEvent>,
    kbstate: &mut KbStat,
    mousestate: &mut MouseStat,
) {
    match event {
        LibinputEvent::Device(DeviceEvent::Added(evt)) => {
            let mut device = evt.device();
            configure_device(&mut device);
        }
        LibinputEvent::Keyboard(kbd) => {
            kbstate.recv(&kbd);
            if let Err(broadcast::SendError(v)) = input_channel_tx.send(kbstate.clone().into()) {
                log::trace!("drop {:?} (may be no subscriber)", v)
            }
        }
        LibinputEvent::Pointer(PointerEvent::Motion(motion)) => {
            mousestate.recv_motion(&motion);
            if let Err(broadcast::SendError(v)) = input_channel_tx.send(mousestate.clone().into()) {
                log::trace!("drop {:?} (may be no subscriber)", v)
            }
        }
        LibinputEvent::Pointer(PointerEvent::Button(button)) => {
            mousestate.recv_button(&button);
            if let Err(broadcast::SendError(v)) = input_channel_tx.send(mousestate.clone().into()) {
                log::trace!("drop {:?} (may be no subscriber)", v)
            }
        }
        LibinputEvent::Pointer(PointerEvent::Axis(axis)) => {
            mousestate.recv_axis(&axis);
            if let Err(broadcast::SendError(v)) = input_channel_tx.send(mousestate.clone().into()) {
                log::trace!("drop {:?} (may be no subscriber)", v)
            }
        }
        _ => {}
    }
}

async fn run(
    mut input_channel_tx: broadcast::Sender<InputEvent>,
    mut grab_channel_rx: mpsc::Receiver<GrabRequest>,
) -> Result<(), RunError> {
    let mut stream = LibinputStream::new_from_udev("seat0")?; // TODO seat name
    let mut kbstate = KbStat::new();
    let mut mousestate = MouseStat::new();

    loop {
        tokio::select! {
            Some(event) = stream.next() => {
                process_event(event?, &mut input_channel_tx, &mut kbstate, &mut mousestate)
            },
            Some(request) = grab_channel_rx.next() => {
                match request {
                    GrabRequest::Grab => {
                        stream.grab()?
                    }
                    GrabRequest::Ungrab => {
                        stream.ungrab()?
                    }
                }
            },
            else => return Ok(()) // input or grab close
        }
    }
}

impl InputSource {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let (input_channel_tx, _) = broadcast::channel(32); // size
        let (grab_channel_tx, grab_channel_rx) = mpsc::channel(1);

        let grab_channel_rx = Some(grab_channel_rx);
        Self {
            input_channel_tx,
            grab_channel_tx,
            grab_channel_rx,
        }
    }

    pub fn runner(&mut self) -> Result<impl Future<Output = Result<(), RunError>>, RunError> {
        let grab_channel_rx = self.grab_channel_rx.take();
        if grab_channel_rx.is_none() {
            return Err(RunError::InvalidState);
        }
        Ok(run(self.input_channel_tx.clone(), grab_channel_rx.unwrap()))
    }

    pub fn subscribe(&self) -> impl Stream<Item = Result<InputEvent, SubscribeError>> {
        self.input_channel_tx.subscribe().map_err(Into::into)
    }

    pub async fn grab(&mut self) -> Result<(), GrabRequestError> {
        self.grab_channel_tx
            .send(GrabRequest::Grab)
            .await
            .map_err(|_| GrabRequestError)
    }

    pub async fn ungrab(&mut self) -> Result<(), GrabRequestError> {
        self.grab_channel_tx
            .send(GrabRequest::Ungrab)
            .await
            .map_err(|_| GrabRequestError)
    }
}
