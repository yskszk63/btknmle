use bitflags::bitflags;
use bytes::{Buf, BufMut as _, BytesMut};

use super::{Codec, Result, Smp, SmpItem};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoCapability {
    DisplayOnly,
    DisplayYesNo,
    KeyboardOnly,
    NoInputNoOutput,
    KeyboardDisplay,
    ReservedForFutureUse(u8),
}

impl From<u8> for IoCapability {
    fn from(v: u8) -> Self {
        match v {
            0x00 => Self::DisplayOnly,
            0x01 => Self::DisplayYesNo,
            0x02 => Self::KeyboardOnly,
            0x03 => Self::NoInputNoOutput,
            0x04 => Self::KeyboardDisplay,
            x => Self::ReservedForFutureUse(x),
        }
    }
}

impl From<IoCapability> for u8 {
    fn from(v: IoCapability) -> Self {
        match v {
            IoCapability::DisplayOnly => 0x00,
            IoCapability::DisplayYesNo => 0x01,
            IoCapability::KeyboardOnly => 0x02,
            IoCapability::NoInputNoOutput => 0x03,
            IoCapability::KeyboardDisplay => 0x04,
            IoCapability::ReservedForFutureUse(x) => x,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OobDataFlag {
    NotPresent,
    RemoteDevicePresent,
    ReservedForFutureUse(u8),
}

impl From<u8> for OobDataFlag {
    fn from(v: u8) -> Self {
        match v {
            0x00 => Self::NotPresent,
            0x01 => Self::RemoteDevicePresent,
            x => Self::ReservedForFutureUse(x),
        }
    }
}

impl From<OobDataFlag> for u8 {
    fn from(v: OobDataFlag) -> Self {
        match v {
            OobDataFlag::NotPresent => 0x00,
            OobDataFlag::RemoteDevicePresent => 0x01,
            OobDataFlag::ReservedForFutureUse(x) => x,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoundingFlags {
    NotBounding,
    Bounding,
    ReservedForFutureUse(u8),
}

#[derive(Debug, Clone)]
pub struct AuthReq {
    bounding_flags: BoundingFlags,
    mimt: bool,
    sc: bool,
    keypress: bool,
    ct2: bool,
}

impl From<u8> for AuthReq {
    fn from(v: u8) -> Self {
        let bounding_flags = match v & 0b0000_0011 {
            0b00 => BoundingFlags::NotBounding,
            0b01 => BoundingFlags::Bounding,
            x => BoundingFlags::ReservedForFutureUse(x),
        };
        let mimt = v & 0b0000_0100 != 0;
        let sc = v & 0b0000_1000 != 0;
        let keypress = v & 0b0001_0000 != 0;
        let ct2 = v & 0b0010_0000 != 0;
        Self {
            bounding_flags,
            mimt,
            sc,
            keypress,
            ct2,
        }
    }
}

impl From<AuthReq> for u8 {
    fn from(v: AuthReq) -> Self {
        let bounding_flags = match v.bounding_flags {
            BoundingFlags::NotBounding => 0b00,
            BoundingFlags::Bounding => 0b01,
            BoundingFlags::ReservedForFutureUse(x) => x & 0b0000_0011,
        };
        let mimt = if v.mimt { 0b0000_0100 } else { 0 };
        let sc = if v.sc { 0b0000_1000 } else { 0 };
        let keypress = if v.keypress { 0b0001_0000 } else { 0 };
        let ct2 = if v.ct2 { 0b0010_0000 } else { 0 };

        bounding_flags | mimt | sc | keypress | ct2
    }
}

bitflags! {
    pub struct LeKeyDistribution: u8 {
        const ENC_KEY = 0b0000_0001;
        const ID_KEY = 0b0000_0010;
        const SIGN_KEY = 0b0000_0100;
        const LINK_KEY = 0b0000_1000;
    }
}

#[derive(Debug)]
pub struct PairingRequest {
    io_capability: IoCapability,
    oob_data_flag: OobDataFlag,
    authreq: AuthReq,
    maximum_encryption_keysize: u8,
    initiator_key_distribution: LeKeyDistribution,
    responder_key_distribution: LeKeyDistribution,
}

impl SmpItem for PairingRequest {
    const CODE: u8 = 0x01;
}

impl Codec for PairingRequest {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let io_capability = buf.get_u8().into();
        let oob_data_flag = buf.get_u8().into();
        let authreq = buf.get_u8().into();
        let maximum_encryption_keysize = buf.get_u8();
        let initiator_key_distribution = LeKeyDistribution::from_bits_truncate(buf.get_u8());
        let responder_key_distribution = LeKeyDistribution::from_bits_truncate(buf.get_u8());
        Ok(Self {
            io_capability,
            oob_data_flag,
            authreq,
            maximum_encryption_keysize,
            initiator_key_distribution,
            responder_key_distribution,
        })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        buf.put_u8(self.io_capability.into());
        buf.put_u8(self.oob_data_flag.into());
        buf.put_u8(self.authreq.clone().into());
        buf.put_u8(self.maximum_encryption_keysize);
        buf.put_u8(self.initiator_key_distribution.bits());
        buf.put_u8(self.responder_key_distribution.bits());
        Ok(())
    }
}

impl From<PairingRequest> for Smp {
    fn from(v: PairingRequest) -> Self {
        Self::PairingRequest(v)
    }
}
