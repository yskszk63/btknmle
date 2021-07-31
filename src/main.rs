#![warn(clippy::all)]

use std::path::PathBuf;

use btknmle_keydb::Store;
use clap::Clap;
use futures_util::StreamExt;
use gatt::Server;
use input::{InputEvent, InputSource};
use tokio::select;

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
    NotifyHost(
        &'b mut gatt::server::Notification,
        &'b mut gatt::server::Notification,
    ),
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
            Self::NotifyHost(kbd, mouse) => match evt {
                InputEvent::Keyboard(evt) => {
                    evt.write_to(kbd).await?;
                }
                InputEvent::Mouse(evt) => {
                    evt.write_to(mouse).await?;
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

    let mut server = Server::bind()?;
    server.needs_bond_mitm()?;
    loop {
        let mut sink = InputSink::Nop;

        gap::start_advertising(&gap_client, device_id).await?;
        let mut connection = loop {
            select! {
                connection = server.accept(hogp::new()) => {
                    if let Some(connection) = connection? {
                        break connection;
                    } else {
                        return Ok(());
                    };
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

        let address = connection.address().clone();
        let mut events = connection.events();
        let mut kbdnotify = connection.notification(&hogp::Token::Keyboard)?;
        let mut mousenotify = connection.notification(&hogp::Token::Mouse)?;
        let authnticator = connection.authenticator();
        if bonded(&store, &connection.address().clone().into()) {
            authnticator.mark_authenticated();
        }
        let gatt_loop = connection.run();

        gap::stop_advertising(&gap_client, device_id).await?;
        if bonded(&store, &address.clone().into()) {
            sink = InputSink::NotifyHost(&mut kbdnotify, &mut mousenotify);
            if grab {
                input.grab()?;
            }
        } else {
            sink = InputSink::Nop;
        }
        tokio::pin!(gatt_loop);

        loop {
            select! {
                r = Pin::new(&mut gatt_loop) => {
                    if let Err(err) = r {
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
                                    authnticator.mark_authenticated();
                                    sink = InputSink::NotifyHost(&mut kbdnotify, &mut mousenotify);
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
        drop(gatt_loop);
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
