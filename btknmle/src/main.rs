use tokio::prelude::*;

use btknmle_server::{gatt, mgmt};

mod gap;
mod hogp;

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
                tokio::spawn(async move {
                    let stdin = tokio::io::stdin();
                    let mut lines = tokio::codec::FramedRead::new(stdin, tokio::codec::LinesCodec::new());
                    while let Some(_) = lines.next().await {
                        let _ = kbd_notify.send(vec![0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00]).await;
                        let _ = kbd_notify.send(vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).await;
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
