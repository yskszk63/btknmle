use tokio::prelude::*;

use btknmle::{gap, hogp};
use btknmle_server::{gatt, mgmt};

#[tokio::main(single_thread)]
async fn main() -> Result<(), failure::Error> {
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
                let mut kbd_notify = svc.notify_for(&kbd).unwrap();
                let mut mouse_notify = svc.notify_for(&mouse).unwrap();

                tokio::runtime::current_thread::spawn(async move {
                    use btknmle_input::LibinputStream;
                    use btknmle_input::event::Event;
                    use btknmle_input::event::PointerEvent;

                    let mut kbstat = btknmle::kbstat::KbStat::new();
                    let mut mousestat = btknmle::mousestat::MouseStat::new();

                    let mut stream = LibinputStream::new_from_udev("seat0").unwrap();
                    while let Some(evt) = stream.next().await {
                        match evt.unwrap() {
                            Event::Keyboard(kbd) => {
                                kbstat.recv(&kbd);
                                kbd_notify.send(kbstat.to_bytes()).await.unwrap();
                            },
                            Event::Pointer(PointerEvent::Motion(motion)) => {
                                mousestat.recv_motion(&motion);
                                mouse_notify.send(mousestat.to_bytes()).await.unwrap();
                            }
                            Event::Pointer(PointerEvent::Button(button)) => {
                                mousestat.recv_button(&button);
                                mouse_notify.send(mousestat.to_bytes()).await.unwrap();
                            }
                            Event::Pointer(PointerEvent::Axis(axis)) => {
                                mousestat.recv_axis(&axis);
                                mouse_notify.send(mousestat.to_bytes()).await.unwrap();
                            }
                            _ => {},
                        }
                    }
                });

                tokio::spawn(async move {
                    log::debug!("begin");
                    match svc.run().await {
                        Ok(()) => {}
                        Err(e) => log::warn!("{}", e),
                    }
                    log::debug!("done");
                });
            }
            Err(e) => log::warn!("{}", e),
        }
    }

    Ok(())
}
