use bytes::{Buf, BufMut};

use super::CurrentSettings;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::util::HexDisplay;
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct SetPrivacyCommand {
    ctrl_idx: u16,
    privacy: bool,
    identity_resolving_key: HexDisplay<[u8; 16]>,
}

impl SetPrivacyCommand {
    pub fn new(ctrl_idx: u16, privacy: bool, identity_resolving_key: [u8; 16]) -> Self {
        let identity_resolving_key = HexDisplay::new(identity_resolving_key);
        Self {
            ctrl_idx,
            privacy,
            identity_resolving_key,
        }
    }
}

impl ManagementCommand<CurrentSettings> for SetPrivacyCommand {
    fn parse_result(buf: &mut impl Buf) -> Result<CurrentSettings, crate::CodecError> {
        Ok(CurrentSettings::unpack(buf)?)
    }
}

impl CommandItem for SetPrivacyCommand {
    const CODE: Code = Code(0x002F);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
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
            ctrl_idx: Default::default(),
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

impl From<SetPrivacyCommand> for MgmtCommand {
    fn from(v: SetPrivacyCommand) -> Self {
        Self::SetPrivacyCommand(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = SetPrivacyCommand::new(Default::default(), true, [0; 16]);
        e.pack(&mut b).unwrap();
        let r = SetPrivacyCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
