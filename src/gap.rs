use btknmle_keydb::Store;
use btmgmt::client::Client;
use btmgmt::packet::ControllerIndex;
use btmgmt::packet::{
    command as cmd, AdvDataScanResp, AdvertisingFlag, IoCapability, Privacy, SecureConnections,
    Settings, SystemConfigurationParameter,
};

pub(crate) async fn setup(
    devid: u16,
    store: &Store,
    io_capability: IoCapability,
) -> anyhow::Result<Client> {
    let client = Client::open()?;

    let info = client.call(devid, cmd::ReadControllerInformation).await?;
    let mut current_settings = *info.current_settings();

    if current_settings.contains(Settings::Powered) {
        current_settings = *client.call(devid, cmd::SetPowered::new(false)).await?;
    }
    if !current_settings.contains(Settings::LowEnergy) {
        current_settings = *client.call(devid, cmd::SetLowEnergy::new(true)).await?;
    }
    if current_settings.contains(Settings::BasicRateEnhancedDataRate) {
        current_settings = *client.call(devid, cmd::SetBrEdr::new(false)).await?;
    }
    if !current_settings.contains(Settings::SecureConnections) {
        current_settings = *client
            .call(
                devid,
                cmd::SetSecureConnections::new(SecureConnections::Enable),
            )
            .await?;
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
        current_settings = *client.call(devid, cmd::SetBondable::new(true)).await?;
    }
    if current_settings.contains(Settings::Connectable) {
        current_settings = *client.call(devid, cmd::SetConnectable::new(false)).await?;
    }
    log::debug!("current settings: {:?}", current_settings);

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

    Ok(client)
}

pub(crate) async fn start_advertising(
    client: &Client,
    devid: ControllerIndex,
    timeout: u16,
) -> anyhow::Result<()> {
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
                timeout,
                AdvDataScanResp::new(vec![], vec![0x03, 0x03, 0x12, 0x18]),
            ),
        )
        .await?;
    Ok(())
}

pub(crate) async fn stop_advertising(
    client: &Client,
    devid: ControllerIndex,
) -> anyhow::Result<()> {
    client
        .call(devid, cmd::RemoveAdvertising::new(1.into()))
        .await?;
    Ok(())
}

pub(crate) async fn is_advertising_enabled(
    client: &Client,
    devid: ControllerIndex,
) -> anyhow::Result<bool> {
    let info = client.call(devid, cmd::ReadAdvertisingFeature).await?;
    let mut instances = info.instances().iter();
    Ok(instances.next().is_some())
}
