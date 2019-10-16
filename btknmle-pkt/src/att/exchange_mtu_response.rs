use bytes::{Buf, BufMut as _, BytesMut};

use super::{Att, AttItem, Codec, CodecError};

#[derive(Debug)]
pub struct ExchangeMtuResponse {
    server_rx_mtu: u16,
}

impl ExchangeMtuResponse {
    pub fn new(server_rx_mtu: u16) -> Self {
        Self { server_rx_mtu }
    }
}

impl AttItem for ExchangeMtuResponse {
    const OPCODE: u8 = 0x03;
}

impl Codec for ExchangeMtuResponse {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        let server_rx_mtu = buf.get_u16_le();

        Ok(Self { server_rx_mtu })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        buf.put_u16_le(self.server_rx_mtu);

        Ok(())
    }
}

impl From<ExchangeMtuResponse> for Att {
    fn from(v: ExchangeMtuResponse) -> Att {
        Att::ExchangeMtuResponse(v)
    }
}
