use bytes::{Buf, BufMut};

use crate::{PackError, PacketData, UnpackError};
use super::{Att, AttItem, Handle};

#[derive(Debug)]
pub struct FindByTypeValueResponseBuilder {
    handles_information_list: Vec<HandleInformation>,
}

impl FindByTypeValueResponseBuilder {
    pub fn add(
        &mut self,
        found_attribute_handle: impl Into<Handle>,
        group_end_handle: impl Into<Handle>,
    ) -> &mut Self {
        let data = HandleInformation {
            found_attribute_handle: found_attribute_handle.into(),
            group_end_handle: group_end_handle.into(),
        };
        self.handles_information_list.push(data);
        self
    }

    pub fn build(&mut self) -> FindByTypeValueResponse {
        FindByTypeValueResponse {
            handles_information_list: self.handles_information_list.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HandleInformation {
    found_attribute_handle: Handle,
    group_end_handle: Handle,
}

impl PacketData for HandleInformation {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let found_attribute_handle = PacketData::unpack(buf)?;
        let group_end_handle = PacketData::unpack(buf)?;

        Ok(HandleInformation {
            found_attribute_handle,
            group_end_handle,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.found_attribute_handle.pack(buf)?;
        self.group_end_handle.pack(buf)
    }
}

// TODO implement iter iter_mut into_iter extend from_iter
#[derive(Debug, PartialEq, Eq)]
pub struct FindByTypeValueResponse {
    handles_information_list: Vec<HandleInformation>,
}

impl FindByTypeValueResponse {
    pub fn builder(
        found_attribute_handle: impl Into<Handle>,
        group_end_handle: impl Into<Handle>,
    ) -> FindByTypeValueResponseBuilder {
        let data = HandleInformation {
            found_attribute_handle: found_attribute_handle.into(),
            group_end_handle: group_end_handle.into(),
        };

        FindByTypeValueResponseBuilder {
            handles_information_list: vec![data],
        }
    }
}

impl AttItem for FindByTypeValueResponse {
    const OPCODE: u8 = 0x07;
}

impl PacketData for FindByTypeValueResponse {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let mut handles_information_list = vec![];
        while buf.has_remaining() {
            handles_information_list.push(PacketData::unpack(buf)?)
        }
        Ok(Self { handles_information_list })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        for item in &self.handles_information_list {
            item.pack(buf)?;
        }
        Ok(())
    }
}

impl From<FindByTypeValueResponse> for Att {
    fn from(v: FindByTypeValueResponse) -> Att {
        Att::FindByTypeValueResponse(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = Att::from(FindByTypeValueResponse::builder(0x0000, 0x1111).add(0x2222, 0x3333).build());
        e.pack(&mut b).unwrap();
        let r = Att::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
