use bytes::{Buf, BufMut as _, BytesMut};

use super::{Att, AttItem, Codec, CodecError};

#[derive(Debug)]
pub struct ExchangeMtuRequest {
    client_rx_mtu: u16,
}

impl ExchangeMtuRequest {
    pub fn client_rx_mtu(&self) -> u16 {
        self.client_rx_mtu
    }
}

impl AttItem for ExchangeMtuRequest {
    const OPCODE: u8 = 0x02;
}

impl Codec for ExchangeMtuRequest {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        let client_rx_mtu = buf.get_u16_le();

        Ok(Self { client_rx_mtu })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        buf.put_u16_le(self.client_rx_mtu);

        Ok(())
    }
}

impl From<ExchangeMtuRequest> for Att {
    fn from(v: ExchangeMtuRequest) -> Att {
        Att::ExchangeMtuRequest(v)
    }
}
