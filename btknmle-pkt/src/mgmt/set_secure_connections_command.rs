use bytes::{Buf, BufMut};

use super::CurrentSettings;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::{PackError, PacketData, UnpackError};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SecureConnections {
    Disabled,
    Enabled,
    Only,
}

#[derive(Debug, PartialEq, Eq)]
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

impl ManagementCommand for SetSecureConnectionsCommand {
    type Result = CurrentSettings;
}

impl CommandItem for SetSecureConnectionsCommand {
    const CODE: Code = Code(0x002D);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl PacketData for SetSecureConnectionsCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let v = u8::unpack(buf)?;
        let secure_connections = match v {
            0x00 => SecureConnections::Disabled,
            0x01 => SecureConnections::Enabled,
            0x02 => SecureConnections::Only,
            x => return Err(UnpackError::UnexpectedValue(x)),
        };
        Ok(Self {
            ctrl_idx: Default::default(),
            secure_connections,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        let v = match self.secure_connections {
            SecureConnections::Disabled => 0x00,
            SecureConnections::Enabled => 0x01,
            SecureConnections::Only => 0x02,
        };
        u8::pack(&v, buf)
    }
}

impl From<SetSecureConnectionsCommand> for MgmtCommand {
    fn from(v: SetSecureConnectionsCommand) -> Self {
        Self::SetSecureConnectionsCommand(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = SetSecureConnectionsCommand::new(Default::default(), SecureConnections::Only);
        e.pack(&mut b).unwrap();
        let r = SetSecureConnectionsCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
