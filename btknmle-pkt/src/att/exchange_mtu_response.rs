use bytes::{Buf, BufMut};

use crate::{PackError, PacketData, UnpackError};
use super::{Att, AttItem};

#[derive(Debug, PartialEq, Eq)]
pub struct ExchangeMtuResponse {
    server_rx_mtu: u16,
}

impl ExchangeMtuResponse {
    pub fn new(server_rx_mtu: u16) -> Self {
        Self { server_rx_mtu }
    }

    pub fn server_rx_mtu(&self) -> u16 {
        self.server_rx_mtu
    }
}

impl AttItem for ExchangeMtuResponse {
    const OPCODE: u8 = 0x03;
}

impl PacketData for ExchangeMtuResponse {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let server_rx_mtu = u16::unpack(buf)?;
        Ok(Self { server_rx_mtu })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.server_rx_mtu().pack(buf)
    }
}

impl From<ExchangeMtuResponse> for Att {
    fn from(v: ExchangeMtuResponse) -> Att {
        Att::ExchangeMtuResponse(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = Att::from(ExchangeMtuResponse::new(23));
        e.pack(&mut b).unwrap();
        let r = Att::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
