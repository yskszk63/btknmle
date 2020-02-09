use bytes::{Buf, BufMut as _, BytesMut};

use super::CurrentSettings;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use super::{Codec, Result};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SecureConnections {
    Disabled,
    Enabled,
    Only,
}

#[derive(Debug)]
pub struct SetSecureConnectionsCommand {
    ctrl_idx: u16,
    secure_connections: SecureConnections,
}

impl SetSecureConnectionsCommand {
    pub fn new(ctrl_idx: u16, secure_connections: SecureConnections) -> Self {
        Self {
            ctrl_idx,
            secure_connections,
        }
    }
}

impl ManagementCommand<CurrentSettings> for SetSecureConnectionsCommand {
    fn parse_result(buf: &mut impl Buf) -> Result<CurrentSettings> {
        Ok(CurrentSettings::parse(buf)?)
    }
}

impl CommandItem for SetSecureConnectionsCommand {
    const CODE: Code = Code(0x002D);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl Codec for SetSecureConnectionsCommand {
    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        let v = match self.secure_connections {
            SecureConnections::Disabled => 0x00,
            SecureConnections::Enabled => 0x01,
            SecureConnections::Only => 0x02,
        };
        buf.put_u8(v);
        Ok(())
    }

    fn parse(_buf: &mut impl Buf) -> Result<Self> {
        unimplemented!()
    }
}

impl From<SetSecureConnectionsCommand> for MgmtCommand {
    fn from(v: SetSecureConnectionsCommand) -> Self {
        Self::SetSecureConnectionsCommand(v)
    }
}
