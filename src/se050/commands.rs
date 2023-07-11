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
    type Response<'rdata> = CreateSessionResponse;
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
    type Response<'rdata> = ExchangeSessionDataResponse<'rdata>;
}

#[derive(Clone, Debug)]
pub struct RefreshSession {
    pub policy: Option<SessionPolicy>,
}

impl RefreshSession {
    fn data(&self) -> Option<Tlv<SessionPolicy>> {
        self.policy.map(|data| Tlv::new(TAG_POLICY, data))
    }
    fn command(&self) -> CommandBuilder<Option<Tlv<SessionPolicy>>> {
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
    type Response<'rdata> = RefreshSessionResponse;
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
    type Response<'rdata> = CloseSessionResponse;
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
    type Response<'rdata> = VerifySessionUserIdResponse;
}

#[derive(Clone, Debug)]
pub struct SetLockState {
    pub lock_indicator: TransientIndicator,
    pub lock_state: LockState,
}

impl SetLockState {
    fn data(&self) -> (Tlv<TransientIndicator>, Tlv<LockState>) {
        (Tlv::new(TAG_1, self.lock_indicator), Tlv::new(TAG_2, self.lock_state))
    }
    fn command(&self) -> CommandBuilder<(Tlv<TransientIndicator>, Tlv<LockState>)> {
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
    pub key_type: Option<P1KeyType>,
    pub policy: Option<SessionPolicy>,
    pub max_attempts: Option<Be<u16>>,
    pub object_id: Option<ObjectId>,
    pub curve: Option<EcCurve>,
    pub private_key: Option<&'data [u8]>,
    pub public_key: Option<&'data [u8]>,
}

impl<'data> WriteEcKey<'data> {
    fn data(&self) -> (Option<Tlv<SessionPolicy>>, Option<Tlv<Be<u16>>>, Option<Tlv<ObjectId>>, Option<Tlv<EcCurve>>, Option<Tlv<&'data [u8]>>, Option<Tlv<&'data [u8]>>) {
        (self.policy.map(|data| Tlv::new(TAG_POLICY, data)), self.max_attempts.map(|data| Tlv::new(TAG_MAX_ATTEMPTS, data)), self.object_id.map(|data| Tlv::new(TAG_1, data)), self.curve.map(|data| Tlv::new(TAG_2, data)), self.private_key.map(|data| Tlv::new(TAG_3, data)), self.public_key.map(|data| Tlv::new(TAG_4, data)))
    }
    fn command(&self) -> CommandBuilder<(Option<Tlv<SessionPolicy>>, Option<Tlv<Be<u16>>>, Option<Tlv<ObjectId>>, Option<Tlv<EcCurve>>, Option<Tlv<&'data [u8]>>, Option<Tlv<&'data [u8]>>)> {
        let ins = if self.transient { INS_WRITE | INS_TRANSIENT } else { INS_WRITE };
        let ins = if self.is_auth { ins | INS_AUTH_OBJECT } else { ins };
        let p1: u8 = self.key_type.map(|v| v | P1_EC ).unwrap_or(P1_EC);

        CommandBuilder::new(NO_SM_CLA, ins, p1, P2_DEFAULT, self.data(), 0)
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
pub struct WriteRsaKey<'data> {
    pub transient: bool,
    pub is_auth: bool,
    pub key_type: Option<P1KeyType>,
    pub policy: Option<SessionPolicy>,
    pub max_attempts: Option<Be<u16>>,
    pub object_id: Option<ObjectId>,
    pub key_size: Option<Be<u16>>,
    pub p: Option<&'data [u8]>,
    pub q: Option<&'data [u8]>,
    pub dp: Option<&'data [u8]>,
    pub dq: Option<&'data [u8]>,
    pub inv_q: Option<&'data [u8]>,
    pub e: Option<&'data [u8]>,
    pub d: Option<&'data [u8]>,
    pub n: Option<&'data [u8]>,
}

impl<'data> WriteRsaKey<'data> {
    fn data(&self) -> (Option<Tlv<SessionPolicy>>, Option<Tlv<Be<u16>>>, Option<Tlv<ObjectId>>, Option<Tlv<Be<u16>>>, Option<Tlv<&'data [u8]>>, Option<Tlv<&'data [u8]>>, Option<Tlv<&'data [u8]>>, Option<Tlv<&'data [u8]>>, Option<Tlv<&'data [u8]>>, Option<Tlv<&'data [u8]>>, Option<Tlv<&'data [u8]>>, Option<Tlv<&'data [u8]>>) {
        (self.policy.map(|data| Tlv::new(TAG_POLICY, data)), self.max_attempts.map(|data| Tlv::new(TAG_MAX_ATTEMPTS, data)), self.object_id.map(|data| Tlv::new(TAG_1, data)), self.key_size.map(|data| Tlv::new(TAG_2, data)), self.p.map(|data| Tlv::new(TAG_3, data)), self.q.map(|data| Tlv::new(TAG_4, data)), self.dp.map(|data| Tlv::new(TAG_5, data)), self.dq.map(|data| Tlv::new(TAG_6, data)), self.inv_q.map(|data| Tlv::new(TAG_7, data)), self.e.map(|data| Tlv::new(TAG_8, data)), self.d.map(|data| Tlv::new(TAG_9, data)), self.n.map(|data| Tlv::new(TAG_10, data)))
    }
    fn command(&self) -> CommandBuilder<(Option<Tlv<SessionPolicy>>, Option<Tlv<Be<u16>>>, Option<Tlv<ObjectId>>, Option<Tlv<Be<u16>>>, Option<Tlv<&'data [u8]>>, Option<Tlv<&'data [u8]>>, Option<Tlv<&'data [u8]>>, Option<Tlv<&'data [u8]>>, Option<Tlv<&'data [u8]>>, Option<Tlv<&'data [u8]>>, Option<Tlv<&'data [u8]>>, Option<Tlv<&'data [u8]>>)> {
        let ins = if self.transient { INS_WRITE | INS_TRANSIENT } else { INS_WRITE };
        let ins = if self.is_auth { ins | INS_AUTH_OBJECT } else { ins };
        let p1: u8 = self.key_type.map(|v| v | P1_RSA ).unwrap_or(P1_RSA);

        CommandBuilder::new(NO_SM_CLA, ins, p1, P2_DEFAULT, self.data(), 0)
    }
}

impl<'data> DataSource for WriteRsaKey<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for WriteRsaKey<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<'data, W: Writer> Se050Command<W> for WriteRsaKey<'data> {
    type Response<'rdata> = ();
}

#[derive(Clone, Debug)]
pub struct GenRsaKey {
    pub transient: bool,
    pub is_auth: bool,
    pub policy: Option<SessionPolicy>,
    pub max_attempts: Option<Be<u16>>,
    pub object_id: Option<ObjectId>,
    pub key_size: Option<Be<u16>>,
}

impl GenRsaKey {
    fn data(&self) -> (Option<Tlv<SessionPolicy>>, Option<Tlv<Be<u16>>>, Option<Tlv<ObjectId>>, Option<Tlv<Be<u16>>>) {
        (self.policy.map(|data| Tlv::new(TAG_POLICY, data)), self.max_attempts.map(|data| Tlv::new(TAG_MAX_ATTEMPTS, data)), self.object_id.map(|data| Tlv::new(TAG_1, data)), self.key_size.map(|data| Tlv::new(TAG_2, data)))
    }
    fn command(&self) -> CommandBuilder<(Option<Tlv<SessionPolicy>>, Option<Tlv<Be<u16>>>, Option<Tlv<ObjectId>>, Option<Tlv<Be<u16>>>)> {
        let ins = if self.transient { INS_WRITE | INS_TRANSIENT } else { INS_WRITE };
        let ins = if self.is_auth { ins | INS_AUTH_OBJECT } else { ins };

        CommandBuilder::new(NO_SM_CLA, ins, P1_RSA | P1_KEY_PAIR, P2_RAW, self.data(), 0)
    }
}

impl DataSource for GenRsaKey {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for GenRsaKey {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<W: Writer> Se050Command<W> for GenRsaKey {
    type Response<'rdata> = ();
}

#[derive(Clone, Debug)]
pub struct WriteSymmKey<'data> {
    pub transient: bool,
    pub is_auth: bool,
    pub key_type: SymmKeyType,
    pub policy: Option<SessionPolicy>,
    pub max_attempts: Option<Be<u16>>,
    pub object_id: Option<ObjectId>,
    pub kek_id: Option<ObjectId>,
    pub value: &'data [u8],
}

impl<'data> WriteSymmKey<'data> {
    fn data(&self) -> (Option<Tlv<SessionPolicy>>, Option<Tlv<Be<u16>>>, Option<Tlv<ObjectId>>, Option<Tlv<ObjectId>>, Tlv<&'data [u8]>) {
        (self.policy.map(|data| Tlv::new(TAG_POLICY, data)), self.max_attempts.map(|data| Tlv::new(TAG_MAX_ATTEMPTS, data)), self.object_id.map(|data| Tlv::new(TAG_1, data)), self.kek_id.map(|data| Tlv::new(TAG_2, data)), Tlv::new(TAG_3, self.value))
    }
    fn command(&self) -> CommandBuilder<(Option<Tlv<SessionPolicy>>, Option<Tlv<Be<u16>>>, Option<Tlv<ObjectId>>, Option<Tlv<ObjectId>>, Tlv<&'data [u8]>)> {
        let ins = if self.transient { INS_WRITE | INS_TRANSIENT } else { INS_WRITE };
        let ins = if self.is_auth { ins | INS_AUTH_OBJECT } else { ins };
        let p1: u8 = self.key_type.into();

        CommandBuilder::new(NO_SM_CLA, ins, p1, P2_DEFAULT, self.data(), 0)
    }
}

impl<'data> DataSource for WriteSymmKey<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for WriteSymmKey<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<'data, W: Writer> Se050Command<W> for WriteSymmKey<'data> {
    type Response<'rdata> = ();
}

#[derive(Clone, Debug)]
pub struct WriteBinary<'data> {
    pub transient: bool,
    pub is_auth: bool,
    pub policy: Option<SessionPolicy>,
    pub object_id: Option<ObjectId>,
    pub offset: Be<u16>,
    pub file_length: Option<Be<u16>>,
    pub data: Option<&'data [u8]>,
}

impl<'data> WriteBinary<'data> {
    fn data(&self) -> (Option<Tlv<SessionPolicy>>, Option<Tlv<ObjectId>>, Tlv<Be<u16>>, Option<Tlv<Be<u16>>>, Option<Tlv<&'data [u8]>>) {
        (self.policy.map(|data| Tlv::new(TAG_POLICY, data)), self.object_id.map(|data| Tlv::new(TAG_1, data)), Tlv::new(TAG_2, self.offset), self.file_length.map(|data| Tlv::new(TAG_3, data)), self.data.map(|data| Tlv::new(TAG_4, data)))
    }
    fn command(&self) -> CommandBuilder<(Option<Tlv<SessionPolicy>>, Option<Tlv<ObjectId>>, Tlv<Be<u16>>, Option<Tlv<Be<u16>>>, Option<Tlv<&'data [u8]>>)> {
        let ins = if self.transient { INS_WRITE | INS_TRANSIENT } else { INS_WRITE };
        let ins = if self.is_auth { ins | INS_AUTH_OBJECT } else { ins };

        CommandBuilder::new(NO_SM_CLA, ins, P1_BINARY, P2_DEFAULT, self.data(), 0)
    }
}

impl<'data> DataSource for WriteBinary<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for WriteBinary<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<'data, W: Writer> Se050Command<W> for WriteBinary<'data> {
    type Response<'rdata> = ();
}

