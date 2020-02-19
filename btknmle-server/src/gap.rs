use std::path::Path;
use std::future::Future;
use std::sync::Arc;

use futures::stream::StreamExt as _;
use futures::{Sink, Stream};
use tokio::sync::mpsc;
use tokio::sync::Mutex;

use btknmle_keydb::KeyDb;
use crate::mgmt;
use crate::mgmt::model::{
    AdvertisingFlags, IoCapability, MgmtCommand, MgmtEvent,
    SecureConnections, Address,
};
use crate::mgmt::MgmtCodec;
use crate::sock::{Framed, MgmtSocket};

#[derive(Debug)]
pub struct Gap<C, F> where F: Future<Output=String>, C: FnMut() -> F {
    mgmt: mgmt::Mgmt<Framed<MgmtSocket, MgmtCodec>>,
    db: KeyDb,
    passkey_callback: Arc<Mutex<C>>,
}

impl<C, F> Gap<C, F> where F: Future<Output=String> + Send, C: (FnMut() -> F) + Send + Sync + 'static {
    pub async fn setup<P>(
        devid: u16,
        varfile: P,
        passkey_callback: C,
    ) -> Result<Self, mgmt::Error>
    where
        P: AsRef<Path>,
    {
        let varfile = varfile.as_ref().to_owned();
        let mut db = KeyDb::new(varfile).await?;
        let mut mgmt = mgmt::Mgmt::new(devid).await?;
        let passkey_callback = Arc::new(Mutex::new(passkey_callback));

        setup(&mut mgmt, &mut db).await?;

        Ok(Gap {
            mgmt,
            db,
            passkey_callback,
        })
    }

    pub async fn run(self) -> Result<(), mgmt::Error> {
        let Self {
            mut mgmt,
            mut db,
            passkey_callback,
        } = self;

        let (tx, mut rx) = mpsc::channel(1);

        loop {
            tokio::select! {
                Some(evt) = mgmt.next() => {
            match evt? {
                MgmtEvent::NewLongTermKeyEvent(evt) => {
                    if evt.store_hint() {
                        let key = evt.key();
                        log::debug!("{:?}", key);
                        db.store_ltks(key.clone()).await?;
                    }
                }
                MgmtEvent::NewIdentityResolvingKeyEvent(evt) => {
                    if evt.store_hint() {
                        let key = evt.key();
                        log::debug!("{:?}", key);
                        db.store_irks(key.clone()).await?;
                    }
                }
                MgmtEvent::UserConfirmationRequestEvent(evt) => {
                    log::debug!("{:?}", evt);
                    //mgmt.user_confirmation(evt.address(), evt.address_type()).await?;
                    //sock.user_confirmation_negative(evt.address(), evt.address_type()).await?;
                }
                MgmtEvent::UserPasskeyRequestEvent(evt) => {
                    log::debug!("{:?}", evt);
                    let mut tx = tx.clone();
                    let passkey_callback = passkey_callback.clone();
                    tokio::spawn(async move {
                        use std::ops::DerefMut;
                        let mut passkey_callback = passkey_callback.lock().await;
                        let passkey = (passkey_callback.deref_mut())().await;
                        let passkey = passkey.parse().unwrap();
                        tx.send((evt.address(), evt.address_type(), passkey)).await.unwrap();
                    });
                }
                evt => log::debug!("UNHANDLED {:?}", evt),
            }
                }
                Some((addr, addr_t, passkey)) = rx.recv() => {
                    mgmt.user_passkey_reply(addr, addr_t, passkey).await?;
                }
                else => break
            }
        }

        Ok(())
    }
}

async fn setup<IO>(mgmt: &mut mgmt::Mgmt<IO>, db: &mut KeyDb) -> Result<Address, mgmt::Error>
where
    IO: Sink<MgmtCommand, Error = mgmt::Error>
        + Stream<Item = Result<MgmtEvent, mgmt::Error>>
        + Unpin,
{
    let local_irk = db.load_local_irk().await?;

    mgmt.powered(false).await?;
    mgmt.low_energy(true).await?;
    mgmt.br_edr(false).await?;
    mgmt.secure_connections(SecureConnections::Enabled).await?;
    mgmt.io_capability(IoCapability::KeyboardOnly).await?;
    mgmt.privacy(true, local_irk).await?;
    mgmt.local_name("btknmle", "btknmle").await?;
    mgmt.bondable(true).await?;
    mgmt.connectable(false).await?;

    mgmt.add_advertising(
        1,
        AdvertisingFlags::SWITCH_INTO_CONNECTABLE_MODE
            | AdvertisingFlags::ADVERTISE_AS_DISCOVERABLE
            | AdvertisingFlags::ADD_FLAGS_FIELD_TO_ADV_DATA
            | AdvertisingFlags::ADD_APPEARANCE_FIELD_TO_SCAN_RSP
            | AdvertisingFlags::ADD_LOCAL_NAME_IN_SCAN_RSP,
        0,
        0,
        [].as_ref(),
        [0x07, 0x03, 0x0f, 0x18, 0x0a, 0x18, 0x12, 0x18].as_ref(), // complete uuid16 [180f 180a 1812]
    )
    .await?;

    mgmt.load_irks(db.load_irks().await?).await?;
    mgmt.load_ltks(db.load_ltks().await?).await?;

    mgmt.powered(true).await?;

    let info = mgmt.read_controller_information().await?;
    Ok(info.address())
}
