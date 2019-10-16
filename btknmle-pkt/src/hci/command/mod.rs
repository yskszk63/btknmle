use super::HciPacket;
use super::{Codec, CodecError};
use bytes::{Buf, BufMut, BytesMut};

//pub mod link_ctl;
//pub mod link_policy;
pub mod host_ctl;
//pub mod info_param;
//pub mod status_param;
//pub mod testing_cmd;
pub mod le_ctl;
//pub mod vendor_cmd;

const fn cmd_opcode_pack(ogf: u8, ocf: u16) -> u16 {
    (ocf & 0x3ff) | ((ogf as u16) << 10)
}

const fn cmd_opcode_ogf(op: u16) -> u8 {
    (op >> 10) as u8
}

const fn cmd_opcode_ocf(op: u16) -> u16 {
    op & 0x3ff
}

trait CommandItem: Codec + Into<Command> {
    const OPCODE: (u8, u16);
}

#[derive(Debug)]
pub enum Command {
    Reset(host_ctl::Reset),
    LeSetAdvertisingData(le_ctl::LeSetAdvertisingData),
    LeSetAdvertiseEnable(le_ctl::LeSetAdvertiseEnable),
}

impl Codec for Command {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        if buf.remaining() < 3 {
            return Err(CodecError::Underflow);
        }

        let opcode = buf.get_u16_le();
        let len = buf.get_u8() as usize;
        if buf.remaining() < len {
            return Err(CodecError::Underflow);
        }
        let mut data = buf.take(len);

        let ogf = cmd_opcode_ogf(opcode);
        let ocf = cmd_opcode_ocf(opcode);

        Ok(match (ogf, ocf) {
            host_ctl::Reset::OPCODE => host_ctl::Reset::parse(&mut data)?.into(),
            le_ctl::LeSetAdvertisingData::OPCODE => {
                le_ctl::LeSetAdvertisingData::parse(&mut data)?.into()
            }
            le_ctl::LeSetAdvertiseEnable::OPCODE => {
                le_ctl::LeSetAdvertiseEnable::parse(&mut data)?.into()
            }
            _ => panic!(),
        })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        let mut b = BytesMut::new();
        match self {
            Self::Reset(item) => item.write_to(&mut b)?,
            Self::LeSetAdvertisingData(item) => item.write_to(&mut b)?,
            Self::LeSetAdvertiseEnable(item) => item.write_to(&mut b)?,
        }

        let (ogf, ocf) = match self {
            Self::Reset(..) => host_ctl::Reset::OPCODE,
            Self::LeSetAdvertisingData(..) => le_ctl::LeSetAdvertisingData::OPCODE,
            Self::LeSetAdvertiseEnable(..) => le_ctl::LeSetAdvertiseEnable::OPCODE,
        };
        let opcode = cmd_opcode_pack(ogf, ocf);
        buf.reserve(b.len() + 3);
        buf.put_u16_le(opcode);
        buf.put_u8(b.len() as u8);
        buf.put(b);

        Ok(())
    }
}

impl From<Command> for HciPacket {
    fn from(v: Command) -> Self {
        Self::Command(v)
    }
}
