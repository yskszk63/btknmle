use std::collections::HashMap;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;
use futures::channel::mpsc;
use futures::{Sink, SinkExt as _, Stream, StreamExt as _};

use super::{Database, Result};
use crate::gatt;
use crate::pkt::att::{self, Att};

#[derive(Debug)]
pub struct Notify {
    handle: att::Handle,
    tx: mpsc::Sender<Att>,
}

impl Notify {
    pub async fn send(&mut self, data: impl Into<Bytes>) -> Result<()> {
        let data = att::HandleValueNotification::new(self.handle.clone(), data.into());
        self.tx.send(data.into()).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Writed {
    rx: mpsc::Receiver<Bytes>,
}

impl Stream for Writed {
    type Item = Bytes;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.rx).poll_next(cx)
    }
}

#[derive(Debug)]
pub struct GattConnection<IO> {
    db: Database,
    io: IO,
    rx: mpsc::Receiver<Att>,
    tx: mpsc::Sender<Att>,
    listeners: HashMap<att::Handle, mpsc::Sender<Bytes>>,
}

impl<IO> GattConnection<IO>
where
    IO: Stream,
{
    pub fn new(db: Database, io: IO) -> Self {
        let (tx, rx) = mpsc::channel(2);
        Self {
            db,
            io,
            rx,
            tx,
            listeners: HashMap::new(),
        }
    }
}

