#![warn(clippy::all)]

use std::path::PathBuf;

use btknmle_keydb::Store;
use clap::Clap;
use gatt::Server;
use input::{InputEvent, InputSource};
use tokio::select;
use tokio_stream::StreamExt;

mod gap;
mod hid;
mod hogp;
mod input;
mod sig;

fn authenticated(ltk: &btmgmt::packet::LongTermKey, addr: &btmgmt::packet::Address) -> bool {
    match ltk.key_type() {
        btmgmt::packet::LongTermKeyType::AuthenticatedKey
        | btmgmt::packet::LongTermKeyType::AuthenticatedP256Key => {}
        _ => return false,
    }

    ltk.address().as_ref() == addr.as_ref()
}

fn bonded(store: &Store, addr: &btmgmt::packet::Address) -> bool {
    store.iter_ltks().any(|ltk| authenticated(ltk, addr))
}

enum InputSink<'a, 'b> {
    NotifyHost(&'b gatt::server::Control<hogp::Token>),
    StartAdvertising(&'a btmgmt::Client, u16),
    PasskeyInput(
        &'a btmgmt::Client,
        u16,
        btmgmt::event::UserPasskeyRequest,
        u32,
    ),
    Nop,
}

impl<'a, 'b> InputSink<'a, 'b> {
    async fn handle_event(&mut self, evt: &InputEvent) -> anyhow::Result<()> {
        match self {
            Self::NotifyHost(ctrl) => match evt {
                InputEvent::Keyboard(evt) => {
                    ctrl.notify(&hogp::Token::Keyboard, evt.to_bytes())?;
                }
                InputEvent::Mouse(evt) => {
                    ctrl.notify(&hogp::Token::Mouse, evt.to_bytes())?;
                }
            },

            Self::StartAdvertising(client, devid) => {
                if matches!(evt, InputEvent::Keyboard(..)) {
                    gap::start_advertising(client, *devid).await?;
                    *self = Self::Nop;
                }
            }

            Self::PasskeyInput(client, devid, req, buf, ..) => {
                use hid::KeyboardUsageId::*;
                let keys = if let InputEvent::Keyboard(evt) = evt {
                    evt.keys()
                } else {
                    return Ok(());
                };

                if keys.len() != 1 {
                    return Ok(());
                }

                let k = keys.iter().next().unwrap();
                let k = match k {
                    KEY_0 => 0,
                    KEY_1 => 1,
                    KEY_2 => 2,
                    KEY_3 => 3,
                    KEY_4 => 4,
                    KEY_5 => 5,
                    KEY_6 => 6,
                    KEY_7 => 7,
                    KEY_8 => 8,
                    KEY_9 => 9,
                    KEY_ENTER => {
                        client
                            .call(
                                *devid,
                                btmgmt::command::UserPasskeyReply::new(
                                    req.address().clone(),
                                    req.address_type().clone(),
                                    *buf,
                                ),
                            )
                            .await?;
                        *self = Self::Nop;
                        return Ok(());
                    }
                    _ => return Ok(()),
                };
                *buf = buf.wrapping_mul(10) + k;
                log::debug!("* {}", buf);
            }

            Self::Nop => {}
        }
        Ok(())
    }
}

async fn run(opts: Opts) -> anyhow::Result<()> {
    let Opts {
        device_id,
        grab,
        var_file,
        ..
    } = opts;
    let io_capability = btmgmt::packet::IoCapability::KeyboardOnly;

    let mut store = Store::open(var_file).await?;
    let gap_client = gap::setup(device_id, &store, io_capability).await?;
    let mut gap_events = gap_client.events().await;
    let mut input = InputSource::new()?;
    let mut sig = sig::Sig::new()?;

    let server = Server::bind()?;
    server.needs_bond_mitm()?;
    loop {
        let mut sink = InputSink::Nop;

        gap::start_advertising(&gap_client, device_id).await?;
        let (address, gatt_loop, gatt_ctrl, mut events) = loop {
            select! {
                connection = server.accept() => {
                    let connection = connection?;
                    let authenticated = bonded(&store, &connection.address().clone().into());
                    break connection.run(authenticated, hogp::new());
                }

                gap_evt = gap_events.next() => {
                    if let Some((d, evt)) = gap_evt {
                        if device_id == d.into() {
                            gap::handle_event(device_id, &gap_client, &evt, &mut store, &mut sink).await;
                        }
                    }
                }

                input_evt = input.next() => {
                    sink.handle_event(&input_evt.unwrap()?).await?;
                }

                s = sig.recv() => s?,
            }
        };

        gap::stop_advertising(&gap_client, device_id).await?;
        let mut gatt_loop = tokio::spawn(gatt_loop);
        if bonded(&store, &address.clone().into()) {
            sink = InputSink::NotifyHost(&gatt_ctrl);
            if grab {
                input.grab()?;
            }
        } else {
            sink = InputSink::Nop;
        }

        loop {
            select! {
                r = &mut gatt_loop => {
                    if let Err(err) = r? {
                        log::warn!("{}", err);
                    }
                    break;
                }

                gatt_evt = events.next() => {
                    log::debug!("{:?}", gatt_evt)
                }

                gap_evt = gap_events.next() => {
                    if let Some((d, evt)) = gap_evt {
                        if device_id == d.into() {
                            gap::handle_event(device_id, &gap_client, &evt, &mut store, &mut sink).await;
                            if let btmgmt::event::Event::NewLongTermKey(evt) = evt {
                                if authenticated(evt.key(), &address.clone().into()) {
                                    gatt_ctrl.mark_authenticated();
                                    sink = InputSink::NotifyHost(&gatt_ctrl);
                                    if grab {
                                        input.grab()?;
                                    }
                                }
                            }
                        }
                    }
                }

                input_evt = input.next() => {
                    sink.handle_event(&input_evt.unwrap()?).await?;
                }

                s = sig.recv() => s?,
            }
        }
        input.ungrab()?;
        gatt_loop.abort();
    }
}

#[derive(Debug, Clap)]
struct Opts {
    #[clap(
        short = 'f',
        long,
        env = "BTKNMLE_VAR_FILE",
        default_value = "/var/lib/btknmle/db.toml"
    )]
    var_file: PathBuf,

    #[clap(short = 'd', long, env = "BTKNMLE_DEVID", default_value = "0")]
    device_id: u16,

    #[clap(long, env = "BTKNMLE_GRAB")]
    grab: bool,

    #[clap(short = 'v', long, parse(from_occurrences))]
    verbosity: usize,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    stderrlog::new().verbosity(opts.verbosity).init().ok();
    run(opts).await
}
