use std::fmt;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Clone)]
pub(crate) struct HexDisplay<E>(E);

impl<E> HexDisplay<E> where E: AsRef<[u8]> {
    pub(crate) fn new(item: E) -> HexDisplay<E> {
        Self(item)
    }
}

impl<E> fmt::Debug for HexDisplay<E>
where E: AsRef<[u8]> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, c) in  self.0.as_ref().iter().enumerate() {
            if i != 0 && i % 2 == 0 {
                write!(f, " {:02X}", c)?
            } else {
                write!(f, "{:02X}", c)?
            }
        }
        write!(f, "]")
    }
}

impl<E> AsRef<[u8]> for HexDisplay<E> where E: AsRef<[u8]> {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
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
