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
            x => return Err(UnpackError::UnexpectedValue(x)),
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
    discoverable: Discoverable,
}

impl SetDiscoverableCommand {
    pub fn new(discoverable: Discoverable) -> Self {
        Self { discoverable }
    }
}

impl ManagementCommand for SetDiscoverableCommand {
    type Result = CurrentSettings;

    fn into_mgmt(self, i: ControlIndex) -> MgmtCommand {
        MgmtCommand::SetDiscoverableCommand(i, self)
    }
}

impl CommandItem for SetDiscoverableCommand {
    const CODE: Code = Code(0x0006);
}

impl PacketData for SetDiscoverableCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let discoverable = PacketData::unpack(buf)?;
        Ok(Self { discoverable })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.discoverable.pack(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = SetDiscoverableCommand::new(Discoverable::Limited(10));
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
