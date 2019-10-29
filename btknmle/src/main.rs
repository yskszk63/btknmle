use futures::future::FutureExt as _;
use tokio::prelude::*;

use btknmle::{gap, hogp};
use btknmle::util::{CancelableStreamController, CancelableStreamFactory};
use btknmle::kbstat::KbStat;
use btknmle::mousestat::MouseStat;
use btknmle_server::{gatt, mgmt};
use btknmle_input::event::Event;
use btknmle_input::event::PointerEvent;
use btknmle_input::LibinputStream;


async fn input_loop(mut kb: gatt::Notify, mut mouse: gatt::Notify, factory: CancelableStreamFactory) -> Result<(), failure::Error> {
    let mut kbstat = KbStat::new();
    let mut mousestat = MouseStat::new();
    let stream = LibinputStream::new_from_udev("seat0")?; // FIXME seat
    let mut stream = factory.with_stream::<_, failure::Error>(stream);

    while let Some(evt) = stream.next().await {
        match evt? {
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

#[tokio::main(single_thread)]
async fn main() -> Result<(), failure::Error> {
    use tokio::runtime::current_thread::spawn;

    dotenv::dotenv().ok();
    env_logger::init();

    let devid = 0; // FIXME
    let mut mgmt = mgmt::Mgmt::new(devid).await?;
    gap::setup(&mut mgmt).await?;

    let (db, kbd, mouse) = hogp::new();
    let mut listener = gatt::GattListener::new(db)?;
    while let Some(sock) = listener.next().await {
        log::debug!("connected");
        match sock {
            Ok(svc) => {
                let mut cancel = CancelableStreamController::new();
                let kbd_notify = svc.notify_for(&kbd).unwrap();
                let mouse_notify = svc.notify_for(&mouse).unwrap();

                spawn(input_loop(kbd_notify, mouse_notify, cancel.factory())
                    .map(|e: Result<_, failure::Error>| {
                        if let Err(e) = e {
                            log::warn!("{}", e)
                        }
                    }),
                );

                spawn(async move {
                    log::debug!("begin");
                    match svc.run().await {
                        Ok(()) => {}
                        Err(e) => log::warn!("{}", e),
                    }
                    cancel.cancel().await;
                    log::debug!("done");
                });
            }
            Err(e) => log::warn!("{}", e),
        }
    }

    Ok(())
}
