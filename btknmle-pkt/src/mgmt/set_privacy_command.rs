use bytes::{Buf, BufMut as _, BytesMut};

use super::CurrentSettings;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use super::{Codec, Result};
use crate::util::HexDisplay;

#[derive(Debug)]
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
    fn parse_result(buf: &mut impl Buf) -> Result<CurrentSettings> {
        Ok(CurrentSettings::parse(buf)?)
    }
}

impl CommandItem for SetPrivacyCommand {
    const CODE: Code = Code(0x002F);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl Codec for SetPrivacyCommand {
    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        let v = match self.privacy {
            false => 0x00,
            true => 0x01,
        };
        buf.put_u8(v);
        buf.extend_from_slice(self.identity_resolving_key.as_ref());
        Ok(())
    }

    fn parse(_buf: &mut impl Buf) -> Result<Self> {
        unimplemented!()
    }
}

impl From<SetPrivacyCommand> for MgmtCommand {
    fn from(v: SetPrivacyCommand) -> Self {
        Self::SetPrivacyCommand(v)
    }
}
