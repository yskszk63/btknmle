use std::convert::TryFrom;
use std::path::Path;

use futures::stream::StreamExt as _;
use futures::{Sink, Stream};
use tokio::sync::mpsc;

use btknmle_keydb::KeyDb;
use btknmle_server::mgmt;
use btknmle_server::mgmt::model::{
    Action, Address, AddressType, AdvertisingFlags, IoCapability, MgmtCommand, MgmtEvent,
    SecureConnections,
};
use btknmle_server::mgmt::MgmtCodec;
use btknmle_server::sock::{Framed, MgmtSocket};

use crate::input::PasskeyFilter;

#[derive(Debug)]
pub struct Gap {
    mgmt: mgmt::Mgmt<Framed<MgmtSocket, MgmtCodec>>,
    db: KeyDb,
    passkey_filter: PasskeyFilter,
}

impl Gap {
    pub async fn setup<P>(
        devid: u16,
        varfile: P,
        passkey_filter: PasskeyFilter,
    ) -> Result<Self, mgmt::Error>
    where
        P: AsRef<Path>,
    {
        let varfile = varfile.as_ref().to_owned();
        let mut db = KeyDb::new(varfile).await?;
        let mut mgmt = mgmt::Mgmt::new(devid).await?;

        setup(&mut mgmt, &mut db).await?;

        Ok(Gap {
            mgmt,
            db,
            passkey_filter,
        })
    }

    pub async fn run(self) -> Result<(), mgmt::Error> {
        let Self {
            mut mgmt,
            mut db,
            passkey_filter,
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
                    let passkey_filter = passkey_filter.clone();
                    let mut tx = tx.clone();
                    tokio::spawn(async move {
                        let mut buf = String::new();
                        let mut rx = passkey_filter.subscribe();
                        while let Ok(key) = rx.recv().await {
                            match key {
                                b @ b'0' ..= b'9' => buf.push(b.into()),
                                b'\n' => break,
                                b => log::debug!("ignore {}", b),
                            }
                        }
                        let passkey = buf.parse().unwrap();
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

async fn setup<IO>(mgmt: &mut mgmt::Mgmt<IO>, db: &mut KeyDb) -> Result<(), mgmt::Error>
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

    let irks = db.load_irks().await?;
    mgmt.remove_device(
        Address::try_from("00:00:00:00:00:00".to_string()).unwrap(),
        AddressType::LePublic,
    )
    .await?;
    for irk in &irks {
        mgmt.add_device(
            irk.address(),
            irk.address_type(),
            Action::AllowIncommingConnection,
        )
        .await?;
    }
    mgmt.load_irks(irks).await?;
    mgmt.load_ltks(db.load_ltks().await?).await?;

    mgmt.powered(true).await?;

    Ok(())
}
