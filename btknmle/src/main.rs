#![warn(clippy::all)]

use anyhow::{Error, Result};
use futures::future::TryFutureExt as _;
use futures::stream::StreamExt as _;

use btknmle::hogp;
use btknmle::input::input_loop;
use btknmle::input::PasskeyFilter;
use btknmle_keydb::KeyDb;
use btknmle_server::{gap, gatt};

fn on_err(e: Error) {
    log::error!("{}", e)
}

async fn run(devid: u16, varfile: String, grab: bool) -> Result<()> {
    let passkey_filter = PasskeyFilter::new();

    let gap = {
        let passkey_filter = passkey_filter.clone();
        let adv_uuid = gap::Uuid16::from(0x1812).into();
        gap::Gap::setup(
            devid,
            adv_uuid,
            "btknmle",
            "btknmle",
            KeyDb::new(varfile).await?,
            move || {
                let passkey_filter = passkey_filter.clone();
                async move {
                    let mut buf = String::new();
                    let mut rx = passkey_filter.subscribe();
                    while let Ok(key) = rx.recv().await {
                        match key {
                            b @ b'0'..=b'9' => buf.push(b.into()),
                            b'\n' => break,
                            b => log::debug!("ignore {}", b),
                        }
                    }
                    buf
                }
            },
        )
        .await?
    };
    let mut gap_working = tokio::spawn(gap.run());

    let (db, kbd, mouse) = hogp::new();
    let mut listener = gatt::GattListener::new(db, gatt::AttSecurityLevel::NeedsBoundMitm)?;

    loop {
        tokio::select! {
            maybe_sock = listener.next() => {
                log::debug!("connected");
                match maybe_sock {
                    Some(Ok(svc)) => {
                        let kbd_notify = svc.notify_for(&kbd)?;
                        let mouse_notify = svc.notify_for(&mouse)?;
                        let passkey_filter = passkey_filter.clone();

                        tokio::task::spawn_local(async move {
                            log::debug!("begin");
                            let tasks = tokio::try_join!(
                                input_loop(kbd_notify, mouse_notify, grab, passkey_filter).map_err(Into::<Error>::into),
                                svc.run().map_err(Into::<Error>::into),
                            );
                            log::debug!("done");
                            tasks.map(|_|()).unwrap_or_else(on_err);
                        });
                    }
                    Some(Err(e)) => on_err(e.into()),
                    None => break,
                }
            }
            gap_done = &mut gap_working => gap_done??
        }
    }
    Ok(())
}

fn main() -> Result<()> {
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