#[derive(Clone, Debug)]
pub struct WriteUserId<'data> {
    pub policy: Option<SessionPolicy>,
    pub max_attempts: Option<Be<u8>>,
    pub object_id: ObjectId,
    pub data: &'data [u8],
}

impl<'data> WriteUserId<'data> {
    fn data(&self) -> (Option<Tlv<SessionPolicy>>, Option<Tlv<Be<u8>>>, Tlv<ObjectId>, Tlv<&'data [u8]>) {
        (self.policy.map(|data| Tlv::new(TAG_POLICY, data)), self.max_attempts.map(|data| Tlv::new(TAG_MAX_ATTEMPTS, data)), Tlv::new(TAG_1, self.object_id), Tlv::new(TAG_2, self.data))
    }
    fn command(&self) -> CommandBuilder<(Option<Tlv<SessionPolicy>>, Option<Tlv<Be<u8>>>, Tlv<ObjectId>, Tlv<&'data [u8]>)> {
        CommandBuilder::new(NO_SM_CLA, INS_WRITE | INS_AUTH_OBJECT, P1_USERID, P2_DEFAULT, self.data(), 0)
    }
}

impl<'data> DataSource for WriteUserId<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for WriteUserId<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<'data, W: Writer> Se050Command<W> for WriteUserId<'data> {
    type Response<'rdata> = ();
}

