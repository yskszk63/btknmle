use std::convert::TryInto as _;

use failure::Error;

use btknmle_pkt as pkt;

mod att;
mod gatt;
mod hci;
mod l2cap;
mod util;

fn database() -> gatt::Database {
    let mut builder = gatt::Database::builder();

    builder.begin_service(pkt::att::Uuid::Uuid16(0x1800));
    builder.with_characteristic(
        gatt::CharacteristicProperties::READ,
        pkt::att::Uuid::Uuid16(0x2A00),
        "MYDEVICENAME0123456789ABCDEF",
    );
    builder.with_characteristic(
        gatt::CharacteristicProperties::READ,
        pkt::att::Uuid::Uuid16(0x2A01),
        vec![0xC2, 0x03],
    ); // HID mouse

    builder.begin_service(pkt::att::Uuid::Uuid16(0x1801));
    builder.with_characteristic(
        gatt::CharacteristicProperties::INDICATE,
        pkt::att::Uuid::Uuid16(0x2A05),
        "",
    );
    builder.with_user_description("HELLO WORLD!".into());
    builder.with_cccd(gatt::CCCD::empty());

    builder.begin_service(pkt::att::Uuid::Uuid16(0x180A));
    builder.with_characteristic(
        gatt::CharacteristicProperties::READ,
        pkt::att::Uuid::Uuid16(0x2A29),
        "MYMANUFACTURE",
    );
    builder.with_characteristic(
        gatt::CharacteristicProperties::READ,
        pkt::att::Uuid::Uuid16(0x2A24),
        "1234",
    );
    builder.with_characteristic(
        gatt::CharacteristicProperties::READ,
        pkt::att::Uuid::Uuid16(0x2A24),
        "9999",
    );

    builder.begin_service(pkt::att::Uuid::Uuid16(0x180F));
    let _bash = builder.with_characteristic(
        gatt::CharacteristicProperties::INDICATE,
        pkt::att::Uuid::Uuid16(0x2A19),
        vec![100],
    );
    builder.with_cccd(gatt::CCCD::empty());

    builder.build()
}

#[tokio::main(single_thread)]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    let transport = hci::HciTransport::new(0)?;
    let mut server = l2cap::L2capServer::new(transport);

    server
        .inner_mut()
        .invoke(pkt::hci::command::host_ctl::Reset::new())
        .await?;

    let adv = pkt::adv::AdvertiseList::new(vec![
        (pkt::adv::Flags::LE_GENERAL_DISCOVERABLE_MODE | pkt::adv::Flags::BR_EDR_NOT_SUPPORTED)
            .into(),
        //pkt::adv::IncompleteListUuid16::new(vec![0x180F]).into(),
        //pkt::adv::CompleteListUuid16::new(vec![0x180A]).into(),
        //pkt::adv::IncompleteListUuid128::new(vec![0x180F]).into(),
        //pkt::adv::CompleteListUuid128::new(vec![0x180A]).into(),
        pkt::adv::ShortenedLocalName::new("btknmle").into(),
        pkt::adv::CompleteLocalName::new("btknmle").into(),
        //pkt::adv::TxPower::new(127).into(),
        pkt::adv::Appearance::new(0x03C2).into(),
    ]);
    let (n, b) = adv.try_into().unwrap();

    let adv_data = pkt::hci::command::le_ctl::LeSetAdvertisingData::new(n, b);
    server.inner_mut().invoke(adv_data).await?;

    loop {
        server
            .inner_mut()
            .invoke(pkt::hci::command::le_ctl::LeSetAdvertiseEnable::new(true))
            .await?;
        server
            .serve(|mut connection| match connection.cid() {
                0x0004 => {
                    tokio::spawn(async move {
                        let db = database();
                        let svc = gatt::GattService::new(db);
                        svc.run(connection).await.unwrap();
                    });
                }

                0x0006 => {
                    tokio::spawn(async move {
                        use bytes::IntoBuf;
                        use futures::SinkExt;
                        use futures::StreamExt;
                        use pkt::Codec;
                        while let Some(v) = connection.next().await {
                            log::debug!("{:?}", v);
                            let x = pkt::smp::Smp::parse(&mut v.unwrap().into_buf()).unwrap();
                            log::debug!("{:?}", x);
                            match x {
                                pkt::smp::Smp::PairingRequest(_v) => {
                                    let k = pkt::smp::LeKeyDistribution::from_bits_truncate(7);
                                    //let x = pkt::smp::PairingResponse::new(0x00, 0x00, 33, 16, k, k);
                                    let x =
                                        pkt::smp::PairingResponse::new(0x03, 0x00, 33, 16, k, k);
                                    //let x = pkt::smp::PairingResponse::new(0x04, 0x00, 45, 16, k,
                                    //k);//nosc
                                    let x = pkt::smp::Smp::from(x);
                                    let mut b = bytes::BytesMut::new();
                                    x.write_to(&mut b).unwrap();
                                    connection.send(b.freeze()).await.unwrap();
                                }
                                pkt::smp::Smp::PairingConfirm(v) => {
                                    let x = pkt::smp::Smp::from(v);
                                    let mut b = bytes::BytesMut::new();
                                    x.write_to(&mut b).unwrap();
                                    connection.send(b.freeze()).await.unwrap();
                                }
                                _ => {}
                            }
                        }
                    });
                }
                _ => {}
            })
            .await?;
    }
}
