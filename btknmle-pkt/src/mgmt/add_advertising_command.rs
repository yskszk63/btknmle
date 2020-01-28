use bytes::{Buf, BufMut as _, Bytes, BytesMut};

use super::AdvertisingFlags;
use super::ManagementCommand;
use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use super::{Codec, Result};
use crate::util::HexDisplay;

#[derive(Debug)]
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

impl ManagementCommand<u8> for AddAdvertisingCommand {
    fn parse_result(buf: &mut impl Buf) -> Result<u8> {
        Ok(buf.get_u8())
    }
}

impl CommandItem for AddAdvertisingCommand {
    const CODE: Code = Code(0x003e);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl Codec for AddAdvertisingCommand {
    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        buf.put_u8(self.instance);
        self.flags.write_to(buf)?;
        buf.put_u16_le(self.duration);
        buf.put_u16_le(self.timeout);
        buf.put_u8(self.adv_data.len() as u8);
        buf.put_u8(self.scan_rsp.len() as u8);
        buf.put(self.adv_data.as_ref());
        buf.put(self.scan_rsp.as_ref());
        Ok(())
    }

    fn parse(_buf: &mut impl Buf) -> Result<Self> {
        unimplemented!()
    }
}

impl From<AddAdvertisingCommand> for MgmtCommand {
    fn from(v: AddAdvertisingCommand) -> Self {
        Self::AddAdvertisingCommand(v)
    }
}
