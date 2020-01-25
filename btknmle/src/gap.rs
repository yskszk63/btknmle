use futures::{Sink, Stream};

use btknmle_keydb::KeyDb;
use btknmle_server::mgmt;

pub async fn setup<IO>(mgmt: &mut mgmt::Mgmt<IO>, db: &mut KeyDb) -> Result<(), mgmt::Error>
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
    mgmt.local_name("my ble device", "mbd").await?;
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
