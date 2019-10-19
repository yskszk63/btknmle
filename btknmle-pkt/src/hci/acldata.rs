use std::fmt;

use bitflags::bitflags;
use bytes::{Buf, BufMut as _, Bytes, BytesMut};

use super::{Codec, CodecError};

const fn acl_handle_pack(h: u16, f: u8) -> u16 {
    (h & 0x0fff) | ((f as u16) << 12)
}

const fn acl_handle(h: u16) -> u16 {
    h & 0x0fff
}

const fn acl_flags(h: u16) -> u8 {
    (h >> 12) as u8
}

bitflags! {
    pub struct AclFlags: u8 {
        const ACL_START_NO_FLUSH = 0x00;
        const ACL_CONT = 0x01;
        const ACL_START = 0x02;
        const ACL_ACTIVE_BCAST = 0x04;
        const ACL_PICO_BCAST = 0x05;
    }
}

pub struct AclData {
    flags: AclFlags,
    handle: u16,
    data: Bytes,
}

impl fmt::Debug for AclData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AclData(flags={:?}, handle=0x{:X}, data=", self.flags, self.handle)?;
        for d in &self.data {
            write!(f, "{:02X}", d)?;
        }
        write!(f, ")")
    }
}

impl AclData {
    pub fn new(flags: AclFlags, handle: u16, data: Bytes) -> Self {
        AclData {
            flags,
            handle,
            data,
        }
    }

    pub fn flags(&self) -> &AclFlags {
        &self.flags
    }

    pub fn handle(&self) -> u16 {
        self.handle
    }

    pub fn data(&self) -> &Bytes {
        &self.data
    }
}

impl Codec for AclData {
    fn parse(buf: &mut impl Buf) -> Result<Self, CodecError> {
        if buf.remaining() < 4 {
            return Err(CodecError::Underflow);
        }

        let handle = buf.get_u16_le();
        let dlen = buf.get_u16_le() as usize;
        let handle = acl_handle(handle);
        let flags = acl_flags(handle);
        let flags = AclFlags::from_bits(flags).unwrap();
        if buf.remaining() < dlen {
            return Err(CodecError::Underflow);
        }
        let data = buf.take(dlen).iter().collect();
        Ok(AclData {
            flags,
            handle,
            data,
        })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<(), CodecError> {
        let handle = acl_handle_pack(self.handle, self.flags.bits());
        let dlen = self.data.len();
        buf.reserve(4 + dlen);

        buf.put_u16_le(handle);
        buf.put_u16_le(dlen as u16);
        buf.put(&self.data);
        Ok(())
    }
}