#[derive(Clone, Debug)]
pub struct WriteCounter {
    pub transient: bool,
    pub policy: Option<SessionPolicy>,
    pub object_id: Option<ObjectId>,
    pub data: Option<CounterSize>,
    pub value: Option<Be<u64>>,
}

impl WriteCounter {
    fn data(&self) -> (Option<Tlv<SessionPolicy>>, Option<Tlv<ObjectId>>, Option<Tlv<CounterSize>>, Option<Tlv<Be<u64>>>) {
        (self.policy.map(|data| Tlv::new(TAG_POLICY, data)), self.object_id.map(|data| Tlv::new(TAG_1, data)), self.data.map(|data| Tlv::new(TAG_2, data)), self.value.map(|data| Tlv::new(TAG_3, data)))
    }
    fn command(&self) -> CommandBuilder<(Option<Tlv<SessionPolicy>>, Option<Tlv<ObjectId>>, Option<Tlv<CounterSize>>, Option<Tlv<Be<u64>>>)> {
        let ins = if self.transient { INS_WRITE | INS_TRANSIENT } else { INS_WRITE };

        CommandBuilder::new(NO_SM_CLA, ins, P1_COUNTER, P2_DEFAULT, self.data(), 0)
    }
}

impl DataSource for WriteCounter {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for WriteCounter {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<W: Writer> Se050Command<W> for WriteCounter {
    type Response<'rdata> = ();
}

#[derive(Clone, Debug)]
pub struct WritePcr<'data> {
    pub transient: bool,
    pub policy: Option<SessionPolicy>,
    pub object_id: Option<ObjectId>,
    pub initial_value: Option<&'data [u8]>,
    pub extend: Option<&'data [u8]>,
}

impl<'data> WritePcr<'data> {
    fn data(&self) -> (Option<Tlv<SessionPolicy>>, Option<Tlv<ObjectId>>, Option<Tlv<&'data [u8]>>, Option<Tlv<&'data [u8]>>) {
        (self.policy.map(|data| Tlv::new(TAG_POLICY, data)), self.object_id.map(|data| Tlv::new(TAG_1, data)), self.initial_value.map(|data| Tlv::new(TAG_2, data)), self.extend.map(|data| Tlv::new(TAG_3, data)))
    }
    fn command(&self) -> CommandBuilder<(Option<Tlv<SessionPolicy>>, Option<Tlv<ObjectId>>, Option<Tlv<&'data [u8]>>, Option<Tlv<&'data [u8]>>)> {
        let ins = if self.transient { INS_WRITE | INS_TRANSIENT } else { INS_WRITE };

        CommandBuilder::new(NO_SM_CLA, ins, P1_PCR, P2_DEFAULT, self.data(), 0)
    }
}

impl<'data> DataSource for WritePcr<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for WritePcr<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<'data, W: Writer> Se050Command<W> for WritePcr<'data> {
    type Response<'rdata> = ();
}

