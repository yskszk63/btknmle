use bytes::{Buf, BufMut};

use super::{Att, AttItem, Handle};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct FindInformationRequest {
    starting_handle: Handle,
    ending_handle: Handle,
}

impl FindInformationRequest {
    pub fn new(starting_handle: Handle, ending_handle: Handle) -> Self {
        Self {
            starting_handle,
            ending_handle,
        }
    }

    pub fn starting_handle(&self) -> Handle {
        self.starting_handle.clone()
    }

    pub fn ending_handle(&self) -> Handle {
        self.ending_handle.clone()
    }
}

impl AttItem for FindInformationRequest {
    const OPCODE: u8 = 0x04;
}

impl PacketData for FindInformationRequest {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let starting_handle = PacketData::unpack(buf)?;
        let ending_handle = PacketData::unpack(buf)?;

        Ok(Self {
            starting_handle,
            ending_handle,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.starting_handle.pack(buf)?;
        self.ending_handle.pack(buf)
    }
}

impl From<FindInformationRequest> for Att {
    fn from(v: FindInformationRequest) -> Att {
        Att::FindInformationRequest(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = Att::from(FindInformationRequest::new(
            Handle::from(0x0000),
            Handle::from(0xFFFF),
        ));
        e.pack(&mut b).unwrap();
        let r = Att::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
