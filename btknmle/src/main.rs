use futures::future::FutureExt as _;
use futures::stream::StreamExt as _;

use input::{Device, DeviceCapability};

use btknmle::kbstat::KbStat;
use btknmle::mousestat::MouseStat;
use btknmle::util::{CancelableStreamController, CancelableStreamFactory};
use btknmle::{gap, hogp};
use btknmle_input::event::DeviceEvent;
use btknmle_input::event::Event;
use btknmle_input::event::EventTrait as _;
use btknmle_input::event::PointerEvent;
use btknmle_input::LibinputStream;
use btknmle_server::{gatt, mgmt};

fn configure_device(device: &mut Device) {
    if device.has_capability(DeviceCapability::Gesture) {
        if let Err(e) = device.config_tap_set_enabled(true) {
            log::warn!("failed to set clickfinger {:?}", e);
        }
    }
}

async fn input_loop(
    mut kb: gatt::Notify,
    mut mouse: gatt::Notify,
    factory: CancelableStreamFactory,
) -> Result<(), failure::Error> {
    let mut kbstat = KbStat::new();
    let mut mousestat = MouseStat::new();
    let stream = LibinputStream::new_from_udev("seat0", true)?; // FIXME seat
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

fn main() -> Result<(), failure::Error> {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    tokio::task::LocalSet::new().block_on(&mut rt, main0())
}

async fn main0() -> Result<(), failure::Error> {
    use tokio::task::spawn_local;

    dotenv::dotenv().ok();
    env_logger::init();

    spawn_local(
        (async {
            use btknmle_keydb::KeyDb;
            let db = "/tmp/e0dfa780-416c-446c-b621-c66ffeaebbee";
            let mut db = KeyDb::new(db).await?;

            let devid = 0; // FIXME
            let mut sock = mgmt::Mgmt::new(devid).await?;
            gap::setup(&mut sock).await?;

            sock.load_irks(db.load_irks().await?).await?;
            sock.load_ltks(db.load_ltks().await?).await?;

            while let Some(evt) = sock.next().await {
                use mgmt::model::MgmtEvent;

                match evt {
                    Ok(MgmtEvent::NewLongTermKeyEvent(evt)) => {
                        if evt.store_hint() {
                            let key = evt.key();
                            println!("{:?}", key);
                            db.store_ltks(key).await?;
                        }
                    }
                    Ok(MgmtEvent::NewIdentityResolvingKeyEvent(evt)) => {
                        if evt.store_hint() {
                            let key = evt.key();
                            println!("{:?}", key);
                            db.store_irks(key).await?;
                        }
                    }
                    Ok(MgmtEvent::UserConfirmationRequestEvent(evt)) => {
                        println!("{:?}", evt);
                        sock.user_confirmation(evt.address(), evt.address_type())
                            .await?;
                        //sock.user_confirmation_negative(evt.address(), evt.address_type()).await?;
                    }
                    evt => println!("{:?}", evt),
                }
            }

            Ok(())
        })
        .map(|e: Result<_, failure::Error>| {
            if let Err(e) = e {
                log::warn!("{}", e)
            }
        }),
    );

    let (db, kbd, mouse) = hogp::new();
    let mut listener = gatt::GattListener::new(db)?;
    while let Some(sock) = listener.next().await {
        log::debug!("connected");
        match sock {
            Ok(svc) => {
                let mut cancel = CancelableStreamController::new();
                let kbd_notify = svc.notify_for(&kbd).unwrap();
                let mouse_notify = svc.notify_for(&mouse).unwrap();

                spawn_local(input_loop(kbd_notify, mouse_notify, cancel.factory()).map(
                    |e: Result<_, failure::Error>| {
                        if let Err(e) = e {
                            log::warn!("{}", e)
                        }
                    },
                ));

                spawn_local(async move {
                    log::debug!("begin");
                    match svc.run().await {
                        Ok(()) => {}
                        Err(e) => log::warn!("{}", e),
                    }
                    cancel.cancel();
                    log::debug!("done");
                });
            }
            Err(e) => log::warn!("{}", e),
        }
    }

    Ok(())
}
