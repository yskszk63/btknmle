use std::num::NonZeroU8;

use bytes::{Buf, BufMut};

use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct RemoveAdvertisingCommand {
    ctrl_idx: u16,
    instance: u8,
}

impl RemoveAdvertisingCommand {
    pub fn new(ctrl_idx: u16, instance: Option<NonZeroU8>) -> Self {
        let instance = instance.map(NonZeroU8::get).unwrap_or_else(|| 0);
        Self { ctrl_idx, instance }
    }
}

impl ManagementCommand<u8> for RemoveAdvertisingCommand {
    fn parse_result(buf: &mut impl Buf) -> Result<u8, crate::CodecError> {
        Ok(u8::unpack(buf)?)
    }
}

impl CommandItem for RemoveAdvertisingCommand {
    const CODE: Code = Code(0x003f);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl PacketData for RemoveAdvertisingCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let instance = PacketData::unpack(buf)?;
        Ok(Self {
            ctrl_idx: Default::default(),
            instance,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.instance.pack(buf)?;
        Ok(())
    }
}

impl From<RemoveAdvertisingCommand> for MgmtCommand {
    fn from(v: RemoveAdvertisingCommand) -> Self {
        Self::RemoveAdvertisingCommand(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = RemoveAdvertisingCommand::new(Default::default(), None);
        e.pack(&mut b).unwrap();
        let r = RemoveAdvertisingCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
