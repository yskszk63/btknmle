use tokio::prelude::*;

use btknmle_server::mgmt;

pub async fn setup<IO>(mgmt: &mut mgmt::Mgmt<IO>) -> Result<(), mgmt::Error>
where
    IO: Sink<mgmt::model::MgmtCommand, Error = mgmt::Error>
        + Stream<Item = Result<mgmt::model::MgmtEvent, mgmt::Error>>
        + Unpin,
{
    mgmt.powered(false).await?;
    mgmt.low_energy(true).await?;
    mgmt.br_edr(false).await?;
    mgmt.secure_connections(mgmt::model::SecureConnections::Enabled)
        .await?;
    mgmt.io_capability(mgmt::model::IoCapability::DisplayOnly)
        .await?;
    mgmt.privacy(
        true,
        [
            0x34, 0xcd, 0x2d, 0xd3, 0x3d, 0x21, 0x4d, 0x16, 0xaa, 0x89, 0xfe, 0x48, 0xa8, 0xa7,
            0x77, 0x26,
        ],
    )
    .await?;
    mgmt.powered(true).await?;
    mgmt.appearance(960).await?;
    mgmt.local_name("my ble device", "mbd").await?;
    mgmt.connectable(true).await?;
    mgmt.bondable(true).await?;
    mgmt.discoverable(mgmt::model::Discoverable::General)
        .await?;
    mgmt.advertising(mgmt::model::Advertising::Connectable)
        .await?;

    Ok(())
}
