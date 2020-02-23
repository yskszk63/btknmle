use bytes::{Buf, BufMut};

use super::{Att, AttItem};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct WriteResponse {}

impl WriteResponse {
    pub fn new() -> Self {
        Self {}
    }
}

impl AttItem for WriteResponse {
    const OPCODE: u8 = 0x13;
}

impl PacketData for WriteResponse {
    fn unpack(_buf: &mut impl Buf) -> Result<Self, UnpackError> {
        Ok(Self {})
    }

    fn pack(&self, _buf: &mut impl BufMut) -> Result<(), PackError> {
        Ok(())
    }
}

impl From<WriteResponse> for Att {
    fn from(v: WriteResponse) -> Att {
        Att::WriteResponse(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = Att::from(WriteResponse::new());
        e.pack(&mut b).unwrap();
        let r = Att::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