#[derive(Clone, Debug)]
pub struct ImportObject<'data> {
    pub transient: bool,
    pub object_id: Option<ObjectId>,
    pub key_component: RsaKeyComponent,
    pub serialized_object: Option<&'data [u8]>,
}

impl<'data> ImportObject<'data> {
    fn data(&self) -> (Option<Tlv<ObjectId>>, Tlv<RsaKeyComponent>, Option<Tlv<&'data [u8]>>) {
        (self.object_id.map(|data| Tlv::new(TAG_1, data)), Tlv::new(TAG_2, self.key_component), self.serialized_object.map(|data| Tlv::new(TAG_3, data)))
    }
    fn command(&self) -> CommandBuilder<(Option<Tlv<ObjectId>>, Tlv<RsaKeyComponent>, Option<Tlv<&'data [u8]>>)> {
        let ins = if self.transient { INS_WRITE | INS_TRANSIENT } else { INS_WRITE };

        CommandBuilder::new(NO_SM_CLA, ins, P1_DEFAULT, P2_IMPORT, self.data(), 0)
    }
}

impl<'data> DataSource for ImportObject<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for ImportObject<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<'data, W: Writer> Se050Command<W> for ImportObject<'data> {
    type Response<'rdata> = ();
}

#[derive(Clone, Debug)]
pub struct ReadObject<'data> {
    pub object_id: Option<ObjectId>,
    pub offset: Option<Be<u16>>,
    pub length: Option<Be<u16>>,
    pub rsa_key_component: Option<&'data [u8]>,
}

