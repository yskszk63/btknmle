use bytes::{Buf, BufMut};
use failure::Fail;

#[derive(Debug, Fail, PartialEq, Eq)]
pub enum UnpackError {
    #[fail(display = "unexpected eof")]
    UnexpectedEof,
    #[fail(display = "unexpected {}", _0)]
    Unexpected(String),
}

impl UnpackError {
    pub fn unexpected(cause: impl ToString) -> Self {
        Self::Unexpected(cause.to_string())
    }
}

#[derive(Debug, Fail, PartialEq, Eq)]
pub enum PackError {
    #[fail(display = "insufficient buf length")]
    InsufficientBufLength,
}

pub trait PacketData: Sized {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError>;
    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError>;
}

impl PacketData for u8 {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        if !buf.has_remaining() {
            Err(UnpackError::UnexpectedEof)
        } else {
            Ok(buf.get_u8())
        }
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        if !buf.has_remaining_mut() {
            Err(PackError::InsufficientBufLength)
        } else {
            buf.put_u8(*self);
            Ok(())
        }
    }
}

impl PacketData for u16 {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        if buf.remaining() < 2 {
            Err(UnpackError::UnexpectedEof)
        } else {
            Ok(buf.get_u16_le())
        }
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        if buf.remaining_mut() < 2 {
            Err(PackError::InsufficientBufLength)
        } else {
            buf.put_u16_le(*self);
            Ok(())
        }
    }
}

impl PacketData for u128 {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        if buf.remaining() < 16 {
            Err(UnpackError::UnexpectedEof)
        } else {
            Ok(buf.get_u128_le())
        }
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        if buf.remaining_mut() < 16 {
            Err(PackError::InsufficientBufLength)
        } else {
            buf.put_u128_le(*self);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8() {
        let t = vec![10u8];

        let r = u8::unpack(&mut t.as_ref()).unwrap();
        assert_eq!(r, 10);

        let mut b = vec![];
        r.pack(&mut b).unwrap();
        assert_eq!(b, vec![10]);

        assert_eq!(Err(UnpackError::UnexpectedEof), u8::unpack(&mut &[][..]));
        assert_eq!(Err(PackError::InsufficientBufLength), 0u8.pack(&mut &mut [][..]));
    }

    #[test]
    fn test_u16() {
        let t = vec![10u8, 0u8];

        let r = u16::unpack(&mut t.as_ref()).unwrap();
        assert_eq!(r, 10);

        let mut b = vec![];
        r.pack(&mut b).unwrap();
        assert_eq!(b, vec![10, 0]);

        assert_eq!(Err(UnpackError::UnexpectedEof), u16::unpack(&mut &[0x00][..]));
        assert_eq!(Err(PackError::InsufficientBufLength), 0u16.pack(&mut &mut [0x00][..]));
    }

    #[test]
    fn test_u128() {
        let t = (0..16).collect::<Vec<u8>>();

        let r = u128::unpack(&mut t.as_ref()).unwrap();
        assert_eq!(r, 0x0F_0E_0D_0C_0B_0A_09_08_07_06_05_04_03_02_01_00);

        let mut b = vec![];
        r.pack(&mut b).unwrap();
        assert_eq!(b, t);

        assert_eq!(Err(UnpackError::UnexpectedEof), u128::unpack(&mut [0; 15].as_ref()));
        assert_eq!(Err(PackError::InsufficientBufLength), 0u128.pack(&mut &mut [0; 15][..]));
    }
}
