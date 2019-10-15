use std::io;
use std::sync::Arc;

use bytes::{BytesMut, IntoBuf};
use futures::{SinkExt as _, StreamExt as _};
use tokio::codec::{Decoder, Encoder};
use tokio::sync::Mutex;
use either::{Left, Right};

use btknmle_pkt::{self as pkt, Codec as _, HciPacket};
use btknmle_sock::{HciFramed, HciSocket};

mod l2cap;
mod att;
mod gatt;

struct PacketCodec;

impl Encoder for PacketCodec {
    type Item = HciPacket;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        item.write_to(buf).unwrap();
        Ok(())
    }
}

impl Decoder for PacketCodec {
    type Item = HciPacket;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let item = HciPacket::parse(&mut buf.take().into_buf()).unwrap();
        Ok(Some(item))
    }
}

#[tokio::main(single_thread)]
async fn main() {
    let sock = HciSocket::bind(0).unwrap();
    let mut frames = HciFramed::new(sock, PacketCodec);

    let pkt = HciPacket::Command(pkt::command::host_ctl::Reset::new().into());
    frames.send(pkt).await.unwrap();
    let pkt = frames.next().await.unwrap();
    println!("{:?}", &pkt);

    let mut b = [0; 31];
    b[0] = 0x02;
    b[1] = 0x01;
    b[2] = 0x06;
    let pkt = HciPacket::Command(pkt::command::le_ctl::LeSetAdvertisingData::new(3, b).into());
    frames.send(pkt).await.unwrap();
    let pkt = frames.next().await.unwrap();
    println!("{:?}", &pkt);

    let pkt = HciPacket::Command(pkt::command::le_ctl::LeSetAdvertiseEnable::new(true).into());
    frames.send(pkt).await.unwrap();
    let pkt = frames.next().await.unwrap();
    println!("{:?}", &pkt);

    let mut builder = gatt::Database::builder();

    builder.begin_service(pkt::att::Uuid::Uuid16(0x1800));
    builder.with_characteristic(
        gatt::CharacteristicProperties::READ, pkt::att::Uuid::Uuid16(0x2A00), "MYDEVICENAME0123456789ABCDEF");
    builder.with_characteristic(
        gatt::CharacteristicProperties::READ, pkt::att::Uuid::Uuid16(0x2A01), vec![0xC2, 0x03]); // HID mouse

    builder.begin_service(pkt::att::Uuid::Uuid16(0x1801));
    builder.with_characteristic(
        gatt::CharacteristicProperties::INDICATE, pkt::att::Uuid::Uuid16(0x2A05), "");
    builder.with_user_description("HELLO WORLD!".into());
    builder.with_cccd(gatt::CCCD::empty());

    builder.begin_service(pkt::att::Uuid::Uuid16(0x180A));
    builder.with_characteristic(
        gatt::CharacteristicProperties::READ, pkt::att::Uuid::Uuid16(0x2A29), "MYMANUFACTURE");
    builder.with_characteristic(
        gatt::CharacteristicProperties::READ, pkt::att::Uuid::Uuid16(0x2A24), "1234");
    builder.with_characteristic(
        gatt::CharacteristicProperties::READ, pkt::att::Uuid::Uuid16(0x2A24), "9999");

    builder.begin_service(pkt::att::Uuid::Uuid16(0x180F));
    let bash = builder.with_characteristic(
        gatt::CharacteristicProperties::INDICATE, pkt::att::Uuid::Uuid16(0x2A19), vec![100]);
    builder.with_cccd(gatt::CCCD::empty());

    let mut gatt = builder.build();
    let transport = att::AttTransport::new(l2cap::L2CapTransport::new(frames));
    let (tx, mut rx) = transport.split();
    let tx = Arc::new(Mutex::new(tx));

    let tx2 = tx.clone();
    tokio::runtime::current_thread::spawn(async move {
        let mut interval = tokio::timer::Interval::new_interval(std::time::Duration::from_secs(1));
        while let Some(..) = interval.next().await {
            let b = pkt::att::HandleValueNotification::new(bash.clone(), vec![(std::time::UNIX_EPOCH.elapsed().unwrap().as_secs() % 100) as u8]);
            let d = pkt::att::Att::from(b);
            tx2.lock().await.send((l2cap::Handle(16), d)).await.unwrap();
            //println!("{:?}", g);
        }
    });

    loop {
        let p = rx.next().await.unwrap().unwrap();
        match p {
            Left((handle, data)) => {
                println!("{:?} {:?}", handle, data);
                let mut tx = tx.lock().await;

                match data {
                    pkt::att::Att::ReadByGroupTypeRequest(item) => {
                        let response = gatt.read_by_group_type(
                            item.starting_handle(),
                            item.ending_handle(),
                            item.attribute_group_type());

                        match response {
                            Ok(response) => {
                                let mut iter = response.iter();
                                let head = iter.next().unwrap();
                                let mut b = pkt::att::ReadByGroupTypeResponse::builder(
                                    (head.0).start().clone(), (head.0).end().clone(), head.1.clone());
                                while let Some(item) = iter.next() {
                                    b.add((item.0).start().clone(), (item.0).end().clone(), item.1.clone());
                                };
                                let d = pkt::att::Att::from(b.build());
                                tx.send((handle, d)).await.unwrap();
                            },
                            Err(gatt::Error::AttError(e)) => {
                                let d = pkt::att::ErrorResponse::new(
                                    0x10,//pkt::att::ReadByGroupTypeRequest::OPCODE,
                                    item.starting_handle(),
                                    e);
                                let d = pkt::att::Att::from(d);
                                tx.send((handle, d)).await.unwrap();
                            },
                            Err(..) => {
                                let d = pkt::att::ErrorResponse::new(
                                    0x10,//pkt::att::ReadByGroupTypeRequest::OPCODE,
                                    item.starting_handle(),
                                    pkt::att::ErrorCode::AttributeNotFound);
                                let d = pkt::att::Att::from(d);
                                tx.send((handle, d)).await.unwrap();
                            },
                        };
                    },

                    pkt::att::Att::ReadByTypeRequest(item) => {
                        let response = gatt.read_by_type(
                            item.starting_handle(),
                            item.ending_handle(),
                            item.attribute_type());

                        match response {
                            Ok(response) => {
                                let mut iter = response.iter();
                                let head = iter.next().unwrap();
                                let mut b = pkt::att::ReadByTypeResponse::builder(
                                    head.0.clone(), head.1.clone());
                                while let Some(item) = iter.next() {
                                    b.add(item.0.clone(), item.1.clone());
                                };
                                let d = pkt::att::Att::from(b.build());
                                tx.send((handle, d)).await.unwrap();
                            },
                            Err(gatt::Error::AttError(e)) => {
                                let d = pkt::att::ErrorResponse::new(
                                    0x08,//pkt::att::ReadByTypeRequest::OPCODE,
                                    item.starting_handle(),
                                    e);
                                let d = pkt::att::Att::from(d);
                                tx.send((handle, d)).await.unwrap();
                            },
                            Err(..) => {
                                let d = pkt::att::ErrorResponse::new(
                                    0x08,//pkt::att::ReadByTypeRequest::OPCODE,
                                    item.starting_handle(),
                                    pkt::att::ErrorCode::AttributeNotFound);
                                let d = pkt::att::Att::from(d);
                                tx.send((handle, d)).await.unwrap();
                            },
                        };
                    },

                    pkt::att::Att::FindInformationRequest(item) => {
                        let response = gatt.find_information(
                            item.starting_handle(),
                            item.ending_handle());

                        match response {
                            Ok(response) => {
                                let mut iter = response.iter();
                                let head = iter.next().unwrap();
                                let mut b = pkt::att::FindInformationResponse::builder(
                                    head.0.clone(), head.1.clone());
                                while let Some(item) = iter.next() {
                                    b.add(item.0.clone(), item.1.clone());
                                };
                                let d = pkt::att::Att::from(b.build());
                                tx.send((handle, d)).await.unwrap();
                            },
                            Err(gatt::Error::AttError(e)) => {
                                let d = pkt::att::ErrorResponse::new(
                                    0x05,//pkt::att::FindInformationResponse::OPCODE,
                                    item.starting_handle(),
                                    e);
                                let d = pkt::att::Att::from(d);
                                tx.send((handle, d)).await.unwrap();
                            },
                            Err(..) => {
                                let d = pkt::att::ErrorResponse::new(
                                    0x05,//pkt::att::FindInformationResponse::OPCODE,
                                    item.starting_handle(),
                                    pkt::att::ErrorCode::UnlikelyError);
                                let d = pkt::att::Att::from(d);
                                tx.send((handle, d)).await.unwrap();
                            },
                        }
                    },

                    pkt::att::Att::ReadRequest(item) => {
                        let response = gatt.read(item.attribute_handle());

                        match response {
                            Ok(response) => {
                                let b = pkt::att::ReadResponse::new(response);
                                let d = pkt::att::Att::from(b);
                                tx.send((handle, d)).await.unwrap();
                            },
                            Err(gatt::Error::AttError(e)) => {
                                let d = pkt::att::ErrorResponse::new(
                                    0x0A,//pkt::att::ReadRequest::OPCODE,
                                    item.attribute_handle(),
                                    e);
                                let d = pkt::att::Att::from(d);
                                tx.send((handle, d)).await.unwrap();
                            },
                            Err(..) => {
                                let d = pkt::att::ErrorResponse::new(
                                    0x0A,//pkt::att::ReadRequest::OPCODE,
                                    item.attribute_handle(),
                                    pkt::att::ErrorCode::UnlikelyError);
                                let d = pkt::att::Att::from(d);
                                tx.send((handle, d)).await.unwrap();
                            },
                        };
                    },

                    pkt::att::Att::ReadBlobRequest(item) => {
                        let response = gatt.read_blob(item.attribute_handle(), item.value_offset());

                        match response {
                            Ok(response) => {
                                let b = pkt::att::ReadBlobResponse::new(response);
                                let d = pkt::att::Att::from(b);
                                tx.send((handle, d)).await.unwrap();
                            },
                            Err(gatt::Error::AttError(e)) => {
                                let d = pkt::att::ErrorResponse::new(
                                    0x0C,//pkt::att::ReadRequest::OPCODE,
                                    item.attribute_handle(),
                                    e);
                                let d = pkt::att::Att::from(d);
                                tx.send((handle, d)).await.unwrap();
                            },
                            Err(..) => {
                                let d = pkt::att::ErrorResponse::new(
                                    0x0C,//pkt::att::ReadRequest::OPCODE,
                                    item.attribute_handle(),
                                    pkt::att::ErrorCode::UnlikelyError);
                                let d = pkt::att::Att::from(d);
                                tx.send((handle, d)).await.unwrap();
                            },
                        };
                    },

                    pkt::att::Att::WriteRequest(item) => {
                        let response = gatt.write(item.attribute_handle(), item.attribute_value());

                        match response {
                            Some(_response) => {
                                let b = pkt::att::WriteResponse::new();
                                let d = pkt::att::Att::from(b);
                                tx.send((handle, d)).await.unwrap();
                            },
                            None => {
                                let d = pkt::att::ErrorResponse::new(
                                    0x0A,//pkt::att::ReadRequest::OPCODE,
                                    item.attribute_handle(),
                                    pkt::att::ErrorCode::AttributeNotFound);
                                let d = pkt::att::Att::from(d);
                                tx.send((handle, d)).await.unwrap();
                            },
                        };
                    },

                    pkt::att::Att::ExchangeMtuRequest(item) => {
                        let response = gatt.exchange_mtu(item.client_rx_mtu());

                        match response {
                            Ok(mtu) => {
                                let b = pkt::att::ExchangeMtuResponse::new(mtu);
                                let d = pkt::att::Att::from(b);
                                tx.send((handle, d)).await.unwrap();
                            },
                            Err(..) => {
                                let d = pkt::att::ErrorResponse::new(
                                    0x02,//pkt::att::ReadRequest::OPCODE,
                                    pkt::att::Handle::from(0),
                                    pkt::att::ErrorCode::UnlikelyError);
                                let d = pkt::att::Att::from(d);
                                tx.send((handle, d)).await.unwrap();
                            },
                        }
                    },

                    x => unimplemented!("{:?}", x),
                };

                /*
                let b = pkt::att::HandleValueNotification::new(bash.clone(), vec![(std::time::UNIX_EPOCH.elapsed().unwrap().as_secs() % 100) as u8]);
                let d = pkt::att::Att::from(b);
                transport.send((l2cap::Handle(16), d)).await.unwrap();
                */
            },
            Right(e) => {
                println!("{:?}", e);
            }
        }
    }
}
