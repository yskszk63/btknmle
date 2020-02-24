use crate::{PackError, PacketData, UnpackError};
use bytes::{Buf, BufMut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Success,
    UnknownCommand,
    NotConnected,
    Failed,
    ConnectFailed,
    AuthenticationFailed,
    NotPaired,
    NoResources,
    Timeout,
    AlreadyConnected,
    Busy,
    Rejected,
    NotSupported,
    InvalidParameters,
    Disconnected,
    NotPowered,
    Cancelled,
    InvalidIndex,
    RfKilled,
    AlreadyPaired,
    PermissionDenied,
    Unknown(u8),
}

impl PacketData for Status {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        u8::unpack(buf).map(Into::into)
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        u8::from(self.clone()).pack(buf)
    }
}

impl From<u8> for Status {
    fn from(v: u8) -> Self {
        match v {
            0x00 => Self::Success,
            0x01 => Self::UnknownCommand,
            0x02 => Self::NotConnected,
            0x03 => Self::Failed,
            0x04 => Self::ConnectFailed,
            0x05 => Self::AuthenticationFailed,
            0x06 => Self::NotPaired,
            0x07 => Self::NoResources,
            0x08 => Self::Timeout,
            0x09 => Self::AlreadyConnected,
            0x0A => Self::Busy,
            0x0B => Self::Rejected,
            0x0C => Self::NotSupported,
            0x0D => Self::InvalidParameters,
            0x0E => Self::Disconnected,
            0x0F => Self::NotPowered,
            0x10 => Self::Cancelled,
            0x11 => Self::InvalidIndex,
            0x12 => Self::RfKilled,
            0x13 => Self::AlreadyPaired,
            0x14 => Self::PermissionDenied,
            x => Self::Unknown(x),
        }
    }
}

impl From<Status> for u8 {
    fn from(v: Status) -> Self {
        match v {
            Status::Success => 0x00,
            Status::UnknownCommand => 0x01,
            Status::NotConnected => 0x02,
            Status::Failed => 0x03,
            Status::ConnectFailed => 0x04,
            Status::AuthenticationFailed => 0x05,
            Status::NotPaired => 0x06,
            Status::NoResources => 0x07,
            Status::Timeout => 0x08,
            Status::AlreadyConnected => 0x09,
            Status::Busy => 0x0A,
            Status::Rejected => 0x0B,
            Status::NotSupported => 0x0C,
            Status::InvalidParameters => 0x0D,
            Status::Disconnected => 0x0E,
            Status::NotPowered => 0x0F,
            Status::Cancelled => 0x10,
            Status::InvalidIndex => 0x11,
            Status::RfKilled => 0x12,
            Status::AlreadyPaired => 0x13,
            Status::PermissionDenied => 0x14,
            Status::Unknown(x) => x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        for n in 0..=0xFF {
            let s = Status::from(n);
            let mut b = vec![];
            s.pack(&mut b).unwrap();
            let s2 = Status::unpack(&mut b.as_ref()).unwrap();
            assert_eq!(s, s2)
        }
    }
}
