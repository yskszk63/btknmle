use bytes::{Buf, BytesMut};

use super::{Code, CommandItem, ControlIndex, MgmtCommand};
use super::{Codec, Result};

#[derive(Debug)]
pub struct SetLocalNameCommand {
    ctrl_idx: u16,
    name: String,
    short_name: String,
}

impl SetLocalNameCommand {
    pub fn new(ctrl_idx: u16, name: impl ToString, short_name: impl ToString) -> Self {
        let name = name.to_string();
        let short_name = short_name.to_string();
        Self {
            ctrl_idx,
            name,
            short_name,
        }
    }
}

impl CommandItem for SetLocalNameCommand {
    const CODE: Code = Code(0x000F);

    fn controller_index(&self) -> ControlIndex {
        self.ctrl_idx.into()
    }
}

impl Codec for SetLocalNameCommand {
    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        let mut name = BytesMut::from(self.name.clone());
        name.resize(249, 0);
        buf.extend(name);

        let mut short_name = BytesMut::from(self.short_name.clone());
        short_name.resize(11, 0);
        buf.extend(short_name);

        Ok(())
    }

    fn parse(_buf: &mut impl Buf) -> Result<Self> {
        unimplemented!()
    }
}

impl From<SetLocalNameCommand> for MgmtCommand {
    fn from(v: SetLocalNameCommand) -> Self {
        Self::SetLocalNameCommand(v)
    }
}
