use std::convert::{TryFrom, TryInto};

use bytes::{Buf, BufMut};

use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    BackgroundScanForDevice,
    AllowIncommingConnection,
    AutoConnectRemoteDevice,
}

impl PacketData for Action {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let b = u8::unpack(buf)?;
        b.try_into().map_err(UnpackError::UnexpectedValue)
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        buf.put_u8(self.clone().into());
        Ok(())
    }
}

impl From<Action> for u8 {
    fn from(v: Action) -> Self {
        match v {
            Action::BackgroundScanForDevice => 0x00,
            Action::AllowIncommingConnection => 0x01,
            Action::AutoConnectRemoteDevice => 0x02,
        }
    }
}

impl TryFrom<u8> for Action {
    type Error = u8;
    fn try_from(v: u8) -> std::result::Result<Self, Self::Error> {
        Ok(match v {
            0x00 => Action::BackgroundScanForDevice,
            0x01 => Action::AllowIncommingConnection,
            0x02 => Action::AutoConnectRemoteDevice,
            v => return Err(v),
        })
    }
}
