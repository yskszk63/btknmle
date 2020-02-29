use bytes::{Buf, BufMut};

use super::CurrentSettings;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::util::HexDisplay;
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct SetPrivacyCommand {
    privacy: bool,
    identity_resolving_key: HexDisplay<[u8; 16]>,
}

impl SetPrivacyCommand {
    pub fn new(privacy: bool, identity_resolving_key: [u8; 16]) -> Self {
        let identity_resolving_key = HexDisplay::new(identity_resolving_key);
        Self {
            privacy,
            identity_resolving_key,
        }
    }
}

impl ManagementCommand for SetPrivacyCommand {
    type Result = CurrentSettings;

    fn into_mgmt(self, i: ControlIndex) -> MgmtCommand {
        MgmtCommand::SetPrivacyCommand(i, self)
    }
}

impl CommandItem for SetPrivacyCommand {
    const CODE: Code = Code(0x002F);
}

impl PacketData for SetPrivacyCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let privacy = u8::unpack(buf)? != 0;
        if buf.remaining() < 16 {
            return Err(UnpackError::UnexpectedEof);
        }
        let mut identity_resolving_key = HexDisplay::new([0; 16]);
        buf.copy_to_slice(identity_resolving_key.as_mut());
        Ok(Self {
            privacy,
            identity_resolving_key,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        (self.privacy as u8).pack(buf)?;
        if buf.remaining_mut() < 16 {
            return Err(PackError::InsufficientBufLength);
        }
        buf.put(self.identity_resolving_key.as_ref());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = SetPrivacyCommand::new(true, [0; 16]);
        let e = e.into_mgmt(Default::default());
        e.pack(&mut b).unwrap();
        let r = MgmtCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
