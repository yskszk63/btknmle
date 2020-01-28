use futures::future::TryFutureExt as _;
use futures::stream::StreamExt as _;

use btknmle::input::input_loop;
use btknmle::util::CancelableStreamController;
use btknmle::{gap, hogp};
use btknmle_server::gatt;

fn on_err(e: failure::Error) {
    log::error!("{}", e)
}

async fn run(devid: u16, varfile: String, grab: bool) -> Result<(), failure::Error> {
    use tokio::task::spawn_local;

    let gap = gap::Gap::setup(devid, varfile, |p| println!("Please input '{}'", p)).await?;
    spawn_local(
        gap.run()
            .map_err(Into::<failure::Error>::into)
            .unwrap_or_else(on_err),
    );

    let (db, kbd, mouse) = hogp::new();
    let mut listener = gatt::GattListener::new(db)?;
    while let Some(sock) = listener.next().await {
        log::debug!("connected");
        match sock {
            Ok(svc) => {
                let mut cancel = CancelableStreamController::new();
                let kbd_notify = svc.notify_for(&kbd)?;
                let mouse_notify = svc.notify_for(&mouse)?;

                spawn_local(
                    input_loop(kbd_notify, mouse_notify, cancel.factory(), grab)
                        .unwrap_or_else(on_err),
                );

                spawn_local(async move {
                    log::debug!("begin");
                    svc.run()
                        .map_err(Into::<failure::Error>::into)
                        .unwrap_or_else(on_err)
                        .await;
                    cancel.cancel();
                    log::debug!("done");
                });
            }
            Err(e) => on_err(e.into()),
        }
    }

    Ok(())
}

fn main() -> Result<(), failure::Error> {
    use clap::*;

    dotenv::dotenv().ok();
    env_logger::init();

    let m = clap::app_from_crate!()
        .arg(
            Arg::with_name("var-file")
                .short("f")
                .long("var-file")
                .value_name("FILE")
                .env("BTKNMLE_VAR_FILE")
                .default_value("/var/lib/btknmle/db")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("device-id")
                .short("d")
                .long("device-id")
                .value_name("DEVID")
                .env("BTKNMLE_DEVID")
                .default_value("0")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("grab")
                .long("grab")
                .value_name("GRAB")
                .env("BTKNMLE_GRAB")
                .default_value("0")
                .takes_value(true),
        )
        .get_matches();

    let varfile = m.value_of("var-file").unwrap();
    let devid = m.value_of("device-id").unwrap().parse()?;
    let grab = m.value_of("grab").unwrap() != "0";

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    tokio::task::LocalSet::new().block_on(&mut rt, run(devid, varfile.into(), grab))
}
