use std::borrow::Cow;
use std::ffi::CStr;
use std::fmt;

use bytes::buf::BufExt as _;
use bytes::{Buf, BufMut};

use crate::{PackError, PacketData, UnpackError};

pub trait NameKind: AsRef<[u8]> + AsMut<[u8]> + Default {
    type Array;
}

pub struct CompleteName([u8; 249]);

impl Default for CompleteName {
    fn default() -> Self {
        Self([0; 249])
    }
}

impl std::cmp::PartialEq<CompleteName> for CompleteName {
    fn eq(&self, other: &CompleteName) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl std::cmp::Eq for CompleteName {}

impl AsRef<[u8]> for CompleteName {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for CompleteName {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl NameKind for CompleteName {
    type Array = [u8; 249];
}

#[derive(Default, PartialEq, Eq)]
pub struct ShortName([u8; 11]);

impl AsRef<[u8]> for ShortName {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for ShortName {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl NameKind for ShortName {
    type Array = [u8; 11];
}

#[derive(Debug, PartialEq, Eq)]
pub struct NameTooLong;

#[derive(PartialEq, Eq)]
pub struct Name<T>(T)
where
    T: NameKind;

impl Name<CompleteName> {
    pub fn with_complete_name<S>(name: S) -> Result<Self, NameTooLong>
    where
        S: AsRef<str>,
    {
        Self::new(name)
    }
}

impl Name<ShortName> {
    pub fn with_short_name<S>(name: S) -> Result<Self, NameTooLong>
    where
        S: AsRef<str>,
    {
        Self::new(name)
    }
}

impl<T> Name<T>
where
    T: NameKind,
{
    fn new<S>(name: S) -> Result<Self, NameTooLong>
    where
        S: AsRef<str>,
    {
        let name = name.as_ref().as_bytes();
        let mut inner = T::default();
        if name.len() > inner.as_ref().len() - 1 {
            // needs terminate with NUL
            return Err(NameTooLong);
        }
        inner.as_mut()[0..name.len()].copy_from_slice(name);

        Ok(Self(inner))
    }

    pub fn to_string_lossy(&self) -> Cow<str> {
        let s = self.0.as_ref();
        let n = s.iter().position(|&x| x == b'\0').unwrap();
        let cstr = CStr::from_bytes_with_nul(&s[..=n]).unwrap();
        cstr.to_string_lossy()
    }
}

impl<T> fmt::Debug for Name<T>
where
    T: NameKind,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_lossy())
    }
}

impl<T> PacketData for Name<T>
where
    T: NameKind,
{
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let mut inner = T::default();
        let mut slice = inner.as_mut();
        let mut b = buf.take(slice.len());
        if b.remaining() < slice.len() {
            return Err(UnpackError::UnexpectedEof);
        }
        b.copy_to_slice(&mut slice);
        Ok(Self(inner))
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        let slice = self.0.as_ref();
        if buf.remaining_mut() < slice.len() {
            return Err(PackError::InsufficientBufLength);
        }
        buf.put(slice);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        Name::<CompleteName>::new("").unwrap();
        Name::<CompleteName>::new(['a'; 248].iter().collect::<String>()).unwrap();
        assert_eq!(
            Err(NameTooLong),
            Name::<CompleteName>::new(['a'; 249].iter().collect::<String>())
        );

        Name::<ShortName>::new("").unwrap();
        Name::<ShortName>::new(['a'; 10].iter().collect::<String>()).unwrap();
        assert_eq!(
            Err(NameTooLong),
            Name::<ShortName>::new(['a'; 11].iter().collect::<String>())
        );

        assert_eq!(
            Name::<CompleteName>::new("abc")
                .unwrap()
                .to_string_lossy()
                .to_owned(),
            "abc"
        );
    }

    #[test]
    fn test() {
        let mut b = vec![];
        let name = Name::<CompleteName>::new("name").unwrap();
        name.pack(&mut b).unwrap();
        let name2 = Name::<CompleteName>::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(name, name2);
    }
}
