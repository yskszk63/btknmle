use bytes::{Buf, BufMut};

use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use super::{CompleteName, Name, ShortName};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct SetLocalNameCommandResult {
    name: Name<CompleteName>,
    short_name: Name<ShortName>,
}

impl PacketData for SetLocalNameCommandResult {
    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.name.pack(buf)?;
        self.short_name.pack(buf)
    }

    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let name = PacketData::unpack(buf)?;
        let short_name = PacketData::unpack(buf)?;
        Ok(Self { name, short_name })
    }
}

impl SetLocalNameCommandResult {
    pub fn new(name: impl AsRef<str>, short_name: impl AsRef<str>) -> Self {
        let name = Name::with_complete_name(name).unwrap(); // FIXME
        let short_name = Name::with_short_name(short_name).unwrap(); // FIXME
        Self { name, short_name }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SetLocalNameCommand {
    ctrl_idx: u16,
    name: Name<CompleteName>,
    short_name: Name<ShortName>,
}

impl SetLocalNameCommand {
    pub fn new(ctrl_idx: u16, name: impl AsRef<str>, short_name: impl AsRef<str>) -> Self {
        let name = Name::with_complete_name(name).unwrap(); // FIXME
        let short_name = Name::with_short_name(short_name).unwrap(); // FIXME
        Self {
            ctrl_idx,
            name,
            short_name,
        }
    }
}

impl ManagementCommand<SetLocalNameCommandResult> for SetLocalNameCommand {
    fn parse_result(buf: &mut impl Buf) -> Result<SetLocalNameCommandResult, crate::CodecError> {
        Ok(SetLocalNameCommandResult::unpack(buf)?)
    }
}

impl CommandItem for SetLocalNameCommand {
    const CODE: Code = Code(0x000F);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl PacketData for SetLocalNameCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let name = PacketData::unpack(buf)?;
        let short_name = PacketData::unpack(buf)?;
        Ok(Self {
            ctrl_idx: Default::default(),
            name,
            short_name,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.name.pack(buf)?;
        self.short_name.pack(buf)
    }
}

impl From<SetLocalNameCommand> for MgmtCommand {
    fn from(v: SetLocalNameCommand) -> Self {
        Self::SetLocalNameCommand(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = SetLocalNameCommand::new(Default::default(), "aaa", "bbb");
        e.pack(&mut b).unwrap();
        let r = SetLocalNameCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }

    #[test]
    fn test_result() {
        let mut b = vec![];
        let e = SetLocalNameCommandResult::new("aaa", "bbb");
        e.pack(&mut b).unwrap();
        let r = SetLocalNameCommandResult::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
