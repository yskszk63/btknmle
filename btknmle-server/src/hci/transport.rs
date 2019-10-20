use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::{BytesMut, IntoBuf};
use failure::Fail;
use futures::{Sink, SinkExt as _, Stream, StreamExt as _};
use log::debug;
use tokio::codec::{Decoder, Encoder};

use btknmle_pkt::hci::command::Command;
use btknmle_pkt::hci::event::Event;
use btknmle_pkt::hci::HciPacket;
use btknmle_pkt::Codec as _;
use btknmle_sock::{HciFramed, HciSocket};

#[derive(Debug, Fail)]
pub enum HciCommandInvocationError {
    #[fail(display = "IO Error {}", _0)]
    Io(#[fail(cause)] io::Error),
    #[fail(display = "Unexpected EOF")]
    UnexpectedEof,
    #[fail(display = "Unexpected Result")]
    UnexpectedResult,
}

impl From<io::Error> for HciCommandInvocationError {
    fn from(v: io::Error) -> Self {
        Self::Io(v)
    }
}

#[derive(Debug)]
struct PacketCodec;

impl Encoder for PacketCodec {
    type Item = HciPacket;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        debug!("> {:?}", item);
        item.write_to(buf).unwrap();
        Ok(())
    }
}

impl Decoder for PacketCodec {
    type Item = HciPacket;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let item = HciPacket::parse(&mut buf.take().into_buf()).unwrap();
        debug!("< {:?}", item);
        Ok(Some(item))
    }
}

#[derive(Debug)]
pub struct HciTransport(HciFramed<PacketCodec>);

impl HciTransport {
    pub fn new(devid: u16) -> io::Result<Self> {
        let sock = HciSocket::bind(devid)?;
        let frames = HciFramed::new(sock, PacketCodec);
        Ok(Self(frames))
    }

    pub async fn invoke(
        &mut self,
        command: impl Into<Command>,
    ) -> Result<(), HciCommandInvocationError> {
        let command = command.into();
        let opcode = command.opcode();
        self.send(HciPacket::Command(command.into())).await?;
        match self.next().await {
            Some(e) => {
                if let HciPacket::Event(Event::CmdComplete(e)) = e? {
                    if e.opcode() == opcode {
                        Ok(())
                    } else {
                        Err(HciCommandInvocationError::UnexpectedResult)
                    }
                } else {
                    Err(HciCommandInvocationError::UnexpectedResult)
                }
            }
            None => Err(HciCommandInvocationError::UnexpectedEof),
        }
    }
}

impl Stream for HciTransport {
    type Item = io::Result<HciPacket>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.0).poll_next(cx)
    }
}

impl Sink<HciPacket> for HciTransport {
    type Error = io::Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.0).poll_ready(cx)
    }

    fn start_send(mut self: Pin<&mut Self>, item: HciPacket) -> io::Result<()> {
        Pin::new(&mut self.0).start_send(item)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.0).poll_flush(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.0).poll_close(cx)
    }
}
