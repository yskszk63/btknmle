use std::future::Future;
use std::sync::Arc;

use bytes::Bytes;
use futures::stream::StreamExt as _;
use futures::{Sink, Stream};
use tokio::sync::mpsc;
use tokio::sync::Mutex;

pub use btknmle_pkt::{Uuid, Uuid16};

use crate::mgmt;
use crate::mgmt::model::{
    Address, AdvertisingFlags, IoCapability, MgmtCommand, MgmtEvent, SecureConnections,
};
use crate::mgmt::MgmtCodec;
use crate::sock::{Framed, MgmtSocket};
use crate::KeyStore;

#[derive(Debug)]
pub struct Gap<K, C, F>
where
    F: Future<Output = String>,
    C: FnMut() -> F,
{
    mgmt: mgmt::Mgmt<Framed<MgmtSocket, MgmtCodec>>,
    keystore: K,
    passkey_callback: Arc<Mutex<C>>,
    scan_data: Vec<u8>,
}

impl<K, C, F> Gap<K, C, F>
where
    F: Future<Output = String> + Send,
    C: (FnMut() -> F) + Send + Sync + 'static,
    K: KeyStore,
{
    pub async fn setup(
        devid: u16,
        adv_uuid: Uuid,
        local_name: &str,
        short_local_name: &str,
        mut keystore: K,
        passkey_callback: C,
    ) -> Result<Self, mgmt::Error> {
        let mut mgmt = mgmt::Mgmt::new(devid).await?;
        let passkey_callback = Arc::new(Mutex::new(passkey_callback));

        let scan_data = match adv_uuid {
            Uuid::Uuid16(uuid) => {
                let uuid = Bytes::from(Uuid16::from(uuid));
                let mut scan_data = vec![];
                scan_data.push((uuid.len() + 1) as u8);
                scan_data.push(0x03);
                scan_data.extend(uuid);
                scan_data
            }
            _ => todo!(),
        };

        setup(
            &mut mgmt,
            &mut keystore,
            &scan_data,
            local_name,
            short_local_name,
        )
        .await?;

        Ok(Gap {
            mgmt,
            keystore,
            passkey_callback,
            scan_data,
        })
    }

    pub async fn run(self) -> Result<(), mgmt::Error> {
        let Self {
            mut mgmt,
            mut keystore,
            passkey_callback,
            scan_data,
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
                        keystore.store_ltks(key.clone()).await?;
                    }
                }
                MgmtEvent::NewIdentityResolvingKeyEvent(evt) => {
                    if evt.store_hint() {
                        let key = evt.key();
                        log::debug!("{:?}", key);
                        keystore.store_irks(key.clone()).await?;
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
                MgmtEvent::DeviceConnectedEvent(..) => {
                    mgmt.remove_advertising(None).await?;
                }
                MgmtEvent::DeviceDisconnectedEvent(..) => {
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
                        scan_data.as_ref(),
                    )
                    .await?;
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

async fn setup<IO, K>(
    mgmt: &mut mgmt::Mgmt<IO>,
    keystore: &mut K,
    scan_data: &[u8],
    local_name: &str,
    short_local_name: &str,
) -> Result<Address, mgmt::Error>
where
    IO: Sink<MgmtCommand, Error = mgmt::Error>
        + Stream<Item = Result<MgmtEvent, mgmt::Error>>
        + Unpin,
    K: KeyStore,
{
    let local_irk = keystore.load_local_irk().await?;
    let irks = keystore.load_irks().await?;
    let ltks = keystore.load_ltks().await?;

    mgmt.powered(false).await?;
    mgmt.low_energy(true).await?;
    mgmt.br_edr(false).await?;
    mgmt.secure_connections(SecureConnections::Enabled).await?;
    mgmt.io_capability(IoCapability::KeyboardOnly).await?;
    mgmt.privacy(true, local_irk).await?;
    mgmt.local_name(local_name, short_local_name).await?;
    mgmt.bondable(true).await?;
    mgmt.connectable(false).await?;

    mgmt.load_irks(irks).await?;
    mgmt.load_ltks(ltks).await?;

    mgmt.powered(true).await?;

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
        scan_data.as_ref(),
    )
    .await?;

    let info = mgmt.read_controller_information().await?;
    Ok(info.address())
}
