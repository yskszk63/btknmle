use bytes::{Buf, BufMut as _, BytesMut};

use super::{Codec, Result, Smp, SmpItem};

#[derive(Debug, Clone, Copy)]
pub enum Reason {
    PasskeyEntryFailed,
    OobNotAvailable,
    AuthenticationRequirements,
    ConfirmValueFailed,
    PairingNotSupported,
    EncryptionKeySize,
    CommandNotSupported,
    UnspecifiedReason,
    RepeatedAttempts,
    InvalidParameters,
    DhKeyCheckFailed,
    NumericComparisonFailed,
    BrEdrPairingInProgress,
    CrossTransportKeyDerivationGenerationNotAllowed,
    ReservedForFutureUse(u8),
}

impl From<u8> for Reason {
    fn from(v: u8) -> Self {
        match v {
            0x01 => Self::PasskeyEntryFailed,
            0x02 => Self::OobNotAvailable,
            0x03 => Self::AuthenticationRequirements,
            0x04 => Self::ConfirmValueFailed,
            0x05 => Self::PairingNotSupported,
            0x06 => Self::EncryptionKeySize,
            0x07 => Self::CommandNotSupported,
            0x08 => Self::UnspecifiedReason,
            0x09 => Self::RepeatedAttempts,
            0x0A => Self::InvalidParameters,
            0x0B => Self::DhKeyCheckFailed,
            0x0C => Self::NumericComparisonFailed,
            0x0D => Self::BrEdrPairingInProgress,
            0x0E => Self::CrossTransportKeyDerivationGenerationNotAllowed,
            x => Self::ReservedForFutureUse(x),
        }
    }
}

impl From<Reason> for u8 {
    fn from(v: Reason) -> Self {
        match v {
            Reason::PasskeyEntryFailed => 0x01,
            Reason::OobNotAvailable => 0x02,
            Reason::AuthenticationRequirements => 0x03,
            Reason::ConfirmValueFailed => 0x04,
            Reason::PairingNotSupported => 0x05,
            Reason::EncryptionKeySize => 0x06,
            Reason::CommandNotSupported => 0x07,
            Reason::UnspecifiedReason => 0x08,
            Reason::RepeatedAttempts => 0x09,
            Reason::InvalidParameters => 0x0A,
            Reason::DhKeyCheckFailed => 0x0B,
            Reason::NumericComparisonFailed => 0x0C,
            Reason::BrEdrPairingInProgress => 0x0D,
            Reason::CrossTransportKeyDerivationGenerationNotAllowed => 0x0E,
            Reason::ReservedForFutureUse(x) => x,
        }
    }
}

#[derive(Debug)]
pub struct PairingFailed {
    reason: Reason,
}

impl PairingFailed {
    pub fn new(reason: Reason) -> Self {
        Self { reason }
    }
}

impl SmpItem for PairingFailed {
    const CODE: u8 = 0x05;
}

impl Codec for PairingFailed {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let reason = buf.get_u8().into();
        Ok(Self { reason })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        buf.put_u8(self.reason.into());
        Ok(())
    }
}

impl From<PairingFailed> for Smp {
    fn from(v: PairingFailed) -> Self {
        Self::PairingFailed(v)
    }
}
