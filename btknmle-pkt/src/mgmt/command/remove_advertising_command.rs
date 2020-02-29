use std::num::NonZeroU8;

use bytes::{Buf, BufMut};

use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct RemoveAdvertisingCommand {
    instance: u8,
}

impl RemoveAdvertisingCommand {
    pub fn new(instance: Option<NonZeroU8>) -> Self {
        let instance = instance.map(NonZeroU8::get).unwrap_or_else(|| 0);
        Self { instance }
    }
}

impl ManagementCommand for RemoveAdvertisingCommand {
    type Result = u8;

    fn into_mgmt(self, i: ControlIndex) -> MgmtCommand {
        MgmtCommand::RemoveAdvertisingCommand(i, self)
    }
}

impl CommandItem for RemoveAdvertisingCommand {
    const CODE: Code = Code(0x003f);
}

impl PacketData for RemoveAdvertisingCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let instance = PacketData::unpack(buf)?;
        Ok(Self { instance })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.instance.pack(buf)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = RemoveAdvertisingCommand::new(None);
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
