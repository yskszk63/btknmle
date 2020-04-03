use std::io;
use std::sync::Arc;

use bytes::Bytes;
use futures::stream::StreamExt as _;
use futures::{Sink, Stream};
use thiserror::Error;
use tokio::fs;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

pub use btknmle_pkt::{Uuid, Uuid16};

use crate::mgmt;
use crate::mgmt::model::{
    command::{IoCapability, MgmtCommand, SecureConnections},
    event::MgmtEvent,
    Address, AddressType, AdvertisingFlags,
};
use crate::mgmt::MgmtCodec;
use crate::sock::{Framed, MgmtSocket};
use crate::KeyStore;

#[derive(Debug)]
struct ChangeAdvInterval {
    devid: u16,
    adv_max_interval: Option<String>,
    adv_min_interval: Option<String>,
}

impl ChangeAdvInterval {
    async fn new(devid: u16, min: usize, max: usize) -> io::Result<Self> {
        let prefix = "/sys/kernel/debug/bluetooth";

        let path = format!("{}/hci{}/adv_min_interval", prefix, devid);
        let adv_min_interval = fs::read_to_string(&path).await.ok();
        if adv_min_interval.is_some() {
            let min = format!("{}", min);
            fs::write(&path, min).await?;
        }

        let path = format!("{}/hci{}/adv_max_interval", prefix, devid);
        let adv_max_interval = fs::read_to_string(&path).await.ok();
        if adv_max_interval.is_some() {
            let max = format!("{}", max);
            fs::write(&path, max).await?;
        }

        Ok(ChangeAdvInterval {
            devid,
            adv_max_interval,
            adv_min_interval,
        })
    }

    async fn close(mut self) {
        let prefix = "/sys/kernel/debug/bluetooth";
        let devid = self.devid;
        if let Some(val) = self.adv_max_interval.take() {
            let path = format!("{}/hci{}/adv_max_interval", prefix, devid);
            fs::write(path, val).await.ok();
        }
        if let Some(val) = self.adv_min_interval.take() {
            let path = format!("{}/hci{}/adv_min_interval", prefix, devid);
            fs::write(path, val).await.ok();
        }
    }
}

impl Drop for ChangeAdvInterval {
    fn drop(&mut self) {
        let prefix = "/sys/kernel/debug/bluetooth";
        let devid = self.devid;
        if let Some(val) = &self.adv_max_interval {
            std::fs::write(format!("{}/hci{}/adv_max_interval", prefix, devid), val).ok();
        }
        if let Some(val) = &self.adv_min_interval {
            std::fs::write(format!("{}/hci{}/adv_min_interval", prefix, devid), val).ok();
        }
    }
}

#[async_trait::async_trait]
pub trait GapCallback: Send + Sync + 'static {
    async fn passkey_request(&mut self) -> String;
    async fn device_connected(&mut self);
    async fn device_disconnected(&mut self);
    async fn start_advertise(&mut self);
    async fn end_advertise(&mut self);
}

#[derive(Debug)]
enum AdvCtrlMessage {
    StartAdv,
    CancelAdv,
}

#[derive(Error, Debug)]
pub enum AdvCtrlError {
    #[error("failed to send")]
    SendError,
}

#[derive(Debug, Clone)]
pub struct AdvCtrl {
    channel: mpsc::Sender<AdvCtrlMessage>,
}

impl AdvCtrl {
    pub async fn start_advertise(&mut self) -> Result<(), AdvCtrlError> {
        self.channel
            .send(AdvCtrlMessage::StartAdv)
            .await
            .map_err(|_| AdvCtrlError::SendError)?;
        Ok(())
    }