impl<'data> ReadObject<'data> {
    fn data(&self) -> (Option<Tlv<ObjectId>>, Option<Tlv<Be<u16>>>, Option<Tlv<Be<u16>>>, Option<Tlv<&'data [u8]>>) {
        (self.object_id.map(|data| Tlv::new(TAG_1, data)), self.offset.map(|data| Tlv::new(TAG_2, data)), self.length.map(|data| Tlv::new(TAG_3, data)), self.rsa_key_component.map(|data| Tlv::new(TAG_4, data)))
    }
    fn command(&self) -> CommandBuilder<(Option<Tlv<ObjectId>>, Option<Tlv<Be<u16>>>, Option<Tlv<Be<u16>>>, Option<Tlv<&'data [u8]>>)> {
        CommandBuilder::new(NO_SM_CLA, INS_READ, P1_DEFAULT, P2_DEFAULT, self.data(), 0)
    }
}

impl<'data> DataSource for ReadObject<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for ReadObject<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct ReadObjectResponse<'data> {
    pub data: &'data [u8],
}

impl<'data> Se050Response<'data> for ReadObjectResponse<'data> {
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

impl<'data, W: Writer> Se050Command<W> for ReadObject<'data> {
    type Response<'rdata> = ReadObjectResponse<'rdata>;
}

#[derive(Clone, Debug)]
pub struct ReadAttestObject<'data> {
    pub object_id: Option<ObjectId>,
    pub offset: Option<Be<u16>>,
    pub length: Option<Be<u16>>,
    pub rsa_key_component: Option<&'data [u8]>,
    pub attestation_object: ObjectId,
    pub attestation_algo: AttestationAlgo,
    pub freshness_random: Option<&'data [u8; 16]>,
}

impl<'data> ReadAttestObject<'data> {
    fn data(&self) -> (Option<Tlv<ObjectId>>, Option<Tlv<Be<u16>>>, Option<Tlv<Be<u16>>>, Option<Tlv<&'data [u8]>>, Tlv<ObjectId>, Tlv<AttestationAlgo>, Option<Tlv<&'data [u8; 16]>>) {
        (self.object_id.map(|data| Tlv::new(TAG_1, data)), self.offset.map(|data| Tlv::new(TAG_2, data)), self.length.map(|data| Tlv::new(TAG_3, data)), self.rsa_key_component.map(|data| Tlv::new(TAG_4, data)), Tlv::new(TAG_5, self.attestation_object), Tlv::new(TAG_6, self.attestation_algo), self.freshness_random.map(|data| Tlv::new(TAG_7, data)))
    }
    fn command(&self) -> CommandBuilder<(Option<Tlv<ObjectId>>, Option<Tlv<Be<u16>>>, Option<Tlv<Be<u16>>>, Option<Tlv<&'data [u8]>>, Tlv<ObjectId>, Tlv<AttestationAlgo>, Option<Tlv<&'data [u8; 16]>>)> {
        CommandBuilder::new(NO_SM_CLA, INS_READ | INS_ATTEST, P1_DEFAULT, P2_DEFAULT, self.data(), 0)
    }
}

