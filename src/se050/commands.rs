// Generated Automatically by `generate_commands.py DO NOT MODIFY MANUALLY

use super::policies::*;
use super::*;
use iso7816::command::CommandBuilder;
use iso7816::tlv::{Tlv, take_do};

#[derive(Clone, Debug)]pub struct CreateSession {
    pub object_id: ObjectId,
}

impl CreateSession {
    fn data(&self) -> Tlv<ObjectId> {
        Tlv::new(TAG_1, self.object_id)
    }
    fn command(&self) -> CommandBuilder<Tlv<ObjectId>> {
        CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_DEFAULT, P2_DEFAULT, self.data(), 12)
    }
}

impl DataSource for CreateSession {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for CreateSession {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]pub struct CreateSessionResponse {
    pub session_id: SessionId,
}

impl<'data> TryFrom<&'data [u8]> for CreateSessionResponse {    type Error = Error;
    fn try_from(rem: &'data [u8]) -> Result<Self, Error> {

        let (session_id, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value.try_into()?, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { session_id })
    }
}
impl<W: Writer> Se050Command<W> for CreateSession {
    type Response<'rdata> = CreateSessionResponse;
}

#[derive(Clone, Debug)]pub struct ExchangeSessionData<'data> {
    pub session_policy: SessionPolicy,
    pub c_mac: &'data [u8],
}

impl<'data> ExchangeSessionData<'data> {
    fn data(&self) -> (Tlv<SessionPolicy>, &'data [u8]) {
        (Tlv::new(TAG_1, self.session_policy), self.c_mac)
    }
    fn command(&self) -> CommandBuilder<(Tlv<SessionPolicy>, &'data [u8])> {
        CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_DEFAULT, P2_SESSION_POLICY, self.data(), 0)
    }
}

impl<'data> DataSource for ExchangeSessionData<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for ExchangeSessionData<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]pub struct ExchangeSessionDataResponse<'data> {
    pub r_mac: &'data [u8],
}

impl<'data> TryFrom<&'data [u8]> for ExchangeSessionDataResponse<'data> {    type Error = Error;
    fn try_from(rem: &'data [u8]) -> Result<Self, Error> {
        let r_mac = rem;
        let _ = rem;
        Ok(Self { r_mac })
    }
}
impl<'data, W: Writer> Se050Command<W> for ExchangeSessionData<'data> {
    type Response<'rdata> = ExchangeSessionDataResponse<'rdata>;
}

#[derive(Clone, Debug)]pub struct GetRandom {
    pub length: Be<u16>,
}

impl GetRandom {
    fn data(&self) -> Tlv<Be<u16>> {
        Tlv::new(TAG_1, self.length)
    }
    fn command(&self) -> CommandBuilder<Tlv<Be<u16>>> {
        CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_DEFAULT, P2_RANDOM, self.data(), 0)
    }
}

impl DataSource for GetRandom {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for GetRandom {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]pub struct GetRandomResponse<'data> {
    pub data: &'data [u8],
}

impl<'data> TryFrom<&'data [u8]> for GetRandomResponse<'data> {    type Error = Error;
    fn try_from(rem: &'data [u8]) -> Result<Self, Error> {

        let (data, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value.try_into()?, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { data })
    }
}
impl<W: Writer> Se050Command<W> for GetRandom {
    type Response<'rdata> = GetRandomResponse<'rdata>;
}
