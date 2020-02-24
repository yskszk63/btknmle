use bytes::{Buf, BufMut};

use super::Address;
use super::CurrentSettings;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use super::{CompleteName, Name, ShortName};
use crate::util::HexDisplay;
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct ReadControllerInformationResult {
    address: Address,
    bluetooth_version: u8,
    manufacturer: u16,
    supported_settings: CurrentSettings,
    current_settings: CurrentSettings,
    class_of_device: HexDisplay<[u8; 3]>,
    name: Name<CompleteName>,
    short_name: Name<ShortName>,
}

impl ReadControllerInformationResult {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        address: Address,
        bluetooth_version: u8,
        manufacturer: u16,
        supported_settings: CurrentSettings,
        current_settings: CurrentSettings,
        class_of_device: [u8; 3],
        name: String,
        short_name: String,
    ) -> Self {
        let class_of_device = class_of_device.into();
        let name = Name::with_complete_name(name).unwrap(); // FIXME
        let short_name = Name::with_short_name(short_name).unwrap(); // FIXME
        Self {
            address,
            bluetooth_version,
            manufacturer,
            supported_settings,
            current_settings,
            class_of_device,
            name,
            short_name,
        }
    }

    pub fn address(&self) -> Address {
        self.address.clone()
    }
    pub fn bluetooth_version(&self) -> u8 {
        self.bluetooth_version
    }
    pub fn manufacturer(&self) -> u16 {
        self.manufacturer
    }
    pub fn supported_settings(&self) -> CurrentSettings {
        self.supported_settings
    }
    pub fn current_settings(&self) -> CurrentSettings {
        self.current_settings
    }
    pub fn class_of_device(&self) -> &[u8] {
        self.class_of_device.as_ref()
    }
    pub fn name(&self) -> String {
        self.name.to_string_lossy().to_owned().to_string()
    }
    pub fn short_name(&self) -> String {
        self.short_name.to_string_lossy().to_owned().to_string()
    }
}

impl PacketData for ReadControllerInformationResult {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let address = PacketData::unpack(buf)?;
        let bluetooth_version = PacketData::unpack(buf)?;
        let manufacturer = PacketData::unpack(buf)?;
        let supported_settings = PacketData::unpack(buf)?;
        let current_settings = PacketData::unpack(buf)?;
        if buf.remaining() < 3 {
            return Err(UnpackError::UnexpectedEof);
        }
        let mut class_of_device = HexDisplay::new([0; 3]);
        buf.copy_to_slice(class_of_device.as_mut());
        let name = PacketData::unpack(buf)?;
        let short_name = PacketData::unpack(buf)?;

        Ok(Self {
            address,
            bluetooth_version,
            manufacturer,
            supported_settings,
            current_settings,
            class_of_device,
            name,
            short_name,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.address.pack(buf)?;
        self.bluetooth_version.pack(buf)?;
        self.manufacturer.pack(buf)?;
        self.supported_settings.pack(buf)?;
        self.current_settings.pack(buf)?;
        if buf.remaining_mut() < 3 {
            return Err(PackError::InsufficientBufLength);
        }
        buf.put(self.class_of_device.as_ref());
        self.name.pack(buf)?;
        self.short_name.pack(buf)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ReadControllerInformationCommand {
    ctrl_idx: u16,
}

impl ReadControllerInformationCommand {
    pub fn new(ctrl_idx: u16) -> Self {
        Self { ctrl_idx }
    }
}

impl ManagementCommand<ReadControllerInformationResult> for ReadControllerInformationCommand {
    fn parse_result(
        buf: &mut impl Buf,
    ) -> Result<ReadControllerInformationResult, crate::CodecError> {
        Ok(ReadControllerInformationResult::unpack(buf)?)
    }
}

impl CommandItem for ReadControllerInformationCommand {
    const CODE: Code = Code(0x0004);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl PacketData for ReadControllerInformationCommand {
    fn unpack(_buf: &mut impl Buf) -> Result<Self, UnpackError> {
        Ok(Self {
            ctrl_idx: Default::default(),
        })
    }

    fn pack(&self, _buf: &mut impl BufMut) -> Result<(), PackError> {
        Ok(())
    }
}

impl From<ReadControllerInformationCommand> for MgmtCommand {
    fn from(v: ReadControllerInformationCommand) -> Self {
        Self::ReadControllerInformationCommand(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = ReadControllerInformationCommand::new(Default::default());
        e.pack(&mut b).unwrap();
        let r = ReadControllerInformationCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }

    #[test]
    fn test_result() {
        let mut b = vec![];
        let e = ReadControllerInformationResult::new(
            "00:11:22:33:44:55".parse().unwrap(),
            1,
            2,
            CurrentSettings::POWERED,
            CurrentSettings::POWERED,
            [3; 3],
            "name".to_owned(),
            "shortname".to_owned(),
        );
        e.pack(&mut b).unwrap();
        let r = ReadControllerInformationResult::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
