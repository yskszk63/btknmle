use bytes::{Buf, BufMut};

use super::CurrentSettings;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub enum Discoverable {
    Disabled,
    General,
    Limited(u16),
}

impl PacketData for Discoverable {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let v = u8::unpack(buf)?;
        let t = u16::unpack(buf)?;
        Ok(match v {
            0x00 => Discoverable::Disabled,
            0x01 => Discoverable::General,
            0x02 => Discoverable::Limited(t),
            x => return Err(UnpackError::unexpected(format!("value {}", x))),
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        let (v, t) = match self {
            Discoverable::Disabled => (0x00u8, 0x0000),
            Discoverable::General => (0x01u8, 0x0000),
            Discoverable::Limited(t) => (0x02u8, *t),
        };
        v.pack(buf)?;
        t.pack(buf)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SetDiscoverableCommand {
    ctrl_idx: u16,
    discoverable: Discoverable,
}

impl SetDiscoverableCommand {
    pub fn new(ctrl_idx: u16, discoverable: Discoverable) -> Self {
        Self {
            ctrl_idx,
            discoverable,
        }
    }
}

impl ManagementCommand<CurrentSettings> for SetDiscoverableCommand {
    fn parse_result(buf: &mut impl Buf) -> Result<CurrentSettings, crate::CodecError> {
        Ok(CurrentSettings::unpack(buf)?)
    }
}

impl CommandItem for SetDiscoverableCommand {
    const CODE: Code = Code(0x0006);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl PacketData for SetDiscoverableCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let discoverable = PacketData::unpack(buf)?;
        Ok(Self {
            ctrl_idx: Default::default(),
            discoverable,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.discoverable.pack(buf)
    }
}

impl From<SetDiscoverableCommand> for MgmtCommand {
    fn from(v: SetDiscoverableCommand) -> Self {
        Self::SetDiscoverableCommand(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = SetDiscoverableCommand::new(Default::default(), Discoverable::Limited(10));
        e.pack(&mut b).unwrap();
        let r = SetDiscoverableCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
