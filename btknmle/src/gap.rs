use tokio::task::JoinHandle;

use btknmle_keydb::Store;
use btmgmt::client::TaskError;
use btmgmt::{
    command as cmd, event::Event, AdvertisingFlag, Client, IoCapability, Privacy,
    SecureConnections, Settings, SystemConfigurationParameter,
};

pub(crate) async fn setup(
    devid: u16,
    store: &Store,
    io_capability: IoCapability,
) -> anyhow::Result<(JoinHandle<Result<(), TaskError>>, Client)> {
    let (client, handle) = Client::open()?;

    let info = client
        .call(devid, cmd::ReadControllerInformation::new())
        .await?;
    let mut current_settings = *info.current_settings();

    if current_settings.contains(Settings::Powered) {
        current_settings = *client
            .call(devid, cmd::SetPowered::new(false))
            .await?
            .current_settings();
    }
    if !current_settings.contains(Settings::LowEnergy) {
        current_settings = *client
            .call(devid, cmd::SetLowEnergy::new(true))
            .await?
            .current_settings();
    }
    if current_settings.contains(Settings::BasicRateEnhancedDataRate) {
        current_settings = *client
            .call(devid, cmd::SetBrEdr::new(false))
            .await?
            .current_settings();
    }
    if !current_settings.contains(Settings::SecureConnections) {
        current_settings = *client
            .call(
                devid,
                cmd::SetSecureConnections::new(SecureConnections::Enable),
            )
            .await?
            .current_settings();
    }
    client
        .call(devid, cmd::SetIoCapability::new(io_capability))
        .await?;
    client
        .call(
            devid,
            cmd::SetPrivacy::new(Privacy::Enable, *store.key_for_resolvable_private_address()),
        )
        .await?;

    client.call(devid, cmd::SetApperance::new(0x03c0)).await?;
    client
        .call(
            devid,
            cmd::SetLocalName::new("btknmle".parse()?, "btknmle".parse()?),
        )
        .await?;

    if !current_settings.contains(Settings::Bondable) {
        current_settings = *client
            .call(devid, cmd::SetBondable::new(true))
            .await?
            .current_settings();
    }
    if current_settings.contains(Settings::Connectable) {
        client
            .call(devid, cmd::SetConnectable::new(false))
            .await?
            .current_settings();
    }

    client
        .call(
            devid,
            store
                .iter_irks()
                .cloned()
                .collect::<cmd::LoadIdentityResolvingKeys>(),
        )
        .await?;
    client
        .call(
            devid,
            store.iter_ltks().cloned().collect::<cmd::LoadLongTermKey>(),
        )
        .await?;

    client
        .call(
            devid,
            vec![
                SystemConfigurationParameter::LEAdvertisementMinInterval(224), // 140ms
                SystemConfigurationParameter::LEAdvertisementMaxInterval(338), // 211.25ms
            ]
            .into_iter()
            .collect::<cmd::SetDefaultSystemConfiguration>(),
        )
        .await?;

    client.call(devid, cmd::SetPowered::new(true)).await?;

    Ok((handle, client))
}

pub(crate) async fn start_advertising(client: &Client, devid: u16) -> anyhow::Result<()> {
    log::info!("Start advertising.");
    client
        .call(
            devid,
            cmd::AddAdvertising::new(
                1.into(),
                AdvertisingFlag::SwitchIntoConnectableMode
                    | AdvertisingFlag::AdvertiseAsLimitedDiscoverable
                    | AdvertisingFlag::AddFlagsFieldToAdvData
                    | AdvertisingFlag::AddAppearanceFieldToScanResp
                    | AdvertisingFlag::AddLocalNameInScanResp,
                0,
                60,
                vec![],
                vec![0x03, 0x03, 0x12, 0x18],
            ),
        )
        .await?;
    Ok(())
}

pub(crate) async fn stop_advertising(client: &Client, devid: u16) -> anyhow::Result<()> {
    client
        .call(devid, cmd::RemoveAdvertising::new(1.into()))
        .await?;
    Ok(())
}

pub(crate) async fn handle_event<'a>(
    devid: u16,
    client: &'a Client,
    evt: &Event,
    store: &mut Store,
    sink: &mut super::InputSink<'a, '_>,
) {
    match evt {
        Event::NewLongTermKey(evt) => {
            if *evt.store_hint() {
                store.add_ltk(evt.key().clone()).await.unwrap();
            }
        }

        Event::NewIdentityResolvingKey(evt) => {
            if *evt.store_hint() {
                store.add_irk(evt.key().clone()).await.unwrap();
            }
        }

        Event::AdvertisingRemoved(..) => {
            if matches!(sink, super::InputSink::Nop) {
                *sink = super::InputSink::StartAdvertising(client, devid);
            }
        }

        Event::UserConfirmationRequest(evt) => {
            client
                .call(
                    devid,
                    cmd::UserConfirmationNegativeReply::new(
                        evt.address().clone(),
                        evt.address_type().clone(),
                    ),
                )
                .await
                .unwrap();
        }

        Event::UserPasskeyRequest(evt) => {
            *sink = super::InputSink::PasskeyInput(client, devid, evt.clone(), 0);
        }

        x => println!("{:?}", x),
    }
}
