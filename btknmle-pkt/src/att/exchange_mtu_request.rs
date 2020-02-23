use bytes::{Buf, BufMut};

use crate::{PackError, PacketData, UnpackError};
use super::{Att, AttItem};

#[derive(Debug, PartialEq, Eq)]
pub struct ExchangeMtuRequest {
    client_rx_mtu: u16,
}

impl ExchangeMtuRequest {
    pub fn new(client_rx_mtu: u16) -> Self {
        Self { client_rx_mtu }
    }

    pub fn client_rx_mtu(&self) -> u16 {
        self.client_rx_mtu
    }
}

impl AttItem for ExchangeMtuRequest {
    const OPCODE: u8 = 0x02;
}

impl PacketData for ExchangeMtuRequest {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let client_rx_mtu = u16::unpack(buf)?;
        Ok(Self { client_rx_mtu })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.client_rx_mtu().pack(buf)
    }
}

impl From<ExchangeMtuRequest> for Att {
    fn from(v: ExchangeMtuRequest) -> Att {
        Att::ExchangeMtuRequest(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = Att::from(ExchangeMtuRequest::new(23));
        e.pack(&mut b).unwrap();
        let r = Att::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
