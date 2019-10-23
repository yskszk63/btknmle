use std::fmt;

use bytes::{Buf, BufMut as _, BytesMut};

use super::{Codec, CodecError, Result};

pub use command_complete_event::*;
pub use command_status_event::*;
pub use set_advertising_command::*;
pub use set_bondable_command::*;
pub use set_connectable_command::*;
pub use set_local_name_command::*;
pub use set_low_energy_command::*;
pub use set_powered_command::*;
pub use set_br_edr_command::*;
pub use current_settings::*;
pub use status::*;

mod command_complete_event;
mod command_status_event;
mod set_advertising_command;
mod set_bondable_command;
mod set_connectable_command;
mod set_local_name_command;
mod set_low_energy_command;
mod set_powered_command;
mod set_br_edr_command;
mod current_settings;
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlIndex {
    ControllerId(u16),
    NonController,
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

impl Default for ControlIndex {
    fn default() -> Self {
        ControlIndex::NonController
    }
}

pub trait ManagementCommand<T>: Into<MgmtCommand> {
    fn parse_result(buf: &mut impl Buf) -> Result<T>;
}

trait CommandItem: Codec + Into<MgmtCommand> {
    const CODE: Code;

    fn controller_index(&self) -> ControlIndex;

    fn code(&self) -> Code {
        Self::CODE
    }
}

trait EventItem: Codec + Into<MgmtEvent> {
    const CODE: Code;

    fn with_controller_index(self, idx: ControlIndex) -> Self;

    fn code(&self) -> Code {
        Self::CODE
    }
}

#[derive(Debug)]
pub enum MgmtCommand {
    SetPoweredCommand(SetPoweredCommand),
    SetConnectableCommand(SetConnectableCommand),
    SetBondableCommand(SetBondableCommand),
    SetLowEnergyCommand(SetLowEnergyCommand),
    SetLocalNameCommand(SetLocalNameCommand),
    SetAdvertisingCommand(SetAdvertisingCommand),
    SetBrEdrCommand(SetBrEdrCommand),
}

impl Codec for MgmtCommand {
    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        let code = match self {
            MgmtCommand::SetPoweredCommand(v) => v.code(),
            MgmtCommand::SetConnectableCommand(v) => v.code(),
            MgmtCommand::SetBondableCommand(v) => v.code(),
            MgmtCommand::SetLowEnergyCommand(v) => v.code(),
            MgmtCommand::SetLocalNameCommand(v) => v.code(),
            MgmtCommand::SetAdvertisingCommand(v) => v.code(),
            MgmtCommand::SetBrEdrCommand(v) => v.code(),
        };
        buf.put_u16_le(code.into());

        let mut b = BytesMut::new();
        match self {
            MgmtCommand::SetPoweredCommand(v) => v.write_to(&mut b)?,
            MgmtCommand::SetConnectableCommand(v) => v.write_to(&mut b)?,
            MgmtCommand::SetBondableCommand(v) => v.write_to(&mut b)?,
            MgmtCommand::SetLowEnergyCommand(v) => v.write_to(&mut b)?,
            MgmtCommand::SetLocalNameCommand(v) => v.write_to(&mut b)?,
            MgmtCommand::SetAdvertisingCommand(v) => v.write_to(&mut b)?,
            MgmtCommand::SetBrEdrCommand(v) => v.write_to(&mut b)?,
        };
        let b = b.freeze();

        buf.put_u16_le(
            match self {
                MgmtCommand::SetPoweredCommand(v) => v.controller_index(),
                MgmtCommand::SetConnectableCommand(v) => v.controller_index(),
                MgmtCommand::SetBondableCommand(v) => v.controller_index(),
                MgmtCommand::SetLowEnergyCommand(v) => v.controller_index(),
                MgmtCommand::SetLocalNameCommand(v) => v.controller_index(),
                MgmtCommand::SetAdvertisingCommand(v) => v.controller_index(),
                MgmtCommand::SetBrEdrCommand(v) => v.controller_index(),
            }
            .into(),
        );

        buf.put_u16_le(b.len() as u16);
        buf.put(b);
        Ok(())
    }

    fn parse(_buf: &mut impl Buf) -> Result<Self> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub enum MgmtEvent {
    CommandCompleteEvent(CommandCompleteEvent),
    CommandStatusEvent(CommandStatusEvent),
}

impl Codec for MgmtEvent {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let code = buf.get_u16_le().into();
        let controller_index = buf.get_u16_le().into();
        let len = buf.get_u16_le() as usize;

        let mut data = buf.take(len);
        Ok(match code {
            CommandCompleteEvent::CODE => CommandCompleteEvent::parse(&mut data)?
                .with_controller_index(controller_index)
                .into(),
            CommandStatusEvent::CODE => CommandStatusEvent::parse(&mut data)?
                .with_controller_index(controller_index)
                .into(),
            x => return Err(CodecError::UnknownMgmt(x.into())),
        })
    }

    fn write_to(&self, _buf: &mut BytesMut) -> Result<()> {
        unimplemented!()
    }
}
