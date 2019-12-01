use bytes::buf::BufExt as _;
use bytes::{Buf, Bytes, BytesMut};

use super::{Code, ControlIndex, EventItem, MgmtEvent};
use super::{Codec, Result};

#[derive(Debug)]
pub struct ExtendedControllerInformationChangedEvent {
    controller_index: ControlIndex,
    eir_data: Bytes,
}

impl ExtendedControllerInformationChangedEvent {
    pub fn controller_index(&self) -> ControlIndex {
        self.controller_index.clone()
    }

    pub fn eir_data(&self) -> Bytes {
        self.eir_data.clone()
    }
}

impl EventItem for ExtendedControllerInformationChangedEvent {
    const CODE: Code = Code(0x0025);

    fn with_controller_index(mut self, idx: ControlIndex) -> Self {
        self.controller_index = idx;
        self
    }
}

impl Codec for ExtendedControllerInformationChangedEvent {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let controller_index = Default::default();
        let len = buf.get_u16_le() as usize;
        let eir_data = buf.take(len).to_bytes();
        Ok(Self {
            controller_index,
            eir_data,
        })
    }

    fn write_to(&self, _buf: &mut BytesMut) -> Result<()> {
        unimplemented!()
    }
}

impl From<ExtendedControllerInformationChangedEvent> for MgmtEvent {
    fn from(v: ExtendedControllerInformationChangedEvent) -> Self {
        Self::ExtendedControllerInformationChangedEvent(v)
    }
}
