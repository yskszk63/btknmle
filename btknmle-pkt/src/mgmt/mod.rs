use std::fmt;

use bytes::{Buf, BufMut};

use crate::{PackError, PacketData, UnpackError};

pub use action::*;
pub use address::*;
pub use address_type::*;
pub use advertising_flags::*;
pub use current_settings::*;
pub use key::*;
pub use name::*;
pub use status::*;

mod action;
mod address;
mod address_type;
mod advertising_flags;
pub mod command;
mod current_settings;
pub mod event;
mod key;
mod name;
mod status;

#[derive(Clone, PartialEq, Eq)]
pub struct Code(u16);

impl fmt::Debug for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:04X}", self.0)
    }
}

impl From<u16> for Code {
    fn from(v: u16) -> Self {
        Self(v)
    }
}

impl From<Code> for u16 {
    fn from(v: Code) -> Self {
        v.0
    }
}

impl PacketData for Code {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        u16::unpack(buf).map(Into::into)
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        u16::from(self.clone()).pack(buf)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum ControlIndex {
    ControllerId(u16),
    NonController,
}

impl fmt::Debug for ControlIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ControllerId(v) => v.fmt(f),
            Self::NonController => "N/A".fmt(f),
        }
    }
}

impl From<u16> for ControlIndex {
    fn from(v: u16) -> Self {
        if v == 0xFFFF {
            Self::NonController
        } else {
            Self::ControllerId(v)
        }
    }
}

impl From<ControlIndex> for u16 {
    fn from(v: ControlIndex) -> Self {
        match v {
            ControlIndex::ControllerId(v) => v,
            ControlIndex::NonController => 0xFFFF,
        }
    }
}

impl PacketData for ControlIndex {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        u16::unpack(buf).map(Into::into)
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        u16::from(self.clone()).pack(buf)
    }
}

impl Default for ControlIndex {
    fn default() -> Self {
        ControlIndex::NonController
    }
}
