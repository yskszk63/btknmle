use std::fmt;

use bytes::{Buf, BufMut};

use super::{Address, AddressType};
use super::{Code, ControlIndex, EventItem, MgmtEvent};
use crate::{PackError, PacketData, UnpackError};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Passkey(u32);

impl PacketData for Passkey {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        Ok(Self(PacketData::unpack(buf)?))
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.0.pack(buf)
    }
}

impl fmt::Display for Passkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:06}", self.0)
    }
}

impl fmt::Debug for Passkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PasskeyNotifyEvent {
    controller_index: ControlIndex,
    address: Address,
    address_type: AddressType,
    passkey: Passkey,
    entered: bool,
}

impl PasskeyNotifyEvent {
    pub fn new(
        controller_index: ControlIndex,
        address: Address,
        address_type: AddressType,
        passkey: Passkey,
        entered: bool,
    ) -> Self {
        Self {
            controller_index,
            address,
            address_type,
            passkey,
            entered,
        }
    }

    pub fn controller_index(&self) -> ControlIndex {
        self.controller_index.clone()
    }

    pub fn address(&self) -> Address {
        self.address.clone()
    }

    pub fn address_type(&self) -> AddressType {
        self.address_type.clone()
    }

    pub fn passkey(&self) -> &Passkey {
        &self.passkey
    }

    pub fn entered(&self) -> bool {
        self.entered
    }
}

impl EventItem for PasskeyNotifyEvent {
    const CODE: Code = Code(0x0017);

    fn with_controller_index(mut self, idx: ControlIndex) -> Self {
        self.controller_index = idx;
        self
    }
}

impl PacketData for PasskeyNotifyEvent {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let address = PacketData::unpack(buf)?;
        let address_type = PacketData::unpack(buf)?;
        let passkey = PacketData::unpack(buf)?;
        let entered = u8::unpack(buf)? != 0;
        Ok(Self {
            controller_index: Default::default(),
            address,
            address_type,
            passkey,
            entered,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.address.pack(buf)?;
        self.address_type.pack(buf)?;
        self.passkey.pack(buf)?;
        (self.entered as u8).pack(buf)
    }
}

impl From<PasskeyNotifyEvent> for MgmtEvent {
    fn from(v: PasskeyNotifyEvent) -> Self {
        Self::PasskeyNotifyEvent(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = PasskeyNotifyEvent::new(
            Default::default(),
            "00:11:22:33:44:55".parse().unwrap(),
            AddressType::LeRandom,
            Passkey(1234),
            true,
        );
        e.pack(&mut b).unwrap();
        let r = PasskeyNotifyEvent::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
