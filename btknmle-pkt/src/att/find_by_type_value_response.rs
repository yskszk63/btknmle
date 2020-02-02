use bytes::{Buf, BytesMut};

use super::{Att, AttItem, Codec, CodecError, Handle};

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

#[derive(Debug, Clone)]
struct HandleInformation {
    found_attribute_handle: Handle,
    group_end_handle: Handle,
}

#[derive(Debug)]
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

impl Codec for FindByTypeValueResponse {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        let mut handles_information_list = vec![];
        while buf.has_remaining() {
            let found_attribute_handle = Handle::parse(buf)?;
            let group_end_handle = Handle::parse(buf)?;
            handles_information_list.push(HandleInformation {
                found_attribute_handle,
                group_end_handle,
            });
        }
        Ok(Self {
            handles_information_list,
        })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        let iter = self.handles_information_list.iter();
        for item in iter {
            item.found_attribute_handle.write_to(buf)?;
            item.group_end_handle.write_to(buf)?;
        }
        Ok(())
    }
}

impl From<FindByTypeValueResponse> for Att {
    fn from(v: FindByTypeValueResponse) -> Att {
        Att::FindByTypeValueResponse(v)
    }
}
