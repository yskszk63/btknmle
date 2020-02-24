use std::convert::{TryFrom, TryInto};

use bytes::{Buf, BufMut};

use super::CurrentSettings;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::{PackError, PacketData, UnpackError};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Advertising {
    Disabled,
    Enabled,
    Connectable,
}

impl PacketData for Advertising {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        u8::unpack(buf)?
            .try_into()
            .map_err(|x| UnpackError::unexpected(format!("value {}", x)))
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        u8::from(self.clone()).pack(buf)
    }
}

impl TryFrom<u8> for Advertising {
    type Error = u8;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0x00 => Ok(Self::Disabled),
            0x01 => Ok(Self::Enabled),
            0x02 => Ok(Self::Connectable),
            x => Err(x),
        }
    }
}

impl From<Advertising> for u8 {
    fn from(v: Advertising) -> Self {
        match v {
            Advertising::Disabled => 0x00,
            Advertising::Enabled => 0x01,
            Advertising::Connectable => 0x02,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SetAdvertisingCommand {
    ctrl_idx: u16,
    advertising: Advertising,
}

impl SetAdvertisingCommand {
    pub fn new(ctrl_idx: u16, advertising: Advertising) -> Self {
        Self {
            ctrl_idx,
            advertising,
        }
    }
}

impl ManagementCommand for SetAdvertisingCommand {
    type Result = CurrentSettings;
}

impl CommandItem for SetAdvertisingCommand {
    const CODE: Code = Code(0x0029);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl PacketData for SetAdvertisingCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let advertising = PacketData::unpack(buf)?;
        Ok(Self {
            ctrl_idx: Default::default(),
            advertising,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.advertising.pack(buf)
    }
}

impl From<SetAdvertisingCommand> for MgmtCommand {
    fn from(v: SetAdvertisingCommand) -> Self {
        Self::SetAdvertisingCommand(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = SetAdvertisingCommand::new(Default::default(), Advertising::Connectable);
        e.pack(&mut b).unwrap();
        let r = SetAdvertisingCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
