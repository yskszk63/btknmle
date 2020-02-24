use std::fmt;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::ops::DerefMut;

use bytes::{Buf, BufMut};

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct HexDisplay<E>(E);

impl<E> HexDisplay<E>
where
    E: AsRef<[u8]>,
{
    pub(crate) fn new(item: E) -> HexDisplay<E> {
        Self(item)
    }
}

impl<E> fmt::Debug for HexDisplay<E>
where
    E: AsRef<[u8]>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, c) in self.0.as_ref().iter().enumerate() {
            if i != 0 && i % 2 == 0 {
                write!(f, " {:02X}", c)?
            } else {
                write!(f, "{:02X}", c)?
            }
        }
        write!(f, "]")
    }
}

impl<E> From<E> for HexDisplay<E> {
    fn from(v: E) -> Self {
        Self(v)
    }
}

impl<E> AsRef<[u8]> for HexDisplay<E>
where
    E: AsRef<[u8]>,
{
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<E> AsMut<[u8]> for HexDisplay<E>
where
    E: AsMut<[u8]>,
{
    fn as_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }
}

impl<E> Deref for HexDisplay<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<E> DerefMut for HexDisplay<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<E> Buf for HexDisplay<E>
where
    E: Buf,
{
    fn remaining(&self) -> usize {
        self.0.remaining()
    }

    fn bytes(&self) -> &[u8] {
        self.0.bytes()
    }

    fn advance(&mut self, cnt: usize) {
        self.0.advance(cnt)
    }
}

impl<E> BufMut for HexDisplay<E>
where
    E: BufMut,
{
    fn remaining_mut(&self) -> usize {
        self.0.remaining_mut()
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        self.0.advance_mut(cnt)
    }

    fn bytes_mut(&mut self) -> &mut [MaybeUninit<u8>] {
        self.0.bytes_mut()
    }
}
