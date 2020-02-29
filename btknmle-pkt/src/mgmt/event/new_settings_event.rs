use bytes::{Buf, BufMut};

use super::CurrentSettings;
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct NewSettingsEvent {
    controller_index: ControlIndex,
    current_settings: CurrentSettings,
}

impl NewSettingsEvent {
    pub fn new(controller_index: ControlIndex, current_settings: CurrentSettings) -> Self {
        Self {
            controller_index,
            current_settings,
        }
    }

    pub fn controller_index(&self) -> ControlIndex {
        self.controller_index.clone()
    }

    pub fn current_settings(&self) -> CurrentSettings {
        self.current_settings
    }
}

impl EventItem for NewSettingsEvent {
    const CODE: Code = Code(0x0006);

    fn with_controller_index(mut self, idx: ControlIndex) -> Self {
        self.controller_index = idx;
        self
    }
}

impl PacketData for NewSettingsEvent {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let current_settings = PacketData::unpack(buf)?;
        Ok(Self {
            controller_index: Default::default(),
            current_settings,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.current_settings.pack(buf)
    }
}

impl From<NewSettingsEvent> for MgmtEvent {
    fn from(v: NewSettingsEvent) -> Self {
        Self::NewSettingsEvent(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = NewSettingsEvent::new(Default::default(), CurrentSettings::POWERED);
        e.pack(&mut b).unwrap();
        let r = NewSettingsEvent::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
