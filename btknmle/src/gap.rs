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
    mgmt.powered(true).await?;
    mgmt.local_name("my ble device", "mbd").await?;
    mgmt.connectable(true).await?;
    mgmt.bondable(true).await?;
    mgmt.discoverable(mgmt::model::Discoverable::General)
        .await?;
    mgmt.advertising(mgmt::model::Advertising::Enabled).await?;

    Ok(())
}
