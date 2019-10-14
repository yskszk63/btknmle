use bytes::{Buf, BufMut as _, BytesMut};

pub mod command;
pub mod event;
pub mod acldata;
pub mod att;

pub const HCI_COMMAND_PKT: u8 = 0x01;
pub const HCI_ACLDATA_PKT: u8 = 0x02;
pub const HCI_SCODATA_PKT: u8 = 0x03;
pub const HCI_EVENT_PKT: u8 = 0x04;
pub const HCI_VENDOR_PKT: u8 = 0xff;

#[derive(Debug)]
pub enum CodecError {
    Underflow,
    UnknownPkt(u8),
    UnknownEvent(u8),
    UnknownAtt(u8),
    Invalid,
}

pub trait Codec: Sized {
    fn parse(data: &mut impl Buf) -> Result<Self, CodecError>;
    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError>;
}

#[derive(Debug)]
pub enum HciPacket {
    Command(command::Command),
    Acldata(acldata::AclData),
    Scodata(()),
    Event(event::Event),
    Vendor(()),
}

impl Codec for HciPacket {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        if !buf.has_remaining() {
            return Err(CodecError::Underflow);
        }
        let tag = buf.get_u8();

        Ok(match tag {
            HCI_COMMAND_PKT => Self::Command(command::Command::parse(buf)?),
            HCI_ACLDATA_PKT => Self::Acldata(acldata::AclData::parse(buf)?),
            HCI_SCODATA_PKT => unimplemented!(),
            HCI_EVENT_PKT => Self::Event(event::Event::parse(buf)?),
            HCI_VENDOR_PKT => unimplemented!(),
            x => return Err(CodecError::UnknownPkt(x)),
        })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
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
            Self::Scodata(_item) => unimplemented!(),
            Self::Event(item) => item.write_to(buf)?,
            Self::Vendor(_item) => unimplemented!(),
        };
        Ok(())
    }
}
