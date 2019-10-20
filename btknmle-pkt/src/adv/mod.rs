use std::convert::TryFrom;

use bytes::{Buf, BufMut as _, BytesMut};

use super::{Codec, CodecError, Result};

pub use appearance::*;
pub use comp_list_uuid128::*;
pub use comp_list_uuid16::*;
pub use complete_local_name::*;
pub use flags::*;
pub use incomp_list_uuid128::*;
pub use incomp_list_uuid16::*;
pub use shortened_local_name::*;
pub use tx_power::*;

mod appearance;
mod comp_list_uuid128;
mod comp_list_uuid16;
mod complete_local_name;
mod flags;
mod incomp_list_uuid128;
mod incomp_list_uuid16;
mod shortened_local_name;
mod tx_power;

trait AdvItem: Codec + Into<Advertise> {
    const TYPE: u8;
}

#[derive(Debug)]
pub enum Advertise {
    Flags(Flags),
    IncompleteListUuid16(IncompleteListUuid16),
    CompleteListUuid16(CompleteListUuid16),
    IncompleteListUuid128(IncompleteListUuid128),
    CompleteListUuid128(CompleteListUuid128),
    ShortenedLocalName(ShortenedLocalName),
    CompleteLocalName(CompleteLocalName),
    TxPower(TxPower),
    Appearance(Appearance),
}

impl Codec for Advertise {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        Ok(match buf.get_u8() {
            Flags::TYPE => Flags::parse(buf)?.into(),
            IncompleteListUuid16::TYPE => IncompleteListUuid16::parse(buf)?.into(),
            CompleteListUuid16::TYPE => CompleteListUuid16::parse(buf)?.into(),
            IncompleteListUuid128::TYPE => IncompleteListUuid128::parse(buf)?.into(),
            CompleteListUuid128::TYPE => CompleteListUuid128::parse(buf)?.into(),
            ShortenedLocalName::TYPE => ShortenedLocalName::parse(buf)?.into(),
            CompleteLocalName::TYPE => CompleteLocalName::parse(buf)?.into(),
            TxPower::TYPE => TxPower::parse(buf)?.into(),
            Appearance::TYPE => Appearance::parse(buf)?.into(),
            x => return Err(CodecError::UnknownPkt(x)),
        })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        let t = match self {
            Self::Flags(..) => Flags::TYPE,
            Self::IncompleteListUuid16(..) => IncompleteListUuid16::TYPE,
            Self::CompleteListUuid16(..) => CompleteListUuid16::TYPE,
            Self::IncompleteListUuid128(..) => IncompleteListUuid128::TYPE,
            Self::CompleteListUuid128(..) => CompleteListUuid128::TYPE,
            Self::ShortenedLocalName(..) => ShortenedLocalName::TYPE,
            Self::CompleteLocalName(..) => CompleteLocalName::TYPE,
            Self::TxPower(..) => TxPower::TYPE,
            Self::Appearance(..) => Appearance::TYPE,
        };
        buf.put_u8(t);

        match self {
            Self::Flags(v) => v.write_to(buf),
            Self::IncompleteListUuid16(v) => v.write_to(buf),
            Self::CompleteListUuid16(v) => v.write_to(buf),
            Self::IncompleteListUuid128(v) => v.write_to(buf),
            Self::CompleteListUuid128(v) => v.write_to(buf),
            Self::ShortenedLocalName(v) => v.write_to(buf),
            Self::CompleteLocalName(v) => v.write_to(buf),
            Self::TxPower(v) => v.write_to(buf),
            Self::Appearance(v) => v.write_to(buf),
        }
    }
}

#[derive(Debug)]
pub struct AdvertiseList(Vec<Advertise>);

impl AdvertiseList {
    pub fn new(v: Vec<Advertise>) -> Self {
        Self(v)
    }
}

impl TryFrom<AdvertiseList> for (u8, [u8; 31]) {
    type Error = CodecError;

    fn try_from(v: AdvertiseList) -> std::result::Result<(u8, [u8; 31]), Self::Error> {
        let mut buf = BytesMut::with_capacity(31);
        for item in v.0 {
            let mut b = BytesMut::new();
            item.write_to(&mut b)?;

            buf.put_u8(b.len() as u8);
            buf.put(b);
        }

        let len = buf.len() as u8;
        if len > 31 {
            panic!("{} > 31", len);
        };
        buf.resize(31, 0x00); // FIXME
        let mut result = [0; 31];
        result.copy_from_slice(&buf);
        Ok((len as u8, result))
    }
}