    pub async fn cancel_advertise(&mut self) -> Result<(), AdvCtrlError> {
        self.channel
            .send(AdvCtrlMessage::CancelAdv)
            .await
            .map_err(|_| AdvCtrlError::SendError)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Gap<K, C>
where
    C: GapCallback,
{
    devid: u16,
    mgmt: mgmt::Mgmt<Framed<MgmtSocket, MgmtCodec>>,
    keystore: K,
    callback: Arc<Mutex<C>>,
    scan_data: Vec<u8>,
    adv_ctrl_rx: mpsc::Receiver<AdvCtrlMessage>,
    adv_ctrl: AdvCtrl,
    connected: bool,
    advertising: bool,
}

impl<K, C> Gap<K, C>
where
    C: GapCallback,
    K: KeyStore,
{
    pub async fn setup(
        devid: u16,
        adv_uuid: Uuid,
        local_name: &str,
        short_local_name: &str,
        mut keystore: K,
        callback: C,
    ) -> Result<Self, mgmt::Error> {
        let (adv_tx, adv_ctrl_rx) = mpsc::channel(1);
        let adv_ctrl = AdvCtrl { channel: adv_tx };

        let mut mgmt = mgmt::Mgmt::new(devid).await?;
        let callback = Arc::new(Mutex::new(callback));

        let scan_data = match adv_uuid {
            Uuid::Uuid16(uuid) => {
                let uuid = Bytes::from(Uuid16::from(uuid));
                let mut scan_data = vec![];
                scan_data.push((uuid.len() + 1) as u8);
                scan_data.push(0x03);
                scan_data.extend(uuid);
                scan_data
            }
            _ => unimplemented!(),
        };

        setup(&mut mgmt, &mut keystore, local_name, short_local_name).await?;

        let mut gap = Gap {
            devid,
            mgmt,
            keystore,
            callback,
            scan_data,
            adv_ctrl_rx,
            adv_ctrl,
            connected: false,
            advertising: false,
        };
        gap.start_advertise().await?;
        Ok(gap)
    }

    pub fn adv_ctrl(&self) -> AdvCtrl {
        self.adv_ctrl.clone()
    }

    async fn start_advertise(&mut self) -> Result<(), mgmt::Error> {
        if self.connected || self.advertising {
            return Ok(());
        }

        let mut callback = self.callback.lock().await;
        callback.start_advertise().await;

        self.advertising = true;

        let chinterval = ChangeAdvInterval::new(self.devid, 244, 338).await?; // 152.5ms 211.25ms
        let add_adv_result = self
            .mgmt
            .add_advertising(
                1,
                AdvertisingFlags::SWITCH_INTO_CONNECTABLE_MODE
                    | AdvertisingFlags::ADVERTISE_AS_LIMITED_DISCOVERABLE
                    | AdvertisingFlags::ADD_FLAGS_FIELD_TO_ADV_DATA
                    | AdvertisingFlags::ADD_APPEARANCE_FIELD_TO_SCAN_RSP
                    | AdvertisingFlags::ADD_LOCAL_NAME_IN_SCAN_RSP,
                0,
                60,
                [].as_ref(),
                self.scan_data.as_ref(),
            )
            .await;
        chinterval.close().await;
        add_adv_result?;
        Ok(())
    }

    async fn cancel_advertise(&mut self) -> Result<(), mgmt::Error> {
        self.mgmt.remove_advertising(None).await?;
        Ok(())
    }

    async fn process_evt(
        &mut self,
        evt: MgmtEvent,
        tx: &mpsc::Sender<(Address, AddressType, u32)>,
    ) -> Result<(), mgmt::Error> {
        match evt {
            MgmtEvent::NewLongTermKeyEvent(_, evt) => {
                if evt.store_hint() {
                    let key = evt.key();
                    log::debug!("{:?}", key);
                    self.keystore.store_ltks(key.clone()).await?;
                }
            }
            MgmtEvent::NewIdentityResolvingKeyEvent(_, evt) => {
                if evt.store_hint() {
                    let key = evt.key();
                    log::debug!("{:?}", key);
                    self.keystore.store_irks(key.clone()).await?;
                }
            }
            MgmtEvent::UserConfirmationRequestEvent(_, evt) => {
                log::debug!("{:?}", evt);
                //mgmt.user_confirmation(evt.address(), evt.address_type()).await?;
                //sock.user_confirmation_negative(evt.address(), evt.address_type()).await?;
            }
            MgmtEvent::UserPasskeyRequestEvent(_, evt) => {
                log::debug!("{:?}", evt);
                let mut tx = tx.clone();
                let callback = self.callback.clone();
                tokio::spawn(async move {
                    let mut callback = callback.lock().await;
                    let passkey = callback.passkey_request().await;
                    let passkey = passkey.parse().unwrap();
                    tx.send((evt.address(), evt.address_type(), passkey))
                        .await
                        .unwrap();
                });
            }
            MgmtEvent::DeviceConnectedEvent(..) => {
                self.connected = true;
                self.cancel_advertise().await?;

                let mut callback = self.callback.lock().await;
                callback.device_connected().await;
            }
            MgmtEvent::DeviceDisconnectedEvent(..) => {
                self.connected = false;
                self.start_advertise().await?;
                let mut callback = self.callback.lock().await;
                callback.device_disconnected().await;
            }
            MgmtEvent::AdvertisingRemovedEvent(..) => {
                let mut callback = self.callback.lock().await;
                callback.end_advertise().await;

                self.advertising = false;
            }
            evt => log::debug!("UNHANDLED {:?}", evt),
        }
        Ok(())
    }

    pub async fn run(mut self) -> Result<(), mgmt::Error> {
        let (tx, mut rx) = mpsc::channel(1);

        loop {
            tokio::select! {
                Some(evt) = self.mgmt.next() => self.process_evt(evt?, &tx).await?,
                Some((addr, addr_t, passkey)) = rx.recv() => {
                    self.mgmt.user_passkey_reply(addr, addr_t, passkey).await?;
                }
                Some(msg) = self.adv_ctrl_rx.recv() => {
                    match msg {
                        AdvCtrlMessage::StartAdv => self.start_advertise().await?,
                        AdvCtrlMessage::CancelAdv => self.cancel_advertise().await?,
                    }
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
    mgmt.appearance(0x03c0).await?; // HID Generic
    mgmt.local_name(local_name, short_local_name).await?;
    mgmt.bondable(true).await?;
    mgmt.connectable(false).await?;

    mgmt.load_irks(irks).await?;
    mgmt.load_ltks(ltks).await?;

    mgmt.powered(true).await?;

    let info = mgmt.read_controller_information().await?;
    Ok(info.address())
}
