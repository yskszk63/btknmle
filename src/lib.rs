#![warn(clippy::all)]
use std::future;
use std::path::PathBuf;
use std::sync::Arc;

use btknmle_keydb::Store;
use btmgmt::client::{Client as GapClient, EventSubscribe};
use btmgmt::packet::event::Event as MgmtEvent;
use btmgmt::packet::{command as cmd, ControllerIndex};
use futures_util::lock::Mutex;
use futures_util::{StreamExt, TryFutureExt};
use gatt::server::Connection;
use gatt::Server;

use crate::input::InputEvent;

mod gap;
mod hid;
mod hogp;
mod input;
mod sig;

async fn store_keys(
    device_id: u16,
    events: EventSubscribe,
    store: Arc<Mutex<Store>>,
) -> anyhow::Result<()> {
    let device_id = device_id.into();
    let mut events = events.filter_map(|(idx, evt)| future::ready((idx == device_id).then(|| evt)));

    while let Some(event) = events.next().await {
        match event {
            MgmtEvent::NewLongTermKey(evt) => {
                if *evt.store_hint() {
                    let mut store = store.lock().await;
                    store.add_ltk(evt.key().clone()).await?;
                }
            }

            MgmtEvent::NewIdentityResolvingKey(evt) => {
                if *evt.store_hint() {
                    let mut store = store.lock().await;
                    store.add_irk(evt.key().clone()).await?;
                }
            }

            _ => {}
        }
    }

    Ok(())
}

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

fn add_passkey(buf: u32, kbstat: &input::kbstat::KbStat) -> (bool, u32) {
    use hid::KeyboardUsageId::*;

    if let Some(key) = kbstat.keys().iter().next() {
        let k = match key {
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
            KEY_ENTER => return (true, buf),
            _ => return (false, buf),
        };
        (false, buf.wrapping_mul(10) + k)
    } else {
        (false, buf)
    }
}

async fn passkey_input(
    client: &GapClient,
    device_id: ControllerIndex,
    input: &mut input::InputSource,
) -> anyhow::Result<()> {
    let events = client.events().await;
    let mut events = events.filter_map(|(idx, evt)| future::ready((idx == device_id).then(|| evt)));
    let mut passkey_buf = None;

    let mut adv_enabled = gap::is_advertising_enabled(client, device_id.clone()).await?;

    loop {
        tokio::select! {
            event = events.next() => {
                let event = if let Some(event) = event {
                    event
                } else {
                    anyhow::bail!("unexpect eof.");
                };

                match event {
                    MgmtEvent::UserConfirmationRequest(event) => {
                        client
                            .call(
                                device_id.clone(),
                                cmd::UserConfirmationNegativeReply::new(
                                    event.address().clone(),
                                    event.address_type().clone(),
                                ),
                            )
                            .await?;
                    }

                    MgmtEvent::UserPasskeyRequest(event) => {
                        passkey_buf = Some((event, 0));
                        log::trace!("begin passkey input.");
                    }

                    MgmtEvent::AdvertisingRemoved(..) => {
                        log::info!("advertising stopped by timeout.");
                        adv_enabled = false;
                    }

                    _ => {}
                }
            }

            event = input.next() => {
                let event = if let Some(event) = event {
                    event?
                } else {
                    anyhow::bail!("unexpect eof.");
                };

                if let input::InputEvent::Keyboard(kbstat) = event {
                    if let Some((req, passkey)) = passkey_buf {
                        let (done, passkey) = add_passkey(passkey, &kbstat);
                        if done {
                            client
                                .call(
                                    device_id.clone(),
                                    cmd::UserPasskeyReply::new(
                                        req.address().clone(),
                                        req.address_type().clone(),
                                        passkey,
                                    ),
                                )
                                .await?;
                            passkey_buf = None;
                            log::trace!("passkey done.");
                        } else {
                            passkey_buf = Some((req, passkey))
                        }
                    } else if !adv_enabled {
                        log::info!("Start advertising.");
                        gap::start_advertising(client, device_id.clone()).await?;
                        adv_enabled = true;
                    }
                }
            }
        }
    }
}

