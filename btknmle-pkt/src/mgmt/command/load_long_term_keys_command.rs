use bytes::{Buf, BufMut};

use super::LongTermKey;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct LoadLongTermKeysCommand {
    keys: Vec<LongTermKey>,
}

impl LoadLongTermKeysCommand {
    pub fn new(keys: Vec<LongTermKey>) -> Self {
        Self { keys }
    }
}

impl ManagementCommand for LoadLongTermKeysCommand {
    type Result = ();

    fn into_mgmt(self, i: ControlIndex) -> MgmtCommand {
        MgmtCommand::LoadLongTermKeysCommand(i, self)
    }
}

impl CommandItem for LoadLongTermKeysCommand {
    const CODE: Code = Code(0x0013);
}

impl PacketData for LoadLongTermKeysCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let len = u16::unpack(buf)? as usize;
        let mut keys = vec![];
        for _ in 0..len {
            let key = PacketData::unpack(buf)?;
            keys.push(key);
        }
        Ok(Self { keys })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        (self.keys.len() as u16).pack(buf)?;
        for key in &self.keys {
            key.pack(buf)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::AddressType;
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = LoadLongTermKeysCommand::new(vec![LongTermKey::new(
            "00:11:22:33:44:55".parse().unwrap(),
            AddressType::LeRandom,
            0,
            1,
            2,
            [3; 2],
            [4; 8],
            [5; 16],
        )]);
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
