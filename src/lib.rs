#![warn(clippy::all)]
use std::collections::HashMap;
use std::future;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;

use bdaddr::Address;
use btknmle_keydb::Store;
use btmgmt::client::Client as MgmtClient;
use btmgmt::packet::event::Event as MgmtEvent;
use btmgmt::packet::{command as cmd, ControllerIndex};
use futures_channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use futures_channel::oneshot::{self, Sender};
use futures_util::lock::Mutex;
use futures_util::{pin_mut, select, FutureExt, StreamExt, TryFutureExt};
use gatt::Server;

use crate::input::InputEvent;

mod gap;
mod hid;
mod hogp;
mod input;
mod sig;

fn authenticated(ltk: &btmgmt::packet::LongTermKey, addr: &Address) -> bool {
    match ltk.key_type() {
        btmgmt::packet::LongTermKeyType::AuthenticatedKey
        | btmgmt::packet::LongTermKeyType::AuthenticatedP256Key => {}
        _ => return false,
    }

    ltk.address().as_ref() == addr
}

fn bonded(store: &Store, addr: &Address) -> bool {
    store.iter_ltks().any(|ltk| authenticated(ltk, addr))
}

async fn store_keys(
    device_id: ControllerIndex,
    gap: &MgmtClient,
    mut store: Store,
    mut auth_channel: UnboundedReceiver<(Address, Sender<()>)>,
) -> anyhow::Result<()> {
    let events = gap.events().await;
    let mut events = events
        .filter_map(|(idx, evt)| future::ready((idx == device_id).then(|| evt)))
        .fuse();
    let mut pendings = HashMap::<Address, Sender<()>>::new();

    loop {
        select! {
            item = events.next() => {
                let event = if let Some(event) = item {
                    event
                } else {
                    return Ok(());
                };

                match event {
                    MgmtEvent::NewLongTermKey(evt) => {
                        if *evt.store_hint() {
                            store.add_ltk(evt.key().clone()).await?;

                            // TODO
                            // 1. Get identity resolving key by identity key.
                            // 2. Iterate pending address, wich is Resolvable random address.
                            // 3. If matches this event address. Notify authorized.
                            if let Some(sender) = pendings.remove(evt.key().address()) {
                                log::debug!("New bonded for {}", evt.key().address());
                                sender.send(()).ok();
                            }
                        }
                    }

                    MgmtEvent::NewIdentityResolvingKey(evt) => {
                        if *evt.store_hint() {
                            store.add_irk(evt.key().clone()).await?;
                        }
                    }

                    _ => {}
                }
            },

            item = auth_channel.next() => {
                let (addr, sender) = if let Some((addr, sender)) = item {
                    (addr, sender)
                } else {
                    return Ok(());
                };

                if bonded(&store, &addr) {
                    sender.send(()).ok();
                } else {
                    log::debug!("Pending for {}", addr);
                    pendings.insert(addr, sender);
                }
            },
        }
    }
}

fn add_passkey(buf: &mut u32, kbstat: &input::kbstat::KbStat) -> bool {
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
            KEY_ENTER => return true,
            _ => return false,
        };
        *buf = buf.wrapping_mul(10) + k;
    }
    false
}

async fn fill_passkey(input: &mut input::InputSource) -> anyhow::Result<u32> {
    let mut passkey = 0;
    while let Some(input_event) = input.next().await {
        let kbstat = if let input::InputEvent::Keyboard(kbstat) = input_event? {
            kbstat
        } else {
            continue;
        };
        if add_passkey(&mut passkey, &kbstat) {
            return Ok(passkey);
        }
    }
    anyhow::bail!("failed to fill passkey.")
}

