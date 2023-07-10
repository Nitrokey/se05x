// Generated Automatically by `generate_commands.py DO NOT MODIFY MANUALLY

use super::policies::*;
use super::*;
use iso7816::command::{CommandBuilder, ExpectedLen};
use iso7816::tlv::{take_do, Tlv};

#[derive(Clone, Debug)]
pub struct CreateSession {
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
#[derive(Clone, Debug)]
pub struct CreateSessionResponse {
    pub session_id: SessionId,
}

impl<'data> Se050Response<'data> for CreateSessionResponse {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {

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
    type Response<'rdata> = ();
}

#[derive(Clone, Debug)]
pub struct ExchangeSessionData<'data> {
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
#[derive(Clone, Debug)]
pub struct ExchangeSessionDataResponse<'data> {
    pub r_mac: &'data [u8],
}

impl<'data> Se050Response<'data> for ExchangeSessionDataResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let r_mac = rem;
        let _ = rem;
        Ok(Self { r_mac })
    }
}

impl<'data, W: Writer> Se050Command<W> for ExchangeSessionData<'data> {
    type Response<'rdata> = ();
}

#[derive(Clone, Debug)]
pub struct RefreshSession {
    pub policy: SessionPolicy,
}

impl RefreshSession {
    fn data(&self) -> Tlv<SessionPolicy> {
        Tlv::new(TAG_POLICY, self.policy)
    }
    fn command(&self) -> CommandBuilder<Tlv<SessionPolicy>> {
        CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_DEFAULT, P2_SESSION_REFRESH, self.data(), 0)
    }
}

impl DataSource for RefreshSession {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for RefreshSession {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct RefreshSessionResponse {
}

impl<'data> Se050Response<'data> for RefreshSessionResponse {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let _ = rem;
        Ok(Self {  })
    }
}

impl<W: Writer> Se050Command<W> for RefreshSession {
    type Response<'rdata> = ();
}

#[derive(Clone, Debug)]
pub struct CloseSession {
}

impl CloseSession {
    fn data(&self) -> () {
        ()
    }
    fn command(&self) -> CommandBuilder<()> {
        CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_DEFAULT, P2_SESSION_CLOSE, self.data(), 0)
    }
}

impl DataSource for CloseSession {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for CloseSession {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct CloseSessionResponse {
}

impl<'data> Se050Response<'data> for CloseSessionResponse {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let _ = rem;
        Ok(Self {  })
    }
}

impl<W: Writer> Se050Command<W> for CloseSession {
    type Response<'rdata> = ();
}

#[derive(Clone, Debug)]
pub struct VerifySessionUserId<'data> {
    pub user_id: &'data [u8],
}

impl<'data> VerifySessionUserId<'data> {
    fn data(&self) -> Tlv<&'data [u8]> {
        Tlv::new(TAG_1, self.user_id)
    }
    fn command(&self) -> CommandBuilder<Tlv<&'data [u8]>> {
        CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_DEFAULT, P2_SESSION_USERID, self.data(), 0)
    }
}

impl<'data> DataSource for VerifySessionUserId<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for VerifySessionUserId<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct VerifySessionUserIdResponse {
}

impl<'data> Se050Response<'data> for VerifySessionUserIdResponse {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let _ = rem;
        Ok(Self {  })
    }
}

impl<'data, W: Writer> Se050Command<W> for VerifySessionUserId<'data> {
    type Response<'rdata> = ();
}

#[derive(Clone, Debug)]
pub struct SetLockState {
    pub lock_indicator: LockIndicator,
    pub lock_state: LockState,
}

impl SetLockState {
    fn data(&self) -> (Tlv<LockIndicator>, Tlv<LockState>) {
        (Tlv::new(TAG_1, self.lock_indicator), Tlv::new(TAG_2, self.lock_state))
    }
    fn command(&self) -> CommandBuilder<(Tlv<LockIndicator>, Tlv<LockState>)> {
        CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_DEFAULT, P2_TRANSPORT, self.data(), 0)
    }
}

impl DataSource for SetLockState {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for SetLockState {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<W: Writer> Se050Command<W> for SetLockState {
    type Response<'rdata> = ();
}

#[derive(Clone, Debug)]
pub struct WriteEcKey<'data> {
    pub transient: bool,
    pub is_auth: bool,
    pub policy: SessionPolicy,
    pub max_attempts: Option<Be<u16>>,
    pub object_id: Option<ObjectId>,
    pub curve: Option<EcCurve>,
    pub private_key: Option<&'data [u8]>,
    pub public_key: Option<&'data [u8]>,
}

impl<'data> WriteEcKey<'data> {
    fn data(&self) -> (Tlv<SessionPolicy>, Tlv<Option<Be<u16>>>, Tlv<Option<ObjectId>>, Tlv<Option<EcCurve>>, Tlv<Option<&'data [u8]>>, Tlv<Option<&'data [u8]>>) {
        (Tlv::new(TAG_POLICY, self.policy), Tlv::new(TAG_MAX_ATTEMPTS, self.max_attempts), Tlv::new(TAG_1, self.object_id), Tlv::new(TAG_2, self.curve), Tlv::new(TAG_3, self.private_key), Tlv::new(TAG_4, self.public_key))
    }
    fn command(&self) -> CommandBuilder<(Tlv<SessionPolicy>, Tlv<Option<Be<u16>>>, Tlv<Option<ObjectId>>, Tlv<Option<EcCurve>>, Tlv<Option<&'data [u8]>>, Tlv<Option<&'data [u8]>>)> {
        let ins = if self.transient { INS_WRITE & INS_TRANSIENT } else { INS_WRITE };
        let ins = if self.is_auth { ins & INS_AUTH_OBJECT } else { ins };

        CommandBuilder::new(NO_SM_CLA, ins, P1_EC, P2_DEFAULT, self.data(), 0)
    }
}

impl<'data> DataSource for WriteEcKey<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for WriteEcKey<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<'data, W: Writer> Se050Command<W> for WriteEcKey<'data> {
    type Response<'rdata> = ();
}

#[derive(Clone, Debug)]
pub struct GetRandom {
    pub length: Be<u16>,
}

impl GetRandom {
    fn data(&self) -> Tlv<Be<u16>> {
        Tlv::new(TAG_1, self.length)
    }
    fn command(&self) -> CommandBuilder<Tlv<Be<u16>>> {
        CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_DEFAULT, P2_RANDOM, self.data(), ExpectedLen::Max)
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
#[derive(Clone, Debug)]
pub struct GetRandomResponse<'data> {
    pub data: &'data [u8],
}

impl<'data> Se050Response<'data> for GetRandomResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {

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
    type Response<'rdata> = ();
}
