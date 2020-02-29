use bytes::{Buf, BufMut};

use super::CurrentSettings;
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct NewSettingsEvent {
    current_settings: CurrentSettings,
}

impl NewSettingsEvent {
    pub fn new(current_settings: CurrentSettings) -> Self {
        Self { current_settings }
    }

    pub fn current_settings(&self) -> CurrentSettings {
        self.current_settings
    }
}

impl EventItem for NewSettingsEvent {
    const CODE: Code = Code(0x0006);

    fn into_mgmt(self, index: ControlIndex) -> MgmtEvent {
        MgmtEvent::NewSettingsEvent(index, self)
    }
}

impl PacketData for NewSettingsEvent {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let current_settings = PacketData::unpack(buf)?;
        Ok(Self { current_settings })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.current_settings.pack(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = NewSettingsEvent::new(CurrentSettings::POWERED);
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtEvent::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
