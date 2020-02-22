use bytes::{Buf, BytesMut};

use super::CurrentSettings;
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use super::{Codec, Result};

#[derive(Debug)]
pub struct NewSettingsEvent {
    controller_index: ControlIndex,
    current_settings: CurrentSettings,
}

impl NewSettingsEvent {
    pub fn controller_index(&self) -> ControlIndex {
        self.controller_index.clone()
    }

    pub fn current_settings(&self) -> CurrentSettings {
        self.current_settings.clone()
    }
}

impl EventItem for NewSettingsEvent {
    const CODE: Code = Code(0x0006);

    fn with_controller_index(mut self, idx: ControlIndex) -> Self {
        self.controller_index = idx;
        self
    }
}

impl Codec for NewSettingsEvent {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let controller_index = Default::default();
        let current_settings = CurrentSettings::parse(buf)?;
        Ok(Self {
            controller_index,
            current_settings,
        })
    }

    fn write_to(&self, _buf: &mut BytesMut) -> Result<()> {
        unimplemented!()
    }
}

impl From<NewSettingsEvent> for MgmtEvent {
    fn from(v: NewSettingsEvent) -> Self {
        Self::NewSettingsEvent(v)
    }
}
