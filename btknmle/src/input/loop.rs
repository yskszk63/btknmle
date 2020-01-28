use futures::stream::StreamExt as _;

use input::{Device, DeviceCapability};

use btknmle_input::event::DeviceEvent;
use btknmle_input::event::Event;
use btknmle_input::event::EventTrait as _;
use btknmle_input::event::PointerEvent;
use btknmle_input::LibinputStream;
use btknmle_server::gatt;

use super::kbstat::KbStat;
use super::mousestat::MouseStat;
use crate::util::CancelableStreamFactory;

fn configure_device(device: &mut Device) {
    if device.has_capability(DeviceCapability::Gesture) {
        if let Err(e) = device.config_tap_set_enabled(true) {
            log::warn!("failed to set clickfinger {:?}", e);
        }
    }
}

pub async fn input_loop(
    mut kb: gatt::Notify,
    mut mouse: gatt::Notify,
    factory: CancelableStreamFactory,
    grab: bool,
) -> Result<(), failure::Error> {
    let mut kbstat = KbStat::new();
    let mut mousestat = MouseStat::new();
    let stream = LibinputStream::new_from_udev("seat0", grab)?; // FIXME seat
    let mut stream = factory.with_stream::<_, failure::Error>(stream);

    while let Some(evt) = stream.next().await {
        match evt? {
            Event::Device(DeviceEvent::Added(evt)) => {
                let mut device = evt.device();
                configure_device(&mut device);
            }
            Event::Keyboard(kbd) => {
                kbstat.recv(&kbd);
                kb.send(kbstat.to_bytes()).await?;
            }
            Event::Pointer(PointerEvent::Motion(motion)) => {
                mousestat.recv_motion(&motion);
                mouse.send(mousestat.to_bytes()).await?;
            }
            Event::Pointer(PointerEvent::Button(button)) => {
                mousestat.recv_button(&button);
                mouse.send(mousestat.to_bytes()).await?;
            }
            Event::Pointer(PointerEvent::Axis(axis)) => {
                mousestat.recv_axis(&axis);
                mouse.send(mousestat.to_bytes()).await?;
            }
            _ => {}
        }
    }
    Ok(())
}