async fn passkey_input(
    device_id: ControllerIndex,
    gap: &MgmtClient,
    input: Arc<Mutex<input::InputSource>>,
) -> anyhow::Result<()> {
    let events = gap.events().await;
    let mut events = events.filter_map(|(idx, evt)| future::ready((idx == device_id).then(|| evt)));

    while let Some(event) = events.next().await {
        match event {
            MgmtEvent::UserPasskeyRequest(event) => {
                log::trace!("begin passkey input.");
                let mut input = if let Some(input) = input.try_lock() {
                    input
                } else {
                    let msg = cmd::UserPasskeyNegativeReply::new(
                        event.address().clone(),
                        event.address_type().clone(),
                    );
                    gap.call(device_id.clone(), msg).await?;
                    break;
                };
                let passkey = fill_passkey(&mut input).await?;
                let msg = cmd::UserPasskeyReply::new(
                    event.address().clone(),
                    event.address_type().clone(),
                    passkey,
                );
                gap.call(device_id.clone(), msg).await?;
            }
            _ => {}
        }
    }

    Ok(())
}

async fn advertising(
    device_id: ControllerIndex,
    gap: &MgmtClient,
    input: Arc<Mutex<input::InputSource>>,
) -> anyhow::Result<()> {
    log::info!("Start advertising.");
    gap::start_advertising(&gap, device_id.clone()).await?;

    // TODO controll advertising.
    future::pending::<()>().await;
    Ok(())
}

struct InputSourceWrapper<'a> {
    inner: &'a mut input::InputSource,
    grab: bool,
}

impl<'a> InputSourceWrapper<'a> {
    fn with(inner: &'a mut input::InputSource, grab: bool) -> io::Result<Self> {
        if grab {
            inner.grab()?;
        }
        Ok(Self { inner, grab })
    }

    async fn next(&mut self) -> Option<io::Result<InputEvent>> {
        self.inner.next().await
    }
}

impl<'a> Drop for InputSourceWrapper<'a> {
    fn drop(&mut self) {
        if self.grab {
            self.inner.ungrab().ok();
        }
    }
}

async fn gatt_loop(
    grab: bool,
    input: Arc<Mutex<input::InputSource>>,
    auth_channel: UnboundedSender<(Address, Sender<()>)>,
) -> anyhow::Result<()> {
    let mut server = Server::bind()?;
    server.needs_bond_mitm()?;

    log::info!("Start serving.");

    while let Some(connection) = server.accept(hogp::new()).await? {
        let addr = connection.address().clone();
        let authenticator = connection.authenticator();

        let mut kbdnotify = connection.notification(&hogp::Token::Keyboard)?;
        let mut mousenotify = connection.notification(&hogp::Token::Mouse)?;

        let task = connection.run().fuse();
        pin_mut!(task);

        let kbtask = async move {
            let (reply_tx, reply_rx) = oneshot::channel();
            auth_channel.unbounded_send((addr.clone(), reply_tx))?;

            reply_rx.await?;
            log::debug!("Authenticated {}", addr);
            authenticator.mark_authenticated();

            let mut input = input.lock().await;
            let mut input = InputSourceWrapper::with(&mut input, grab)?;

            while let Some(event) = input.next().await {
                match event? {
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

            anyhow::Result::<()>::Ok(())
        }
        .fuse();
        pin_mut!(kbtask);

        loop {
            select! {
                result = task => {
                    if let Err(err) = result {
                        // may be connection terminated by remote host.
                        log::info!("{}", err);
                    }
                    return Ok(());
                },

                result = kbtask => {
                    result?;
                },
            }
        }
    }
    Ok(())
}

#[allow(unused)]
pub async fn run(var_file: PathBuf, device_id: u16, grab: bool) -> anyhow::Result<()> {
    let io_capability = btmgmt::packet::IoCapability::KeyboardOnly;

    let store = Store::open(var_file).await?;
    let gap_client = gap::setup(device_id, &store, io_capability).await?;

    let input = input::InputSource::new()?;
    let input = Arc::new(Mutex::new(input));

    let (auth_tx, auth_rx) = mpsc::unbounded();
    let mut sig = sig::Sig::new()?;

    log::info!("starting.");
    tokio::try_join!(
        store_keys(device_id.into(), &gap_client, store, auth_rx),
        passkey_input(device_id.into(), &gap_client, input.clone()),
        advertising(device_id.into(), &gap_client, input.clone()),
        gatt_loop(grab, input, auth_tx),
        sig.recv().map_err(Into::<anyhow::Error>::into),
    )?;
    Ok(())
}
