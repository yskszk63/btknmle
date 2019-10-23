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
            x => Self::Unknown(x),
        }
    }
}
