use bytes::{Buf, BytesMut};
use bytes::buf::BufExt as _;

use super::CurrentSettings;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use super::{Codec, Result};
use super::Address;

#[derive(Debug)]
pub struct ReadControllerInformationResult {
    address: Address,
    bluetooth_version: u8,
    manufacturer: u16,
    supported_settings: CurrentSettings,
    current_settings: CurrentSettings,
    class_of_device: [u8; 3],
    name: String,
    short_name: String,
}

impl ReadControllerInformationResult {
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
    pub fn class_of_device(&self) -> [u8; 3] {
        self.class_of_device
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn short_name(&self) -> String {
        self.short_name.clone()
    }
}

impl Codec for ReadControllerInformationResult {
    fn write_to(&self, _buf: &mut BytesMut) -> Result<()> {
        unimplemented!()
    }

    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let address = Address::parse(buf)?;
        let bluetooth_version = buf.get_u8();
        let manufacturer = buf.get_u16_le();
        let supported_settings = CurrentSettings::parse(buf)?;
        let current_settings = CurrentSettings::parse(buf)?;
        let mut class_of_device = [0; 3];
        buf.copy_to_slice(&mut class_of_device);
        let name = buf
            .take(249)
            .bytes()
            .iter()
            .cloned()
            .take_while(|c| c != &0)
            .map(char::from)
            .collect();
        let short_name = buf
            .take(11)
            .bytes()
            .iter()
            .cloned()
            .take_while(|c| c != &0)
            .map(char::from)
            .collect();

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
}

#[derive(Debug)]
pub struct ReadControllerInformationCommand {
    ctrl_idx: u16,
}

impl ReadControllerInformationCommand {
    pub fn new(ctrl_idx: u16) -> Self {
        Self {
            ctrl_idx,
        }
    }
}

impl ManagementCommand<ReadControllerInformationResult> for ReadControllerInformationCommand {
    fn parse_result(buf: &mut impl Buf) -> Result<ReadControllerInformationResult> {
        Ok(ReadControllerInformationResult::parse(buf)?)
    }
}

impl CommandItem for ReadControllerInformationCommand {
    const CODE: Code = Code(0x0004);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl Codec for ReadControllerInformationCommand {
    fn write_to(&self, _buf: &mut BytesMut) -> Result<()> {
        Ok(())
    }

    fn parse(_buf: &mut impl Buf) -> Result<Self> {
        unimplemented!()
    }
}

impl From<ReadControllerInformationCommand> for MgmtCommand {
    fn from(v: ReadControllerInformationCommand) -> Self {
        Self::ReadControllerInformationCommand(v)
    }
}
