use bytes::{Buf, BufMut as _, BytesMut};

use super::{AuthReq, IoCapability, LeKeyDistribution, OobDataFlag};
use super::{Codec, Result, Smp, SmpItem};

#[derive(Debug)]
pub struct PairingResponse {
    io_capability: IoCapability,
    oob_data_flag: OobDataFlag,
    authreq: AuthReq,
    maximum_encryption_keysize: u8,
    initiator_key_distribution: LeKeyDistribution,
    responder_key_distribution: LeKeyDistribution,
}

impl PairingResponse {
    pub fn new(
        io_capability: impl Into<IoCapability>,
        oob_data_flag: impl Into<OobDataFlag>,
        authreq: impl Into<AuthReq>,
        maximum_encryption_keysize: u8,
        initiator_key_distribution: LeKeyDistribution,
        responder_key_distribution: LeKeyDistribution,
    ) -> Self {
        Self {
            io_capability: io_capability.into(),
            oob_data_flag: oob_data_flag.into(),
            authreq: authreq.into(),
            maximum_encryption_keysize,
            initiator_key_distribution,
            responder_key_distribution,
        }
    }
}

impl SmpItem for PairingResponse {
    const CODE: u8 = 0x02;
}

impl Codec for PairingResponse {
    fn parse(buf: &mut impl Buf) -> Result<Self> {
        let io_capability = buf.get_u8().into();
        let oob_data_flag = buf.get_u8().into();
        let authreq = buf.get_u8().into();
        let maximum_encryption_keysize = buf.get_u8();
        let initiator_key_distribution = LeKeyDistribution::from_bits_truncate(buf.get_u8());
        let responder_key_distribution = LeKeyDistribution::from_bits_truncate(buf.get_u8());
        Ok(Self {
            io_capability,
            oob_data_flag,
            authreq,
            maximum_encryption_keysize,
            initiator_key_distribution,
            responder_key_distribution,
        })
    }

    fn write_to(&self, buf: &mut BytesMut) -> Result<()> {
        buf.put_u8(self.io_capability.into());
        buf.put_u8(self.oob_data_flag.into());
        buf.put_u8(self.authreq.clone().into());
        buf.put_u8(self.maximum_encryption_keysize);
        buf.put_u8(self.initiator_key_distribution.bits());
        buf.put_u8(self.responder_key_distribution.bits());
        Ok(())
    }
}

impl From<PairingResponse> for Smp {
    fn from(v: PairingResponse) -> Self {
        Self::PairingResponse(v)
    }
}
