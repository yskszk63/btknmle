use tokio::prelude::*;

use btknmle_server::{gatt, mgmt};

fn database() -> (gatt::Database, gatt::model::Handle, gatt::model::Handle) {
    let mut builder = gatt::Database::builder();

    builder.begin_service(gatt::model::Uuid::Uuid16(0x1800));
    builder.with_characteristic(
        gatt::CharacteristicProperties::READ,
        gatt::model::Uuid::Uuid16(0x2A00),
        "MYDEVICENAME0123456789ABCDEF",
    );
    builder.with_characteristic(
        gatt::CharacteristicProperties::READ,
        gatt::model::Uuid::Uuid16(0x2A01),
        vec![0xC2, 0x03],
    ); // HID mouse

    builder.begin_service(gatt::model::Uuid::Uuid16(0x1801));
    builder.with_characteristic(
        gatt::CharacteristicProperties::INDICATE,
        gatt::model::Uuid::Uuid16(0x2A05),
        "",
    );
    builder.with_user_description("HELLO WORLD!".into());
    builder.with_cccd(gatt::CCCD::empty());

    builder.begin_service(gatt::model::Uuid::Uuid16(0x180A));
    builder.with_characteristic(
        gatt::CharacteristicProperties::READ,
        gatt::model::Uuid::Uuid16(0x2A29),
        "MYMANUFACTURE",
    );
    builder.with_characteristic(
        gatt::CharacteristicProperties::READ,
        gatt::model::Uuid::Uuid16(0x2A24),
        "1234",
    );
    let zzz = builder.with_characteristic(
        gatt::CharacteristicProperties::READ,
        gatt::model::Uuid::Uuid16(0x2A24),
        "9999",
    );

    builder.begin_service(gatt::model::Uuid::Uuid16(0x180F));
    let bash = builder.with_characteristic(
        gatt::CharacteristicProperties::INDICATE,
        gatt::model::Uuid::Uuid16(0x2A19),
        vec![100],
    );
    builder.with_cccd(gatt::CCCD::empty());

    (builder.build(), bash, zzz)
}

#[tokio::main(single_thread)]
async fn main() -> Result<(), failure::Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    let mut mgmt = mgmt::Mgmt::new(0).await?;
    mgmt.powered(false).await?;
    mgmt.low_energy(true).await?;
    mgmt.br_edr(false).await?;
    mgmt.powered(true).await?;
    mgmt.local_name("my ble device", "mbd").await?;
    mgmt.connectable(true).await?;
    mgmt.bondable(true).await?;
    mgmt.advertising(mgmt::model::Advertising::Enabled).await?;

    let (db, _handle, _h2) = database();
    let mut listener = gatt::GattListener::new(db)?;
    while let Some(sock) = listener.next().await {
        match sock {
            Ok(svc) => {
                tokio::spawn(async move {
                    /*
                    let mut h2 = svc.writed_for(&h2).unwrap();
                    tokio::spawn(async move {
                        while let Some(b) = h2.next().await {
                            log::debug!("{:?}", b);
                        }
                    });
                    */

                    /*
                    let mut battery_level = svc.notify_for(&handle).unwrap();
                    tokio::spawn(async move {
                        let mut n = 0u8;
                        loop {
                            let when = tokio::clock::now() + std::time::Duration::from_secs(1);
                            tokio::timer::delay(when).await;
                            if let Err(e) = battery_level.send(vec![n]).await {
                                log::warn!("{}", e);
                                break
                            }
                            n = (n + 1) % 100;
                        }
                    });
                    */

                    match svc.run().await {
                        Ok(()) => {}
                        Err(e) => log::warn!("{}", e),
                    }
                    log::debug!("done");
                });
            }
            Err(e) => log::warn!("{}", e),
        }
    }

    Ok(())
}