impl<'data> DataSource for ReadAttestObject<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for ReadAttestObject<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct ReadAttestObjectResponse<'data> {
    pub data: &'data [u8],
    pub attributes: &'data [u8],
    pub timestamp: &'data [u8; 12],
    pub freshness_random: &'data [u8; 16],
    pub chip_unique_id: &'data [u8; 18],
    pub signature: &'data [u8],
}

impl<'data> Se050Response<'data> for ReadAttestObjectResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {

        let (data, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value.try_into()?, rem_inner);
            }
        };

        let (attributes, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_2 {
                break (value.try_into()?, rem_inner);
            }
        };

        let (timestamp, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_3 {
                break (value.try_into()?, rem_inner);
            }
        };

        let (freshness_random, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_4 {
                break (value.try_into()?, rem_inner);
            }
        };

        let (chip_unique_id, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_5 {
                break (value.try_into()?, rem_inner);
            }
        };

        let (signature, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_6 {
                break (value.try_into()?, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { data, attributes, timestamp, freshness_random, chip_unique_id, signature })
    }
}

impl<'data, W: Writer> Se050Command<W> for ReadAttestObject<'data> {
    type Response<'rdata> = ReadAttestObjectResponse<'rdata>;
}

#[derive(Clone, Debug)]
pub struct ExportObject<'data> {
    pub object_id: Option<ObjectId>,
    pub rsa_key_component: Option<&'data [u8]>,
}

impl<'data> ExportObject<'data> {
    fn data(&self) -> (Option<Tlv<ObjectId>>, Option<Tlv<&'data [u8]>>) {
        (self.object_id.map(|data| Tlv::new(TAG_1, data)), self.rsa_key_component.map(|data| Tlv::new(TAG_2, data)))
    }
    fn command(&self) -> CommandBuilder<(Option<Tlv<ObjectId>>, Option<Tlv<&'data [u8]>>)> {
        CommandBuilder::new(NO_SM_CLA, INS_READ, P1_DEFAULT, P2_EXPORT, self.data(), ExpectedLen::Max)
    }
}

impl<'data> DataSource for ExportObject<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for ExportObject<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<'data, W: Writer> Se050Command<W> for ExportObject<'data> {
    type Response<'rdata> = ();
}

#[derive(Clone, Debug)]
pub struct ReadType {
    pub object_id: Option<ObjectId>,
}

impl ReadType {
    fn data(&self) -> Option<Tlv<ObjectId>> {
        self.object_id.map(|data| Tlv::new(TAG_1, data))
    }
    fn command(&self) -> CommandBuilder<Option<Tlv<ObjectId>>> {
        CommandBuilder::new(NO_SM_CLA, INS_READ, P1_DEFAULT, P2_TYPE, self.data(), ExpectedLen::Max)
    }
}

impl DataSource for ReadType {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for ReadType {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct ReadTypeResponse {
    pub ty: SecureObjectType,
    pub transient_indicator: TransientIndicator,
}

impl<'data> Se050Response<'data> for ReadTypeResponse {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {

        let (ty, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value.try_into()?, rem_inner);
            }
        };

        let (transient_indicator, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_2 {
                break (value.try_into()?, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { ty, transient_indicator })
    }
}

impl<W: Writer> Se050Command<W> for ReadType {
    type Response<'rdata> = ReadTypeResponse;
}

#[derive(Clone, Debug)]
pub struct ReadSize {
    pub object_id: Option<ObjectId>,
}

impl ReadSize {
    fn data(&self) -> Option<Tlv<ObjectId>> {
        self.object_id.map(|data| Tlv::new(TAG_1, data))
    }
    fn command(&self) -> CommandBuilder<Option<Tlv<ObjectId>>> {
        CommandBuilder::new(NO_SM_CLA, INS_READ, P1_DEFAULT, P2_SIZE, self.data(), ExpectedLen::Max)
    }
}

impl DataSource for ReadSize {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for ReadSize {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct ReadSizeResponse {
    pub size: Be<u64>,
}

impl<'data> Se050Response<'data> for ReadSizeResponse {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {

