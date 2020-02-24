use bytes::buf::BufExt;
use bytes::{Buf, BufMut, Bytes};

use super::AdvertisingFlags;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use crate::util::HexDisplay;
use crate::{PackError, PacketData, UnpackError};

#[derive(Debug, PartialEq, Eq)]
pub struct AddAdvertisingCommand {
    ctrl_idx: u16,
    instance: u8,
    flags: AdvertisingFlags,
    duration: u16,
    timeout: u16,
    adv_data: HexDisplay<Bytes>,
    scan_rsp: HexDisplay<Bytes>,
}

impl AddAdvertisingCommand {
    pub fn new(
        ctrl_idx: u16,
        instance: u8,
        flags: AdvertisingFlags,
        duration: u16,
        timeout: u16,
        adv_data: &[u8],
        scan_rsp: &[u8],
    ) -> Self {
        let adv_data = HexDisplay::new(Bytes::copy_from_slice(adv_data));
        let scan_rsp = HexDisplay::new(Bytes::copy_from_slice(scan_rsp));
        Self {
            ctrl_idx,
            instance,
            flags,
            duration,
            timeout,
            adv_data,
            scan_rsp,
        }
    }
}

impl ManagementCommand for AddAdvertisingCommand {
    type Result = u8;
}

impl CommandItem for AddAdvertisingCommand {
    const CODE: Code = Code(0x003e);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl PacketData for AddAdvertisingCommand {
    fn unpack(buf: &mut impl Buf) -> Result<Self, UnpackError> {
        let instance = PacketData::unpack(buf)?;
        let flags = PacketData::unpack(buf)?;
        let duration = PacketData::unpack(buf)?;
        let timeout = PacketData::unpack(buf)?;
        let adv_data_len = u8::unpack(buf)? as usize;
        let scan_rsp_len = u8::unpack(buf)? as usize;
        if buf.remaining() < adv_data_len + scan_rsp_len {
            return Err(UnpackError::UnexpectedEof);
        }
        let adv_data = buf.take(adv_data_len).to_bytes().into();
        let scan_rsp = buf.take(scan_rsp_len).to_bytes().into();

        Ok(Self {
            ctrl_idx: Default::default(),
            instance,
            flags,
            duration,
            timeout,
            adv_data,
            scan_rsp,
        })
    }

    fn pack(&self, buf: &mut impl BufMut) -> Result<(), PackError> {
        self.instance.pack(buf)?;
        self.flags.pack(buf)?;
        self.duration.pack(buf)?;
        self.timeout.pack(buf)?;
        (self.adv_data.len() as u8).pack(buf)?;
        (self.scan_rsp.len() as u8).pack(buf)?;
        if buf.remaining_mut() < self.adv_data.len() + self.scan_rsp.len() {
            return Err(PackError::InsufficientBufLength);
        }
        buf.put(self.adv_data.as_ref());
        buf.put(self.scan_rsp.as_ref());
        Ok(())
    }
}

impl From<AddAdvertisingCommand> for MgmtCommand {
    fn from(v: AddAdvertisingCommand) -> Self {
        Self::AddAdvertisingCommand(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut b = vec![];
        let e = AddAdvertisingCommand::new(
            0,
            2,
            AdvertisingFlags::SWITCH_INTO_CONNECTABLE_MODE,
            3,
            4,
            b"aa",
            b"bb",
        );
        e.pack(&mut b).unwrap();
        let r = AddAdvertisingCommand::unpack(&mut b.as_ref()).unwrap();
        assert_eq!(e, r);
    }
}
