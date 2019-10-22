use std::io;

use bytes::Bytes;
use futures::{Sink, SinkExt as _, Stream, StreamExt as _};

use super::Database;
use crate::att::AttConnection;
use crate::gatt;
use crate::l2cap::SendError;
use crate::pkt::att::{self, Att};

#[derive(Debug)]
pub struct GattService {
    db: Database,
}

impl GattService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    async fn on_read_by_group_type<T>(
        &mut self,
        item: att::ReadByGroupTypeRequest,
        io: &mut AttConnection<T>,
    ) -> Result<(), ()>
    where
        T: Sink<Bytes, Error = SendError> + Stream + Unpin,
    {
        let response = self.db.read_by_group_type(
            item.starting_handle(),
            item.ending_handle(),
            item.attribute_group_type(),
        );
        match response {
            Ok(response) => {
                let mut iter = response.iter();
                let head = iter.next().unwrap();
                let mut b = att::ReadByGroupTypeResponse::builder(
                    (head.0).start().clone(),
                    (head.0).end().clone(),
                    head.1.clone(),
                );
                while let Some(item) = iter.next() {
                    b.add(
                        (item.0).start().clone(),
                        (item.0).end().clone(),
                        item.1.clone(),
                    );
                }
                io.send(b.build().into()).await.unwrap();
            }
            Err(gatt::Error::AttError(e)) => {
                let d = att::ErrorResponse::new(
                    0x10, //pkt::att::ReadByGroupTypeRequest::OPCODE,
                    item.starting_handle(),
                    e,
                );
                io.send(d.into()).await.unwrap();
            }
            Err(..) => {
                let d = att::ErrorResponse::new(
                    0x10, //pkt::att::ReadByGroupTypeRequest::OPCODE,
                    item.starting_handle(),
                    att::ErrorCode::AttributeNotFound,
                );
                io.send(d.into()).await.unwrap();
            }
        };
        Ok(())
    }

    async fn on_read_by_type<T>(
        &mut self,
        item: att::ReadByTypeRequest,
        io: &mut AttConnection<T>,
    ) -> Result<(), ()>
    where
        T: Sink<Bytes, Error = SendError> + Stream + Unpin,
    {
        let response = self.db.read_by_type(
            item.starting_handle(),
            item.ending_handle(),
            item.attribute_type(),
        );
        match response {
            Ok(response) => {
                let mut iter = response.iter();
                let head = iter.next().unwrap();
                let mut b = att::ReadByTypeResponse::builder(head.0.clone(), head.1.clone());
                while let Some(item) = iter.next() {
                    b.add(item.0.clone(), item.1.clone());
                }
                io.send(b.build().into()).await.unwrap();
            }
            Err(gatt::Error::AttError(e)) => {
                let d = att::ErrorResponse::new(
                    0x08, //pkt::att::ReadByTypeRequest::OPCODE,
                    item.starting_handle(),
                    e,
                );
                io.send(d.into()).await.unwrap();
            }
            Err(..) => {
                let d = att::ErrorResponse::new(
                    0x08, //pkt::att::ReadByTypeRequest::OPCODE,
                    item.starting_handle(),
                    att::ErrorCode::AttributeNotFound,
                );
                io.send(d.into()).await.unwrap();
            }
        };
        Ok(())
    }

    async fn on_findinformation<T>(
        &mut self,
        item: att::FindInformationRequest,
        io: &mut AttConnection<T>,
    ) -> Result<(), ()>
    where
        T: Sink<Bytes, Error = SendError> + Stream + Unpin,
    {
        let response = self
            .db
            .find_information(item.starting_handle(), item.ending_handle());

        match response {
            Ok(response) => {
                let mut iter = response.iter();
                let head = iter.next().unwrap();
                let mut b = att::FindInformationResponse::builder(head.0.clone(), head.1.clone());
                while let Some(item) = iter.next() {
                    b.add(item.0.clone(), item.1.clone());
                }
                io.send(b.build().into()).await.unwrap();
            }
            Err(gatt::Error::AttError(e)) => {
                let d = att::ErrorResponse::new(
                    0x05, //pkt::att::FindInformationResponse::OPCODE,
                    item.starting_handle(),
                    e,
                );
                io.send(d.into()).await.unwrap();
            }
            Err(..) => {
                let d = att::ErrorResponse::new(
                    0x05, //pkt::att::FindInformationResponse::OPCODE,
                    item.starting_handle(),
                    att::ErrorCode::UnlikelyError,
                );
                io.send(d.into()).await.unwrap();
            }
        };
        Ok(())
    }

    async fn on_read<T>(
        &mut self,
        item: att::ReadRequest,
        io: &mut AttConnection<T>,
    ) -> Result<(), ()>
    where
        T: Sink<Bytes, Error = SendError> + Stream + Unpin,
    {
        let response = self.db.read(item.attribute_handle());

        match response {
            Ok(response) => {
                let b = att::ReadResponse::new(response);
                io.send(b.into()).await.unwrap();
            }
            Err(gatt::Error::AttError(e)) => {
                let d = att::ErrorResponse::new(
                    0x0A, //pkt::att::ReadRequest::OPCODE,
                    item.attribute_handle(),
                    e,
                );
                io.send(d.into()).await.unwrap();
            }
            Err(..) => {
                let d = att::ErrorResponse::new(
                    0x0A, //pkt::att::ReadRequest::OPCODE,
                    item.attribute_handle(),
                    att::ErrorCode::UnlikelyError,
                );
                io.send(d.into()).await.unwrap();
            }
        };
        Ok(())
    }

    async fn on_read_blob<T>(
        &mut self,
        item: att::ReadBlobRequest,
        io: &mut AttConnection<T>,
    ) -> Result<(), ()>
    where
        T: Sink<Bytes, Error = SendError> + Stream + Unpin,
    {
        let response = self
            .db
            .read_blob(item.attribute_handle(), item.value_offset());

        match response {
            Ok(response) => {
                let b = att::ReadBlobResponse::new(response);
                io.send(b.into()).await.unwrap();
            }
            Err(gatt::Error::AttError(e)) => {
                let d = att::ErrorResponse::new(
                    0x0C, //pkt::att::ReadRequest::OPCODE,
                    item.attribute_handle(),
                    e,
                );
                io.send(d.into()).await.unwrap();
            }
            Err(..) => {
                let d = att::ErrorResponse::new(
                    0x0C, //pkt::att::ReadRequest::OPCODE,
                    item.attribute_handle(),
                    att::ErrorCode::UnlikelyError,
                );
                io.send(d.into()).await.unwrap();
            }
        };
        Ok(())
    }

    async fn on_write<T>(
        &mut self,
        item: att::WriteRequest,
        io: &mut AttConnection<T>,
    ) -> Result<(), ()>
    where
        T: Sink<Bytes, Error = SendError> + Stream + Unpin,
    {
        let response = self
            .db
            .write(item.attribute_handle(), item.attribute_value());

        match response {
            Some(_response) => {
                let b = att::WriteResponse::new();
                io.send(b.into()).await.unwrap();
            }
            None => {
                let d = att::ErrorResponse::new(
                    0x0A, //pkt::att::ReadRequest::OPCODE,
                    item.attribute_handle(),
                    att::ErrorCode::AttributeNotFound,
                );
                io.send(d.into()).await.unwrap();
            }
        };
        Ok(())
    }

    async fn on_exchange_mtu<T>(
        &mut self,
        item: att::ExchangeMtuRequest,
        io: &mut AttConnection<T>,
    ) -> Result<(), ()>
    where
        T: Sink<Bytes, Error = SendError> + Stream + Unpin,
    {
        let response = self.db.exchange_mtu(item.client_rx_mtu());

        match response {
            Ok(mtu) => {
                let b = att::ExchangeMtuResponse::new(mtu);
                io.send(b.into()).await.unwrap();
            }
            Err(..) => {
                let d = att::ErrorResponse::new(
                    0x02, //pkt::att::ReadRequest::OPCODE,
                    att::Handle::from(0),
                    att::ErrorCode::UnlikelyError,
                );
                io.send(d.into()).await.unwrap();
            }
        };
        Ok(())
    }

    pub async fn run<T>(mut self, io: T) -> Result<(), ()>
    where
        T: Sink<Bytes, Error = SendError> + Stream<Item = io::Result<Bytes>> + Unpin,
    {
        let mut io = AttConnection::new(io);
        while let Some(pkt) = io.next().await {
            match pkt.unwrap() {
                Att::ReadByGroupTypeRequest(item) => {
                    self.on_read_by_group_type(item, &mut io).await.unwrap()
                }
                Att::ReadByTypeRequest(item) => self.on_read_by_type(item, &mut io).await.unwrap(),
                Att::FindInformationRequest(item) => {
                    self.on_findinformation(item, &mut io).await.unwrap()
                }
                Att::ReadRequest(item) => self.on_read(item, &mut io).await.unwrap(),
                Att::ReadBlobRequest(item) => self.on_read_blob(item, &mut io).await.unwrap(),
                Att::WriteRequest(item) => self.on_write(item, &mut io).await.unwrap(),
                Att::ExchangeMtuRequest(item) => self.on_exchange_mtu(item, &mut io).await.unwrap(),
                _ => unimplemented!(),
            }
        }

        Ok(())
    }
}