        let (size, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value.try_into()?, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { size })
    }
}

impl<W: Writer> Se050Command<W> for ReadSize {
    type Response<'rdata> = ReadSizeResponse;
}

#[derive(Clone, Debug)]
pub struct ReadIdList {
    pub offset: Be<u16>,
    pub filter: SecureObjectFilter,
}

impl ReadIdList {
    fn data(&self) -> (Tlv<Be<u16>>, Tlv<SecureObjectFilter>) {
        (Tlv::new(TAG_1, self.offset), Tlv::new(TAG_2, self.filter))
    }
    fn command(&self) -> CommandBuilder<(Tlv<Be<u16>>, Tlv<SecureObjectFilter>)> {
        CommandBuilder::new(NO_SM_CLA, INS_READ, P1_DEFAULT, P2_LIST, self.data(), ExpectedLen::Max)
    }
}

impl DataSource for ReadIdList {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for ReadIdList {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct ReadIdListResponse<'data> {
    pub more: MoreIndicator,
    pub ids: &'data [u8],
}

impl<'data> Se050Response<'data> for ReadIdListResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {

        let (more, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value.try_into()?, rem_inner);
            }
        };

        let (ids, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_2 {
                break (value.try_into()?, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { more, ids })
    }
}

impl<W: Writer> Se050Command<W> for ReadIdList {
    type Response<'rdata> = ReadIdListResponse<'rdata>;
}

#[derive(Clone, Debug)]
pub struct CheckObjectExists {
    pub object_id: Option<ObjectId>,
}

impl CheckObjectExists {
    fn data(&self) -> Option<Tlv<ObjectId>> {
        self.object_id.map(|data| Tlv::new(TAG_1, data))
    }
    fn command(&self) -> CommandBuilder<Option<Tlv<ObjectId>>> {
        CommandBuilder::new(NO_SM_CLA, INS_READ, P1_DEFAULT, P2_EXIST, self.data(), ExpectedLen::Max)
    }
}

impl DataSource for CheckObjectExists {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for CheckObjectExists {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct CheckObjectExistsResponse {
    pub result: Se050Result,
}

impl<'data> Se050Response<'data> for CheckObjectExistsResponse {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {

        let (result, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value.try_into()?, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { result })
    }
}

impl<W: Writer> Se050Command<W> for CheckObjectExists {
    type Response<'rdata> = CheckObjectExistsResponse;
}

#[derive(Clone, Debug)]
pub struct DeleteSecureObject {
    pub object_id: Option<ObjectId>,
}

impl DeleteSecureObject {
    fn data(&self) -> Option<Tlv<ObjectId>> {
        self.object_id.map(|data| Tlv::new(TAG_1, data))
    }
    fn command(&self) -> CommandBuilder<Option<Tlv<ObjectId>>> {
        CommandBuilder::new(NO_SM_CLA, INS_READ, P1_DEFAULT, P2_DELETE_OBJECT, self.data(), 0)
    }
}

impl DataSource for DeleteSecureObject {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for DeleteSecureObject {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<W: Writer> Se050Command<W> for DeleteSecureObject {
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
    type Response<'rdata> = GetRandomResponse<'rdata>;
}

#[derive(Clone, Debug)]
pub struct DeleteAll {
}

impl DeleteAll {
    fn data(&self) -> () {
        ()
    }
    fn command(&self) -> CommandBuilder<()> {
        CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_DEFAULT, P2_DELETE_ALL, self.data(), ExpectedLen::Max)
    }
}

impl DataSource for DeleteAll {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for DeleteAll {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<W: Writer> Se050Command<W> for DeleteAll {
    type Response<'rdata> = ();
}
