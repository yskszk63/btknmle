use std::io;

use bytes::BytesMut;
use log::debug;
use tokio_util::codec::{Decoder, Encoder};

use crate::pkt::att::Att;
use crate::pkt::PacketData;

pub struct AttCodec;

impl Encoder for AttCodec {
    type Item = Att;
    type Error = std::io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        debug!("> {:?}", item);
        item.pack(dst)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, failure::Error::from(e)))?;
        Ok(())
    }
}

impl Decoder for AttCodec {
    type Item = Att;
    type Error = std::io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let result = Self::Item::unpack(buf)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, failure::Error::from(e)))?;
        debug!("< {:?}", result);
        Ok(Some(result))
    }
}
