use bytes::{BytesMut, IntoBuf};
use tokio::codec::{Decoder, Encoder};
use tokio::prelude::*;

use btknmle_pkt as pkt;
use pkt::att::Att;
use pkt::mgmt::{self, MgmtCommand, MgmtEvent};
use pkt::{Codec as _};

mod gatt;

struct MgmtCodec;

impl Encoder for MgmtCodec {
    type Item = MgmtCommand;
    type Error = failure::Error; // FIXME

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        item.write_to(dst)?;
        Ok(())
    }
}

impl Decoder for MgmtCodec {
    type Item = MgmtEvent;
    type Error = failure::Error; // FIXME

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let result = Self::Item::parse(&mut buf.take().into_buf())?;
        Ok(Some(result))
    }
}

struct AttCodec;

impl Encoder for AttCodec {
    type Item = Att;
    type Error = std::io::Error; // FIXME

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        item.write_to(dst).unwrap();
        Ok(())
    }
}

impl Decoder for AttCodec {
    type Item = Att;
    type Error = std::io::Error; // FIXME

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let result = Self::Item::parse(&mut buf.take().into_buf()).unwrap();
        Ok(Some(result))
    }
}

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
async fn main() -> Result<(), failure::Error> {
    let mgmtsock = btknmle_sock::MgmtSocket::bind()?;
    let mut mgmtsock = mgmtsock.framed(MgmtCodec);

    mgmtsock
        .send(mgmt::SetPoweredCommand::new(0, true).into())
        .await?;
    let res = mgmtsock.next().await.unwrap().unwrap();
    println!("{:?}", res);

    mgmtsock
        .send(mgmt::SetLocalNameCommand::new(0, "my name", "mnm").into())
        .await?;
    let res = mgmtsock.next().await.unwrap().unwrap();
    println!("{:?}", res);

    mgmtsock
        .send(mgmt::SetConnectableCommand::new(0, true).into())
        .await?;
    let res = mgmtsock.next().await.unwrap().unwrap();
    println!("{:?}", res);

    mgmtsock
        .send(mgmt::SetBondableCommand::new(0, true).into())
        .await?;
    let res = mgmtsock.next().await.unwrap().unwrap();
    println!("{:?}", res);

    mgmtsock
        .send(mgmt::SetLowEnergyCommand::new(0, true).into())
        .await?;
    let res = mgmtsock.next().await.unwrap().unwrap();
    println!("{:?}", res);

    mgmtsock
        .send(mgmt::SetAdvertisingCommand::new(0, mgmt::Advertising::Connectable).into())
        .await?;
    let res = mgmtsock.next().await.unwrap().unwrap();
    println!("{:?}", res);

    let mut l2server = btknmle_sock::L2Listener::bind(0x0004)?.incoming();
    while let Some(sock) = l2server.next().await {
        let connection = sock?.framed(AttCodec);
        let db = database();
        let svc = gatt::GattService::new(db);
        svc.run(connection).await.unwrap();
    }

    Ok(())
}
