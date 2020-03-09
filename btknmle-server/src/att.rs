use std::io;

use bytes::BytesMut;
use log::debug;
use tokio_util::codec::{Decoder, Encoder};

use crate::pkt::att::Att;
use crate::pkt::PacketData;

pub struct AttCodec;

impl Encoder<Att> for AttCodec {
    type Error = std::io::Error;

    fn encode(&mut self, item: Att, dst: &mut BytesMut) -> Result<(), Self::Error> {
        debug!("> {:?}", item);
        item.pack(dst)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(())
    }
}

impl Decoder for AttCodec {
    type Item = Att;
    type Error = std::io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if buf.is_empty() {
            return Ok(None);
        }

        let result =
            Self::Item::unpack(buf).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        debug!("< {:?}", result);
        Ok(Some(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        use crate::pkt::att::WriteResponse;
        use futures::sink::SinkExt;
        use futures::stream::StreamExt;
        use tokio::net::UnixStream;
        use tokio_util::codec::{FramedRead, FramedWrite};

        let (socka, sockb) = UnixStream::pair().unwrap();

        let mut socka = FramedRead::new(socka, AttCodec);
        let mut sockb = FramedWrite::new(sockb, AttCodec);

        let n = 1024;
        tokio::spawn(async move {
            for _ in 0..n {
                sockb.send(WriteResponse::new().into()).await.unwrap();
            }
        });

        for _ in 0..n {
            socka.next().await.unwrap().unwrap();
        }
    }
}
