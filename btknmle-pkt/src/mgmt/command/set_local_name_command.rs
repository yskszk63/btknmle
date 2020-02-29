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
    name: Name<CompleteName>,
    short_name: Name<ShortName>,
}

impl SetLocalNameCommand {
    pub fn new(name: impl AsRef<str>, short_name: impl AsRef<str>) -> Self {
        let name = Name::with_complete_name(name).unwrap(); // FIXME
        let short_name = Name::with_short_name(short_name).unwrap(); // FIXME
        Self { name, short_name }
    }
}

impl ManagementCommand for SetLocalNameCommand {
    type Result = SetLocalNameCommandResult;

    fn into_mgmt(self, i: ControlIndex) -> MgmtCommand {
        MgmtCommand::SetLocalNameCommand(i, Box::new(self))
    }
}

impl CommandItem for SetLocalNameCommand {
    const CODE: Code = Code(0x000F);
}

impl PacketData for SetLocalNameCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let name = PacketData::unpack(buf)?;
        let short_name = PacketData::unpack(buf)?;
        Ok(Self { name, short_name })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.name.pack(buf)?;
        self.short_name.pack(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = SetLocalNameCommand::new("aaa", "bbb");
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtCommand::unpack(&mut b.as_ref()).unwrap();
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