impl<IO> GattConnection<IO>
where
    IO: Sink<Att, Error = io::Error> + Stream<Item = io::Result<Att>> + Unpin,
{
    pub fn notify_for(&self, handle: &att::Handle) -> Result<Notify> {
        // FIXME CHECK
        Ok(Notify {
            handle: handle.clone(),
            tx: self.tx.clone(),
        })
    }

    pub fn writed_for(&mut self, handle: &att::Handle) -> Result<Writed> {
        // FIXME CHECK
        let (tx, rx) = mpsc::channel(2);
        self.listeners.insert(handle.clone(), tx);
        Ok(Writed { rx })
    }

    async fn send(&mut self, item: impl Into<Att>) -> Result<()> {
        self.io.send(item.into()).await?;
        Ok(())
    }

    async fn on_read_by_group_type(&mut self, item: att::ReadByGroupTypeRequest) -> Result<()> {
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
                for item in iter {
                    b.add(
                        (item.0).start().clone(),
                        (item.0).end().clone(),
                        item.1.clone(),
                    );
                }
                self.send(b.build()).await?
            }
            Err(gatt::Error::AttError(e)) => {
                let d = att::ErrorResponse::new(
                    0x10, //pkt::att::ReadByGroupTypeRequest::OPCODE,
                    item.starting_handle(),
                    e,
                );
                self.send(d).await?
            }
            Err(..) => {
                let d = att::ErrorResponse::new(
                    0x10, //pkt::att::ReadByGroupTypeRequest::OPCODE,
                    item.starting_handle(),
                    att::ErrorCode::AttributeNotFound,
                );
                self.send(d).await?
            }
        };
        Ok(())
    }

    async fn on_read_by_type(&mut self, item: att::ReadByTypeRequest) -> Result<()> {
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
                for item in iter {
                    b.add(item.0.clone(), item.1.clone());
                }
                self.send(b.build()).await?
            }
            Err(gatt::Error::AttError(e)) => {
                let d = att::ErrorResponse::new(
                    0x08, //pkt::att::ReadByTypeRequest::OPCODE,
                    item.starting_handle(),
                    e,
                );
                self.send(d).await?
            }
            Err(..) => {
                let d = att::ErrorResponse::new(
                    0x08, //pkt::att::ReadByTypeRequest::OPCODE,
                    item.starting_handle(),
                    att::ErrorCode::AttributeNotFound,
                );
                self.send(d).await?
            }
        };
        Ok(())
    }

    async fn on_findinformation(&mut self, item: att::FindInformationRequest) -> Result<()> {
        let response = self
            .db
            .find_information(item.starting_handle(), item.ending_handle());

        match response {
            Ok(response) => {
                let mut iter = response.iter();
                let head = iter.next().unwrap();
                let mut b = att::FindInformationResponse::builder(head.0.clone(), head.1.clone());
                for item in iter {
                    b.add(item.0.clone(), item.1.clone());
                }
                self.send(b.build()).await?
            }
            Err(gatt::Error::AttError(e)) => {
                let d = att::ErrorResponse::new(
                    0x05, //pkt::att::FindInformationResponse::OPCODE,
                    item.starting_handle(),
                    e,
                );
                self.send(d).await?
            }
            Err(..) => {
                let d = att::ErrorResponse::new(
                    0x05, //pkt::att::FindInformationResponse::OPCODE,
                    item.starting_handle(),
                    att::ErrorCode::UnlikelyError,
                );
                self.send(d).await?
            }
        };
        Ok(())
    }

    async fn on_find_by_type_value(&mut self, item: att::FindByTypeValueRequest) -> Result<()> {
        let response = self.db.find_by_type_value(
            item.starting_handle(),
            item.ending_handle(),
            item.attribute_type(),
            item.attribute_value(),
        );

        match response {
            Ok(response) => {
                let mut iter = response.iter();
                let head = iter.next().unwrap();
                let mut b = att::FindByTypeValueResponse::builder(head.0.clone(), head.1.clone());
                for item in iter {
                    b.add(item.0.clone(), item.1.clone());
                }
                self.send(b.build()).await?
            }
            Err(gatt::Error::AttError(e)) => {
                let d = att::ErrorResponse::new(
                    0x07, //pkt::att::FindInformationResponse::OPCODE,
                    item.starting_handle(),
                    e,
                );
                self.send(d).await?
            }
            Err(..) => {
                let d = att::ErrorResponse::new(
                    0x07, //pkt::att::FindInformationResponse::OPCODE,
                    item.starting_handle(),
                    att::ErrorCode::UnlikelyError,
                );
                self.send(d).await?
            }
        };
        Ok(())
    }

    async fn on_read(&mut self, item: att::ReadRequest) -> Result<()> {
        let response = self.db.read(item.attribute_handle());

        match response {
            Ok(response) => {
                let b = att::ReadResponse::new(response);
                self.send(b).await?
            }
            Err(gatt::Error::AttError(e)) => {
                let d = att::ErrorResponse::new(
                    0x0A, //pkt::att::ReadRequest::OPCODE,
                    item.attribute_handle(),
                    e,
                );
                self.send(d).await?
            }
            Err(..) => {
                let d = att::ErrorResponse::new(
                    0x0A, //pkt::att::ReadRequest::OPCODE,
                    item.attribute_handle(),
                    att::ErrorCode::UnlikelyError,
                );
                self.send(d).await?
            }
        };
        Ok(())
    }

    async fn on_read_blob(&mut self, item: att::ReadBlobRequest) -> Result<()> {
        let response = self
            .db
            .read_blob(item.attribute_handle(), item.value_offset());

        match response {
            Ok(response) => {
                let b = att::ReadBlobResponse::new(response);
                self.send(b).await?
            }
            Err(gatt::Error::AttError(e)) => {
                let d = att::ErrorResponse::new(
                    0x0C, //pkt::att::ReadRequest::OPCODE,
                    item.attribute_handle(),
                    e,
                );
                self.send(d).await?
            }
            Err(..) => {
                let d = att::ErrorResponse::new(
                    0x0C, //pkt::att::ReadRequest::OPCODE,
                    item.attribute_handle(),
                    att::ErrorCode::UnlikelyError,
                );
                self.send(d).await?
            }
        };
        Ok(())
    }

    async fn on_write(&mut self, item: att::WriteRequest) -> Result<()> {
        let response = self
            .db
            .write(item.attribute_handle(), item.attribute_value());

        if let Some(rx) = self.listeners.get_mut(&item.attribute_handle()) {
            rx.send(item.attribute_value()).await?;
        }

        match response {
            Some(_response) => {
                let b = att::WriteResponse::new();
                self.send(b).await?
            }
            None => {
                let d = att::ErrorResponse::new(
                    0x0A, //pkt::att::ReadRequest::OPCODE,
                    item.attribute_handle(),
                    att::ErrorCode::AttributeNotFound,
                );
                self.send(d).await?
            }
        };
        Ok(())
    }

    async fn on_exchange_mtu(&mut self, item: att::ExchangeMtuRequest) -> Result<()> {
        let response = self.db.exchange_mtu(item.client_rx_mtu());

        match response {
            Ok(mtu) => {
                let b = att::ExchangeMtuResponse::new(mtu);
                self.send(b).await?
            }
            Err(..) => {
                let d = att::ErrorResponse::new(
                    0x02, //pkt::att::ReadRequest::OPCODE,
                    att::Handle::from(0),
                    att::ErrorCode::UnlikelyError,
                );
                self.send(d).await?
            }
        };
        Ok(())
    }

    pub async fn run(mut self) -> Result<()> {
        loop {
            tokio::select! {
                Some(pkt) = self.io.next() => {
                    match pkt? {
                        Att::ReadByGroupTypeRequest(item) => self.on_read_by_group_type(item).await?,
                        Att::ReadByTypeRequest(item) => self.on_read_by_type(item).await?,
                        Att::FindInformationRequest(item) => self.on_findinformation(item).await?,
                        Att::FindByTypeValueRequest(item) => self.on_find_by_type_value(item).await?,
                        Att::ReadRequest(item) => self.on_read(item).await?,
                        Att::ReadBlobRequest(item) => self.on_read_blob(item).await?,
                        Att::WriteRequest(item) => self.on_write(item).await?,
                        Att::ExchangeMtuRequest(item) => self.on_exchange_mtu(item).await?,
                        _ => unimplemented!(),
                    }
                }
                Some(pkt) = self.rx.next() => {
                    self.send(pkt).await?;
                }
                else => {
                    break;
                }
            }
        }

        Ok(())
    }
}
