use std::fmt;

use bytes::{Buf, BufMut as _, Bytes, BytesMut};

use super::{Codec, CodecError, Result};

pub mod acldata;
pub mod command;
pub mod event;

pub const HCI_COMMAND_PKT: u8 = 0x01;
pub const HCI_ACLDATA_PKT: u8 = 0x02;
pub const HCI_SCODATA_PKT: u8 = 0x03;
pub const HCI_EVENT_PKT: u8 = 0x04;
pub const HCI_VENDOR_PKT: u8 = 0xff;

pub enum HciPacket {
    Command(command::Command),
    Acldata(acldata::AclData),
    Scodata(Bytes),
    Event(event::Event),
    Vendor(Bytes),
}

impl fmt::Debug for HciPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Command(v) => write!(f, "Command({:?})", v),
            Self::Acldata(v) => write!(f, "{:?}", v),
            Self::Scodata(v) => write!(f, "HciPacket::Scodata({:?})", v),
            Self::Event(v) => write!(f, "Event({:?})", v),
            Self::Vendor(v) => write!(f, "HciPacket::Vendor({:?})", v),
        }
    }
}

impl Codec for HciPacket {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        if !buf.has_remaining() {
            return Err(CodecError::Underflow);
        }
        let tag = buf.get_u8();

        Ok(match tag {
            HCI_COMMAND_PKT => Self::Command(command::Command::parse(buf)?),
            HCI_ACLDATA_PKT => Self::Acldata(acldata::AclData::parse(buf)?),
            HCI_SCODATA_PKT => Self::Scodata(buf.take(usize::max_value()).collect()),
            HCI_EVENT_PKT => Self::Event(event::Event::parse(buf)?),
            HCI_VENDOR_PKT => Self::Vendor(buf.take(usize::max_value()).collect()),
            x => return Err(CodecError::UnknownPkt(x)),
        })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        let tag = match self {
            Self::Command(..) => HCI_COMMAND_PKT,
            Self::Acldata(..) => HCI_ACLDATA_PKT,
            Self::Scodata(..) => HCI_SCODATA_PKT,
            Self::Event(..) => HCI_EVENT_PKT,
            Self::Vendor(..) => HCI_VENDOR_PKT,
        };
        buf.reserve(1);
        buf.put_u8(tag);

        match self {
            Self::Command(item) => item.write_to(buf)?,
            Self::Acldata(item) => item.write_to(buf)?,
            Self::Scodata(item) => buf.extend_from_slice(&item),
            Self::Event(item) => item.write_to(buf)?,
            Self::Vendor(item) => buf.extend_from_slice(&item),
        };
        Ok(())
    }
}
