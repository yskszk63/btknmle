use std::convert::TryFrom;
use std::fmt;

use bytes::{Buf, BytesMut};

use super::{Codec, Result};

#[derive(Clone)]
pub struct Address([u8; 6]);

impl Codec for Address {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let mut b = [0; 6];
        buf.copy_to_slice(&mut b);
        Ok(Address(b))
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        buf.extend_from_slice(&self.0);
        Ok(())
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.0;
        write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            a[0], a[1], a[2], a[3], a[4], a[5]
        )
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl AsRef<[u8]> for Address {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<String> for Address {
    type Error = String;

    fn try_from(v: String) -> std::result::Result<Self, String> {
        let mut result = vec![];
        for i in v.split(":") {
            result.push(if let Ok(i) = u8::from_str_radix(i, 16) {
                i
            } else {
                return Err(v);
            });
        }

        if result.len() == 6 {
            let result = [
                result[0], result[1], result[2], result[3], result[4], result[5],
            ];
            Ok(Address(result))
        } else {
            Err(v)
        }
    }
}
