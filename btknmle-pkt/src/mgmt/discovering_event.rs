use bytes::{Buf, BytesMut};

use super::AddressType;
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use super::{Codec, Result};

#[derive(Debug)]
pub struct DiscoveringEvent {
    controller_index: ControlIndex,
    address_type: AddressType,
    discovering: bool,
}

impl DiscoveringEvent {
    pub fn controller_index(&self) -> ControlIndex {
        self.controller_index.clone()
    }

    pub fn address_type(&self) -> AddressType {
        self.address_type.clone()
    }

    pub fn discovering(&self) -> bool {
        self.discovering
    }
}

impl EventItem for DiscoveringEvent {
    const CODE: Code = Code(0x0013);

    fn with_controller_index(mut self, idx: ControlIndex) -> Self {
        self.controller_index = idx;
        self
    }
}

impl Codec for DiscoveringEvent {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let controller_index = Default::default();
        let address_type = AddressType::parse(buf)?;
        let discovering = buf.get_u8() != 0;
        Ok(Self {
            controller_index,
            address_type,
            discovering,
        })
    }

    fn write_to(&self, _buf: &mut BytesMut) -> Result<()> {
        unimplemented!()
    }
}

impl From<DiscoveringEvent> for MgmtEvent {
    fn from(v: DiscoveringEvent) -> Self {
        Self::DiscoveringEvent(v)
    }
}
