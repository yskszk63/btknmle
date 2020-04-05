#![warn(clippy::all)]

use std::sync::Arc;

use anyhow::{Error, Result};
use futures::stream::StreamExt as _;
use futures::stream::TryStreamExt as _;
use tokio::sync::Mutex;

use btknmle::hogp;
use btknmle::input::{InputEvent, InputSource};
use btknmle_keydb::KeyDb;
use btknmle_server::{gap, gatt};

#[derive(Debug)]
struct Callback {
    input: Arc<Mutex<InputSource>>,
    grab: bool,
}

#[async_trait::async_trait]
impl gap::GapCallback for Callback {
    async fn passkey_request(&mut self) -> String {
        println!("Waiting passkey input.");
        let mut buf = String::new();
        let mut rx = self.input.lock().await.subscribe();
        while let Some(key) = rx.next().await {
            match key.unwrap() {
                InputEvent::Keyboard(k) if k.keys().len() == 1 => {
                    use btknmle_hid::KeyboardUsageId::*;
                    match k.keys().iter().next().unwrap() {
                        KEY_0 => buf.push('0'),
                        KEY_1 => buf.push('1'),
                        KEY_2 => buf.push('2'),
                        KEY_3 => buf.push('3'),
                        KEY_4 => buf.push('4'),
                        KEY_5 => buf.push('5'),
                        KEY_6 => buf.push('6'),
                        KEY_7 => buf.push('7'),
                        KEY_8 => buf.push('8'),
                        KEY_9 => buf.push('9'),
                        KEY_ENTER => break,
                        b => log::debug!("ignore {:?}", b),
                    }
                }
                _ => {}
            }
        }
        println!("Send passkey to host.");
        buf
    }

    async fn device_connected(&mut self) {
        if self.grab {
            self.input
                .lock()
                .await
                .grab()
                .await
                .unwrap_or_else(|e| on_err(e.into()))
        }
    }

    async fn device_disconnected(&mut self) {
        if self.grab {
            self.input
                .lock()
                .await
                .ungrab()
                .await
                .unwrap_or_else(|e| on_err(e.into()))
        }
    }

    async fn start_advertise(&mut self) {
        println!("Start advertise.");
    }

    async fn end_advertise(&mut self) {
        println!("End advertise.");
    }
}

fn on_err(e: Error) {
    log::error!("{}", e)
}

async fn term_signals() -> Result<(), Error> {
    use tokio::signal::unix::{signal, SignalKind};
    let mut alrm = signal(SignalKind::alarm())?;
    let mut hup = signal(SignalKind::hangup())?;
    let mut int = signal(SignalKind::interrupt())?;
    let mut pipe = signal(SignalKind::pipe())?;
    let mut quit = signal(SignalKind::quit())?;
    let mut term = signal(SignalKind::terminate())?;
    let mut usr1 = signal(SignalKind::user_defined1())?;
    let mut usr2 = signal(SignalKind::user_defined2())?;

    tokio::select! {
        _ = alrm.recv() => {},
        _ = hup.recv() => {},
        _ = int.recv() => {},
        _ = pipe.recv() => {},
        _ = quit.recv() => {},
        _ = term.recv() => {},
        _ = usr1.recv() => {},
        _ = usr2.recv() => {},
    }
    Ok(())
}

async fn run(devid: u16, varfile: String, grab: bool) -> Result<()> {
    let mut input = InputSource::new();
    let keydown = input.subscribe().filter_map(|evt| async {
        match evt {
            Ok(InputEvent::Keyboard(k)) if k.keys().len() == 1 => Some(()),
            _ => None,
        }
    });
    tokio::pin!(keydown);

    let mut input_loop = tokio::task::spawn_local(input.runner()?);
    let input = Arc::new(Mutex::new(input));

    let gap = {
        let adv_uuid = gap::Uuid16::from(0x1812).into();
        let callback = Callback {
            input: input.clone(),
            grab,
        };
        gap::Gap::setup(
            devid,
            adv_uuid,
            "btknmle",
            "btknmle",
            KeyDb::new(varfile).await?,
            callback,
        )
        .await?
    };
    let mut advctrl = gap.adv_ctrl();
    let mut gap_working = tokio::task::spawn(gap.run());

    let (db, kbd, mouse) = hogp::new();
    let mut listener = gatt::GattListener::new(db, gatt::AttSecurityLevel::NeedsBoundMitm)?;

    let signals = term_signals();
    tokio::pin!(signals);

    println!("Listening...");
    loop {
        tokio::select! {
            Some(svc) = listener.next() => {
                log::debug!("connected");
                match svc {
                    Ok(svc) => {
                        let kbd = kbd.clone();
                        let mouse = mouse.clone();
                        let notify = input.lock().await.subscribe();
                        let notify = notify.map_ok(move |evt| {
                            match evt {
                                InputEvent::Keyboard(k) => (kbd.clone(), k.to_bytes()),
                                InputEvent::Mouse(m) => (mouse.clone(), m.to_bytes()),
                            }
                        });

                        tokio::task::spawn(async move {
                            println!("Start sending inputs..");
                            svc.run(notify).await.map_err(Into::into).unwrap_or_else(on_err);
                            println!("Terminate sending inputs..");
                        });
                    }
                    Err(e) => on_err(e.into()),
                }
            }
            gap_done = &mut gap_working => gap_done??,
            input_done = &mut input_loop => input_done??,
            _ = &mut keydown.next() => advctrl.start_advertise().await?,
            _ = &mut signals => break,
            else => break
        }
    }
    println!("Terminating..");

    advctrl.cancel_advertise().await.ok();

    Ok(())
}

fn main() -> Result<()> {
    use clap::*;

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
