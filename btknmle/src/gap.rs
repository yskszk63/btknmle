use std::path::Path;

use futures::stream::StreamExt as _;
use futures::{Sink, Stream};

use btknmle_keydb::KeyDb;
use btknmle_server::mgmt;
use btknmle_server::mgmt::model::MgmtEvent;
use btknmle_server::mgmt::MgmtCodec;
use btknmle_server::sock::{Framed, MgmtSocket};

#[derive(Debug)]
pub struct Gap<F> {
    mgmt: mgmt::Mgmt<Framed<MgmtSocket, MgmtCodec>>,
    db: KeyDb,
    passkey_notify: F,
}

impl<F> Gap<F>
where
    F: FnMut(&str),
{
    pub async fn setup<P>(devid: u16, varfile: P, passkey_notify: F) -> Result<Self, mgmt::Error>
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
            passkey_notify,
        })
    }

    pub async fn run(self) -> Result<(), mgmt::Error> {
        let Self {
            mut mgmt,
            mut db,
            mut passkey_notify,
        } = self;

        while let Some(evt) = mgmt.next().await {
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
                MgmtEvent::PasskeyNotifyEvent(evt) => {
                    let passkey = evt.passkey();
                    passkey_notify(&passkey.to_string());
                }
                MgmtEvent::UserConfirmationRequestEvent(evt) => {
                    log::debug!("{:?}", evt);
                    mgmt.user_confirmation(evt.address(), evt.address_type())
                        .await?;
                    //sock.user_confirmation_negative(evt.address(), evt.address_type()).await?;
                }
                evt => log::debug!("UNHANDLED {:?}", evt),
            }
        }

        Ok(())
    }
}

async fn setup<IO>(mgmt: &mut mgmt::Mgmt<IO>, db: &mut KeyDb) -> Result<(), mgmt::Error>
where
    IO: Sink<mgmt::model::MgmtCommand, Error = mgmt::Error>
        + Stream<Item = Result<mgmt::model::MgmtEvent, mgmt::Error>>
        + Unpin,
{
    let local_irk = db.load_local_irk().await?;

    mgmt.powered(false).await?;
    mgmt.low_energy(true).await?;
    mgmt.br_edr(false).await?;
    mgmt.secure_connections(mgmt::model::SecureConnections::Enabled)
        .await?;
    mgmt.io_capability(mgmt::model::IoCapability::DisplayOnly)
        .await?;
    mgmt.privacy(true, local_irk).await?;
    mgmt.powered(true).await?;
    mgmt.appearance(960).await?;
    mgmt.local_name("btknmle", "btknmle").await?;
    mgmt.connectable(true).await?;
    mgmt.bondable(true).await?;
    mgmt.discoverable(mgmt::model::Discoverable::General)
        .await?;
    mgmt.advertising(mgmt::model::Advertising::Connectable)
        .await?;

    mgmt.load_irks(db.load_irks().await?).await?;
    mgmt.load_ltks(db.load_ltks().await?).await?;

    Ok(())
}