async fn accept(
    server: &mut Server,
    gap_client: &GapClient,
    input: &mut input::InputSource,
    device_id: ControllerIndex,
) -> anyhow::Result<Option<Connection<hogp::Token>>> {
    let registration = hogp::new();

    let accept = server.accept(registration);
    tokio::pin!(accept);

    let passkey_input = passkey_input(gap_client, device_id, input);
    tokio::pin!(passkey_input);

    loop {
        tokio::select! {
            connection = Pin::new(&mut accept) => {
                let connection = connection?;
                return Ok(connection);
            }

            result = Pin::new(&mut passkey_input) => {
                result?;
            }
        }
    }
}

async fn gatt_loop(
    store: Arc<Mutex<Store>>,
    device_id: ControllerIndex,
    grab: bool,
    gap_client: GapClient,
) -> anyhow::Result<()> {
    let mut server = Server::bind()?;
    server.needs_bond_mitm()?;

    let mut input = input::InputSource::new()?;

    log::info!("Start serving.");

    log::info!("Start advertising.");
    gap::start_advertising(&gap_client, device_id.clone()).await?;
    while let Some(connection) =
        accept(&mut server, &gap_client, &mut input, device_id.clone()).await?
    {
        let addr = connection.address();
        log::info!("Connection accepted {}", addr);

        log::info!("Stop advertising.");
        gap::stop_advertising(&gap_client, device_id.clone()).await?;

        let store = store.lock().await;
        if bonded(&store, &addr.clone().into()) {
            connection.authenticator().mark_authenticated();
        } else {
            anyhow::bail!("not bounded");
        }

        let mut kbdnotify = connection.notification(&hogp::Token::Keyboard)?;
        let mut mousenotify = connection.notification(&hogp::Token::Mouse)?;

        let task = connection.run();
        tokio::pin!(task);

        if grab {
            log::info!("Grab inputs.");
            input.grab()?;
        }

        loop {
            tokio::select! {
                result = Pin::new(&mut task) => {
                    if let Err(err) = result {
                        // may be connection terminated by remote host.
                        log::info!("{}", err);
                    }
                    break;
                }

                event = input.next() => {
                    let event = if let Some(event) = event {
                        event?
                    } else {
                        anyhow::bail!("unexpected end of input.");
                    };

                    match event {
                        InputEvent::Keyboard(evt) => {
                            if let Err(err) = evt.write_to(&mut kbdnotify).await {
                                // may be connection terminated by remote host.
                                log::info!("{}", err);
                                break;
                            }
                        }
                        InputEvent::Mouse(evt) => {
                            if let Err(err) = evt.write_to(&mut mousenotify).await {
                                // may be connection terminated by remote host.
                                log::info!("{}", err);
                                break;
                            }
                        }
                    }
                }
            }
        }

        if grab {
            log::info!("Ungrab inputs.");
            input.ungrab()?;
        }
        log::info!("Start advertising.");
        gap::start_advertising(&gap_client, device_id.clone()).await?;
    }

    Ok(())
}

pub async fn run(var_file: PathBuf, device_id: u16, grab: bool) -> anyhow::Result<()> {
    let io_capability = btmgmt::packet::IoCapability::KeyboardOnly;

    let store = Store::open(var_file).await?;
    let gap_client = gap::setup(device_id, &store, io_capability).await?;
    let store = Arc::new(Mutex::new(store));

    let mut sig = sig::Sig::new()?;

    log::info!("starting.");
    tokio::try_join!(
        store_keys(device_id, gap_client.events().await, store.clone()),
        gatt_loop(store, device_id.into(), grab, gap_client),
        sig.recv().map_err(Into::<anyhow::Error>::into),
    )?;

    Ok(())
}
