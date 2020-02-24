use std::fmt;
use std::str::FromStr;

use bytes::{Buf, BufMut};

use crate::{PackError, PacketData, UnpackError};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address([u8; 6]);

impl PacketData for Address {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        if buf.remaining() < 6 {
            return Err(UnpackError::UnexpectedEof);
        }

        let mut b = [0; 6];
        buf.copy_to_slice(&mut b);
        Ok(Address(b))
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        if buf.remaining_mut() < 6 {
            return Err(PackError::InsufficientBufLength);
        }

        buf.put(&mut self.as_ref());
        Ok(())
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.0;
        write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            a[5], a[4], a[3], a[2], a[1], a[0]
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

#[derive(failure::Fail, Debug, PartialEq, Eq)]
#[fail(display = "failed to parse address {}", _0)]
pub struct ParseAddressError(String);

impl FromStr for Address {
    type Err = ParseAddressError;

    fn from_str(v: &str) -> Result<Self, Self::Err> {
        let mut result = vec![];
        for i in v.split(':') {
            result.push(if let Ok(i) = u8::from_str_radix(i, 16) {
                i
            } else {
                return Err(ParseAddressError(v.into()));
            });
        }

        if result.len() == 6 {
            let result = [
                result[5], result[4], result[3], result[2], result[1], result[0],
            ];
            Ok(Address(result))
        } else {
            Err(ParseAddressError(v.into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = Address::from_str("00:11:22:33:44:55").unwrap();
        assert_eq!("00:11:22:33:44:55", e.to_string());
        e.pack(&mut b).unwrap();
        let r = Address::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
