// Generated Automatically by `generate_commands.py DO NOT MODIFY MANUALLY

use super::policies::*;
use super::*;
use iso7816::command::{CommandBuilder, ExpectedLen};
use iso7816::tlv::{take_do, Tlv};

// ************* CreateSession ************* //

#[derive(Clone, Debug)]
pub struct CreateSession {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
}

impl CreateSession {
    fn data(&self) -> Tlv<ObjectId> {
        Tlv::new(TAG_1, self.object_id)
    }
    fn command(&self) -> CommandBuilder<Tlv<ObjectId>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_DEFAULT,
            P2_SESSION_CREATE,
            self.data(),
            12,
        )
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
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
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

// ************* ExchangeSessionData ************* //

#[derive(Clone, Debug)]
pub struct ExchangeSessionData<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub session_policy: SessionPolicy,
    /// Serialized to remaining data
    pub c_mac: &'data [u8],
}

impl<'data> ExchangeSessionData<'data> {
    fn data(&self) -> (Tlv<SessionPolicy>, &'data [u8]) {
        (Tlv::new(TAG_1, self.session_policy), self.c_mac)
    }
    fn command(&self) -> CommandBuilder<(Tlv<SessionPolicy>, &'data [u8])> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_DEFAULT,
            P2_SESSION_POLICY,
            self.data(),
            0,
        )
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
    /// Parsed from remaining data
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

// ************* RefreshSession ************* //

#[derive(Clone, Debug)]
pub struct RefreshSession {
    /// Serialized to TLV tag [`TAG_POLICY`](TAG_POLICY)
    pub policy: Option<SessionPolicy>,
}

impl RefreshSession {
    fn data(&self) -> Option<Tlv<SessionPolicy>> {
        self.policy.map(|data| Tlv::new(TAG_POLICY, data))
    }
    fn command(&self) -> CommandBuilder<Option<Tlv<SessionPolicy>>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_DEFAULT,
            P2_SESSION_REFRESH,
            self.data(),
            0,
        )
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
pub struct RefreshSessionResponse {}

impl<'data> Se050Response<'data> for RefreshSessionResponse {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let _ = rem;
        Ok(Self {})
    }
}

impl<W: Writer> Se050Command<W> for RefreshSession {
    type Response<'rdata> = RefreshSessionResponse;
}

// ************* CloseSession ************* //

#[derive(Clone, Debug)]
pub struct CloseSession {}

impl CloseSession {
    fn data(&self) -> () {
        ()
    }
    fn command(&self) -> CommandBuilder<()> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_DEFAULT,
            P2_SESSION_CLOSE,
            self.data(),
            0,
        )
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
pub struct CloseSessionResponse {}

impl<'data> Se050Response<'data> for CloseSessionResponse {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let _ = rem;
        Ok(Self {})
    }
}

impl<W: Writer> Se050Command<W> for CloseSession {
    type Response<'rdata> = CloseSessionResponse;
}

// ************* VerifySessionUserId ************* //

#[derive(Clone, Debug)]
pub struct VerifySessionUserId<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub user_id: &'data [u8],
}

impl<'data> VerifySessionUserId<'data> {
    fn data(&self) -> Tlv<&'data [u8]> {
        Tlv::new(TAG_1, self.user_id)
    }
    fn command(&self) -> CommandBuilder<Tlv<&'data [u8]>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_DEFAULT,
            P2_SESSION_USERID,
            self.data(),
            0,
        )
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
pub struct VerifySessionUserIdResponse {}

impl<'data> Se050Response<'data> for VerifySessionUserIdResponse {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let _ = rem;
        Ok(Self {})
    }
}

impl<'data, W: Writer> Se050Command<W> for VerifySessionUserId<'data> {
    type Response<'rdata> = VerifySessionUserIdResponse;
}

// ************* SetLockState ************* //

#[derive(Clone, Debug)]
pub struct SetLockState {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub lock_indicator: TransientIndicator,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub lock_state: LockState,
}

impl SetLockState {
    fn data(&self) -> (Tlv<TransientIndicator>, Tlv<LockState>) {
        (
            Tlv::new(TAG_1, self.lock_indicator),
            Tlv::new(TAG_2, self.lock_state),
        )
    }
    fn command(&self) -> CommandBuilder<(Tlv<TransientIndicator>, Tlv<LockState>)> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_DEFAULT,
            P2_TRANSPORT,
            self.data(),
            0,
        )
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

// ************* WriteEcKey ************* //

#[derive(Clone, Debug)]
pub struct WriteEcKey<'data> {
    pub transient: bool,
    pub is_auth: bool,
    pub key_type: Option<P1KeyType>,
    /// Serialized to TLV tag [`TAG_POLICY`](TAG_POLICY)
    pub policy: Option<PolicySet<'data>>,
    /// Serialized to TLV tag [`TAG_MAX_ATTEMPTS`](TAG_MAX_ATTEMPTS)
    pub max_attempts: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub curve: Option<EcCurve>,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub private_key: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_4`](TAG_4)
    pub public_key: Option<&'data [u8]>,
}

impl<'data> WriteEcKey<'data> {
    fn data(
        &self,
    ) -> (
        Option<Tlv<PolicySet<'data>>>,
        Option<Tlv<Be<u16>>>,
        Tlv<ObjectId>,
        Option<Tlv<EcCurve>>,
        Option<Tlv<&'data [u8]>>,
        Option<Tlv<&'data [u8]>>,
    ) {
        (
            self.policy.map(|data| Tlv::new(TAG_POLICY, data)),
            self.max_attempts
                .map(|data| Tlv::new(TAG_MAX_ATTEMPTS, data)),
            Tlv::new(TAG_1, self.object_id),
            self.curve.map(|data| Tlv::new(TAG_2, data)),
            self.private_key.map(|data| Tlv::new(TAG_3, data)),
            self.public_key.map(|data| Tlv::new(TAG_4, data)),
        )
    }
    fn command(
        &self,
    ) -> CommandBuilder<(
        Option<Tlv<PolicySet<'data>>>,
        Option<Tlv<Be<u16>>>,
        Tlv<ObjectId>,
        Option<Tlv<EcCurve>>,
        Option<Tlv<&'data [u8]>>,
        Option<Tlv<&'data [u8]>>,
    )> {
        let ins = if self.transient {
            INS_WRITE | INS_TRANSIENT
        } else {
            INS_WRITE
        };
        let ins = if self.is_auth {
            ins | INS_AUTH_OBJECT
        } else {
            ins
        };
        let p1: u8 = self.key_type.map(|v| v | P1_EC).unwrap_or(P1_EC);

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

// ************* WriteRsaKey ************* //

#[derive(Clone, Debug)]
pub struct WriteRsaKey<'data> {
    pub transient: bool,
    pub is_auth: bool,
    pub key_type: Option<P1KeyType>,
    /// Serialized to TLV tag [`TAG_POLICY`](TAG_POLICY)
    pub policy: Option<PolicySet<'data>>,
    /// Serialized to TLV tag [`TAG_MAX_ATTEMPTS`](TAG_MAX_ATTEMPTS)
    pub max_attempts: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub key_size: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub p: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_4`](TAG_4)
    pub q: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_5`](TAG_5)
    pub dp: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_6`](TAG_6)
    pub dq: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_7`](TAG_7)
    pub inv_q: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_8`](TAG_8)
    pub e: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_9`](TAG_9)
    pub d: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_10`](TAG_10)
    pub n: Option<&'data [u8]>,
}

impl<'data> WriteRsaKey<'data> {
    fn data(
        &self,
    ) -> (
        Option<Tlv<PolicySet<'data>>>,
        Option<Tlv<Be<u16>>>,
        Tlv<ObjectId>,
        Option<Tlv<Be<u16>>>,
        Option<Tlv<&'data [u8]>>,
        Option<Tlv<&'data [u8]>>,
        Option<Tlv<&'data [u8]>>,
        Option<Tlv<&'data [u8]>>,
        Option<Tlv<&'data [u8]>>,
        Option<Tlv<&'data [u8]>>,
        Option<Tlv<&'data [u8]>>,
        Option<Tlv<&'data [u8]>>,
    ) {
        (
            self.policy.map(|data| Tlv::new(TAG_POLICY, data)),
            self.max_attempts
                .map(|data| Tlv::new(TAG_MAX_ATTEMPTS, data)),
            Tlv::new(TAG_1, self.object_id),
            self.key_size.map(|data| Tlv::new(TAG_2, data)),
            self.p.map(|data| Tlv::new(TAG_3, data)),
            self.q.map(|data| Tlv::new(TAG_4, data)),
            self.dp.map(|data| Tlv::new(TAG_5, data)),
            self.dq.map(|data| Tlv::new(TAG_6, data)),
            self.inv_q.map(|data| Tlv::new(TAG_7, data)),
            self.e.map(|data| Tlv::new(TAG_8, data)),
            self.d.map(|data| Tlv::new(TAG_9, data)),
            self.n.map(|data| Tlv::new(TAG_10, data)),
        )
    }
    fn command(
        &self,
    ) -> CommandBuilder<(
        Option<Tlv<PolicySet<'data>>>,
        Option<Tlv<Be<u16>>>,
        Tlv<ObjectId>,
        Option<Tlv<Be<u16>>>,
        Option<Tlv<&'data [u8]>>,
        Option<Tlv<&'data [u8]>>,
        Option<Tlv<&'data [u8]>>,
        Option<Tlv<&'data [u8]>>,
        Option<Tlv<&'data [u8]>>,
        Option<Tlv<&'data [u8]>>,
        Option<Tlv<&'data [u8]>>,
        Option<Tlv<&'data [u8]>>,
    )> {
        let ins = if self.transient {
            INS_WRITE | INS_TRANSIENT
        } else {
            INS_WRITE
        };
        let ins = if self.is_auth {
            ins | INS_AUTH_OBJECT
        } else {
            ins
        };
        let p1: u8 = self.key_type.map(|v| v | P1_RSA).unwrap_or(P1_RSA);

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

// ************* GenRsaKey ************* //

#[derive(Clone, Debug)]
pub struct GenRsaKey<'data> {
    pub transient: bool,
    pub is_auth: bool,
    /// Serialized to TLV tag [`TAG_POLICY`](TAG_POLICY)
    pub policy: Option<PolicySet<'data>>,
    /// Serialized to TLV tag [`TAG_MAX_ATTEMPTS`](TAG_MAX_ATTEMPTS)
    pub max_attempts: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub key_size: Option<Be<u16>>,
}

impl<'data> GenRsaKey<'data> {
    fn data(
        &self,
    ) -> (
        Option<Tlv<PolicySet<'data>>>,
        Option<Tlv<Be<u16>>>,
        Tlv<ObjectId>,
        Option<Tlv<Be<u16>>>,
    ) {
        (
            self.policy.map(|data| Tlv::new(TAG_POLICY, data)),
            self.max_attempts
                .map(|data| Tlv::new(TAG_MAX_ATTEMPTS, data)),
            Tlv::new(TAG_1, self.object_id),
            self.key_size.map(|data| Tlv::new(TAG_2, data)),
        )
    }
    fn command(
        &self,
    ) -> CommandBuilder<(
        Option<Tlv<PolicySet<'data>>>,
        Option<Tlv<Be<u16>>>,
        Tlv<ObjectId>,
        Option<Tlv<Be<u16>>>,
    )> {
        let ins = if self.transient {
            INS_WRITE | INS_TRANSIENT
        } else {
            INS_WRITE
        };
        let ins = if self.is_auth {
            ins | INS_AUTH_OBJECT
        } else {
            ins
        };

        CommandBuilder::new(NO_SM_CLA, ins, P1_RSA | P1_KEY_PAIR, P2_RAW, self.data(), 0)
    }
}

impl<'data> DataSource for GenRsaKey<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for GenRsaKey<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<'data, W: Writer> Se050Command<W> for GenRsaKey<'data> {
    type Response<'rdata> = ();
}

// ************* WriteSymmKey ************* //

#[derive(Clone, Debug)]
pub struct WriteSymmKey<'data> {
    pub transient: bool,
    pub is_auth: bool,
    pub key_type: SymmKeyType,
    /// Serialized to TLV tag [`TAG_POLICY`](TAG_POLICY)
    pub policy: Option<PolicySet<'data>>,
    /// Serialized to TLV tag [`TAG_MAX_ATTEMPTS`](TAG_MAX_ATTEMPTS)
    pub max_attempts: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub kek_id: ObjectId,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub value: &'data [u8],
}

impl<'data> WriteSymmKey<'data> {
    fn data(
        &self,
    ) -> (
        Option<Tlv<PolicySet<'data>>>,
        Option<Tlv<Be<u16>>>,
        Tlv<ObjectId>,
        Tlv<ObjectId>,
        Tlv<&'data [u8]>,
    ) {
        (
            self.policy.map(|data| Tlv::new(TAG_POLICY, data)),
            self.max_attempts
                .map(|data| Tlv::new(TAG_MAX_ATTEMPTS, data)),
            Tlv::new(TAG_1, self.object_id),
            Tlv::new(TAG_2, self.kek_id),
            Tlv::new(TAG_3, self.value),
        )
    }
    fn command(
        &self,
    ) -> CommandBuilder<(
        Option<Tlv<PolicySet<'data>>>,
        Option<Tlv<Be<u16>>>,
        Tlv<ObjectId>,
        Tlv<ObjectId>,
        Tlv<&'data [u8]>,
    )> {
        let ins = if self.transient {
            INS_WRITE | INS_TRANSIENT
        } else {
            INS_WRITE
        };
        let ins = if self.is_auth {
            ins | INS_AUTH_OBJECT
        } else {
            ins
        };
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

// ************* WriteBinary ************* //

#[derive(Clone, Debug)]
pub struct WriteBinary<'data> {
    pub transient: bool,
    /// Serialized to TLV tag [`TAG_POLICY`](TAG_POLICY)
    pub policy: Option<PolicySet<'data>>,
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub offset: Option<Be<u16>>,
    /// Only when the object does not yet exists
    ///
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub file_length: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_4`](TAG_4)
    pub data: Option<&'data [u8]>,
}

impl<'data> WriteBinary<'data> {
    fn data(
        &self,
    ) -> (
        Option<Tlv<PolicySet<'data>>>,
        Tlv<ObjectId>,
        Option<Tlv<Be<u16>>>,
        Option<Tlv<Be<u16>>>,
        Option<Tlv<&'data [u8]>>,
    ) {
        (
            self.policy.map(|data| Tlv::new(TAG_POLICY, data)),
            Tlv::new(TAG_1, self.object_id),
            self.offset.map(|data| Tlv::new(TAG_2, data)),
            self.file_length.map(|data| Tlv::new(TAG_3, data)),
            self.data.map(|data| Tlv::new(TAG_4, data)),
        )
    }
    fn command(
        &self,
    ) -> CommandBuilder<(
        Option<Tlv<PolicySet<'data>>>,
        Tlv<ObjectId>,
        Option<Tlv<Be<u16>>>,
        Option<Tlv<Be<u16>>>,
        Option<Tlv<&'data [u8]>>,
    )> {
        let ins = if self.transient {
            INS_WRITE | INS_TRANSIENT
        } else {
            INS_WRITE
        };

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

// ************* WriteUserId ************* //

#[derive(Clone, Debug)]
pub struct WriteUserId<'data> {
    /// Serialized to TLV tag [`TAG_POLICY`](TAG_POLICY)
    pub policy: Option<PolicySet<'data>>,
    /// Serialized to TLV tag [`TAG_MAX_ATTEMPTS`](TAG_MAX_ATTEMPTS)
    pub max_attempts: Option<Be<u8>>,
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub data: &'data [u8],
}

impl<'data> WriteUserId<'data> {
    fn data(
        &self,
    ) -> (
        Option<Tlv<PolicySet<'data>>>,
        Option<Tlv<Be<u8>>>,
        Tlv<ObjectId>,
        Tlv<&'data [u8]>,
    ) {
        (
            self.policy.map(|data| Tlv::new(TAG_POLICY, data)),
            self.max_attempts
                .map(|data| Tlv::new(TAG_MAX_ATTEMPTS, data)),
            Tlv::new(TAG_1, self.object_id),
            Tlv::new(TAG_2, self.data),
        )
    }
    fn command(
        &self,
    ) -> CommandBuilder<(
        Option<Tlv<PolicySet<'data>>>,
        Option<Tlv<Be<u8>>>,
        Tlv<ObjectId>,
        Tlv<&'data [u8]>,
    )> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_WRITE | INS_AUTH_OBJECT,
            P1_USERID,
            P2_DEFAULT,
            self.data(),
            0,
        )
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

// ************* WriteCounter ************* //

#[derive(Clone, Debug)]
pub struct WriteCounter<'data> {
    pub transient: bool,
    /// Serialized to TLV tag [`TAG_POLICY`](TAG_POLICY)
    pub policy: Option<PolicySet<'data>>,
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub data: Option<CounterSize>,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub value: Option<Be<u64>>,
}

impl<'data> WriteCounter<'data> {
    fn data(
        &self,
    ) -> (
        Option<Tlv<PolicySet<'data>>>,
        Tlv<ObjectId>,
        Option<Tlv<CounterSize>>,
        Option<Tlv<Be<u64>>>,
    ) {
        (
            self.policy.map(|data| Tlv::new(TAG_POLICY, data)),
            Tlv::new(TAG_1, self.object_id),
            self.data.map(|data| Tlv::new(TAG_2, data)),
            self.value.map(|data| Tlv::new(TAG_3, data)),
        )
    }
    fn command(
        &self,
    ) -> CommandBuilder<(
        Option<Tlv<PolicySet<'data>>>,
        Tlv<ObjectId>,
        Option<Tlv<CounterSize>>,
        Option<Tlv<Be<u64>>>,
    )> {
        let ins = if self.transient {
            INS_WRITE | INS_TRANSIENT
        } else {
            INS_WRITE
        };

        CommandBuilder::new(NO_SM_CLA, ins, P1_COUNTER, P2_DEFAULT, self.data(), 0)
    }
}

impl<'data> DataSource for WriteCounter<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for WriteCounter<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<'data, W: Writer> Se050Command<W> for WriteCounter<'data> {
    type Response<'rdata> = ();
}

// ************* WritePcr ************* //

#[derive(Clone, Debug)]
pub struct WritePcr<'data> {
    pub transient: bool,
    /// Serialized to TLV tag [`TAG_POLICY`](TAG_POLICY)
    pub policy: Option<PolicySet<'data>>,
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub initial_value: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub extend: Option<&'data [u8]>,
}

impl<'data> WritePcr<'data> {
    fn data(
        &self,
    ) -> (
        Option<Tlv<PolicySet<'data>>>,
        Tlv<ObjectId>,
        Option<Tlv<&'data [u8]>>,
        Option<Tlv<&'data [u8]>>,
    ) {
        (
            self.policy.map(|data| Tlv::new(TAG_POLICY, data)),
            Tlv::new(TAG_1, self.object_id),
            self.initial_value.map(|data| Tlv::new(TAG_2, data)),
            self.extend.map(|data| Tlv::new(TAG_3, data)),
        )
    }
    fn command(
        &self,
    ) -> CommandBuilder<(
        Option<Tlv<PolicySet<'data>>>,
        Tlv<ObjectId>,
        Option<Tlv<&'data [u8]>>,
        Option<Tlv<&'data [u8]>>,
    )> {
        let ins = if self.transient {
            INS_WRITE | INS_TRANSIENT
        } else {
            INS_WRITE
        };

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

// ************* ImportObject ************* //

#[derive(Clone, Debug)]
pub struct ImportObject<'data> {
    pub transient: bool,
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub key_component: RsaKeyComponent,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub serialized_object: Option<&'data [u8]>,
}

impl<'data> ImportObject<'data> {
    fn data(
        &self,
    ) -> (
        Tlv<ObjectId>,
        Tlv<RsaKeyComponent>,
        Option<Tlv<&'data [u8]>>,
    ) {
        (
            Tlv::new(TAG_1, self.object_id),
            Tlv::new(TAG_2, self.key_component),
            self.serialized_object.map(|data| Tlv::new(TAG_3, data)),
        )
    }
    fn command(
        &self,
    ) -> CommandBuilder<(
        Tlv<ObjectId>,
        Tlv<RsaKeyComponent>,
        Option<Tlv<&'data [u8]>>,
    )> {
        let ins = if self.transient {
            INS_WRITE | INS_TRANSIENT
        } else {
            INS_WRITE
        };

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

// ************* ReadObject ************* //

#[derive(Clone, Debug)]
pub struct ReadObject<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub offset: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub length: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_4`](TAG_4)
    pub rsa_key_component: Option<&'data [u8]>,
}

impl<'data> ReadObject<'data> {
    fn data(
        &self,
    ) -> (
        Tlv<ObjectId>,
        Option<Tlv<Be<u16>>>,
        Option<Tlv<Be<u16>>>,
        Option<Tlv<&'data [u8]>>,
    ) {
        (
            Tlv::new(TAG_1, self.object_id),
            self.offset.map(|data| Tlv::new(TAG_2, data)),
            self.length.map(|data| Tlv::new(TAG_3, data)),
            self.rsa_key_component.map(|data| Tlv::new(TAG_4, data)),
        )
    }
    fn command(
        &self,
    ) -> CommandBuilder<(
        Tlv<ObjectId>,
        Option<Tlv<Be<u16>>>,
        Option<Tlv<Be<u16>>>,
        Option<Tlv<&'data [u8]>>,
    )> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_READ,
            P1_DEFAULT,
            P2_DEFAULT,
            self.data(),
            ExpectedLen::Max,
        )
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
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
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

// ************* ReadAttestObject ************* //

#[derive(Clone, Debug)]
pub struct ReadAttestObject<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub offset: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub length: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_4`](TAG_4)
    pub rsa_key_component: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_5`](TAG_5)
    pub attestation_object: ObjectId,
    /// Serialized to TLV tag [`TAG_6`](TAG_6)
    pub attestation_algo: AttestationAlgo,
    /// Serialized to TLV tag [`TAG_7`](TAG_7)
    pub freshness_random: Option<&'data [u8; 16]>,
}

impl<'data> ReadAttestObject<'data> {
    fn data(
        &self,
    ) -> (
        Tlv<ObjectId>,
        Option<Tlv<Be<u16>>>,
        Option<Tlv<Be<u16>>>,
        Option<Tlv<&'data [u8]>>,
        Tlv<ObjectId>,
        Tlv<AttestationAlgo>,
        Option<Tlv<&'data [u8; 16]>>,
    ) {
        (
            Tlv::new(TAG_1, self.object_id),
            self.offset.map(|data| Tlv::new(TAG_2, data)),
            self.length.map(|data| Tlv::new(TAG_3, data)),
            self.rsa_key_component.map(|data| Tlv::new(TAG_4, data)),
            Tlv::new(TAG_5, self.attestation_object),
            Tlv::new(TAG_6, self.attestation_algo),
            self.freshness_random.map(|data| Tlv::new(TAG_7, data)),
        )
    }
    fn command(
        &self,
    ) -> CommandBuilder<(
        Tlv<ObjectId>,
        Option<Tlv<Be<u16>>>,
        Option<Tlv<Be<u16>>>,
        Option<Tlv<&'data [u8]>>,
        Tlv<ObjectId>,
        Tlv<AttestationAlgo>,
        Option<Tlv<&'data [u8; 16]>>,
    )> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_READ | INS_ATTEST,
            P1_DEFAULT,
            P2_DEFAULT,
            self.data(),
            0,
        )
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
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub data: &'data [u8],
    /// Parsed from TLV tag [`TAG_2`](TAG_2)
    pub attributes: &'data [u8],
    /// Parsed from TLV tag [`TAG_3`](TAG_3)
    pub timestamp: &'data [u8; 12],
    /// Parsed from TLV tag [`TAG_4`](TAG_4)
    pub freshness_random: &'data [u8; 16],
    /// Parsed from TLV tag [`TAG_5`](TAG_5)
    pub chip_unique_id: &'data [u8; 18],
    /// Parsed from TLV tag [`TAG_6`](TAG_6)
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
        Ok(Self {
            data,
            attributes,
            timestamp,
            freshness_random,
            chip_unique_id,
            signature,
        })
    }
}

impl<'data, W: Writer> Se050Command<W> for ReadAttestObject<'data> {
    type Response<'rdata> = ReadAttestObjectResponse<'rdata>;
}

// ************* ExportObject ************* //

#[derive(Clone, Debug)]
pub struct ExportObject<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub rsa_key_component: Option<&'data [u8]>,
}

impl<'data> ExportObject<'data> {
    fn data(&self) -> (Tlv<ObjectId>, Option<Tlv<&'data [u8]>>) {
        (
            Tlv::new(TAG_1, self.object_id),
            self.rsa_key_component.map(|data| Tlv::new(TAG_2, data)),
        )
    }
    fn command(&self) -> CommandBuilder<(Tlv<ObjectId>, Option<Tlv<&'data [u8]>>)> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_READ,
            P1_DEFAULT,
            P2_EXPORT,
            self.data(),
            ExpectedLen::Max,
        )
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

// ************* ReadType ************* //

#[derive(Clone, Debug)]
pub struct ReadType {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
}

impl ReadType {
    fn data(&self) -> Tlv<ObjectId> {
        Tlv::new(TAG_1, self.object_id)
    }
    fn command(&self) -> CommandBuilder<Tlv<ObjectId>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_READ,
            P1_DEFAULT,
            P2_TYPE,
            self.data(),
            ExpectedLen::Max,
        )
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
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub ty: SecureObjectType,
    /// Parsed from TLV tag [`TAG_2`](TAG_2)
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
        Ok(Self {
            ty,
            transient_indicator,
        })
    }
}

impl<W: Writer> Se050Command<W> for ReadType {
    type Response<'rdata> = ReadTypeResponse;
}

// ************* ReadSize ************* //

#[derive(Clone, Debug)]
pub struct ReadSize {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
}

impl ReadSize {
    fn data(&self) -> Tlv<ObjectId> {
        Tlv::new(TAG_1, self.object_id)
    }
    fn command(&self) -> CommandBuilder<Tlv<ObjectId>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_READ,
            P1_DEFAULT,
            P2_SIZE,
            self.data(),
            ExpectedLen::Max,
        )
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
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
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

// ************* ReadIdList ************* //

#[derive(Clone, Debug)]
pub struct ReadIdList {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub offset: Be<u16>,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub filter: SecureObjectFilter,
}

impl ReadIdList {
    fn data(&self) -> (Tlv<Be<u16>>, Tlv<SecureObjectFilter>) {
        (Tlv::new(TAG_1, self.offset), Tlv::new(TAG_2, self.filter))
    }
    fn command(&self) -> CommandBuilder<(Tlv<Be<u16>>, Tlv<SecureObjectFilter>)> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_READ,
            P1_DEFAULT,
            P2_LIST,
            self.data(),
            ExpectedLen::Max,
        )
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
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub more: MoreIndicator,
    /// Parsed from TLV tag [`TAG_2`](TAG_2)
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

// ************* CheckObjectExists ************* //

#[derive(Clone, Debug)]
pub struct CheckObjectExists {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
}

impl CheckObjectExists {
    fn data(&self) -> Tlv<ObjectId> {
        Tlv::new(TAG_1, self.object_id)
    }
    fn command(&self) -> CommandBuilder<Tlv<ObjectId>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_READ,
            P1_DEFAULT,
            P2_EXIST,
            self.data(),
            ExpectedLen::Max,
        )
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
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
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

// ************* DeleteSecureObject ************* //

#[derive(Clone, Debug)]
pub struct DeleteSecureObject {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
}

impl DeleteSecureObject {
    fn data(&self) -> Tlv<ObjectId> {
        Tlv::new(TAG_1, self.object_id)
    }
    fn command(&self) -> CommandBuilder<Tlv<ObjectId>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_DEFAULT,
            P2_DELETE_OBJECT,
            self.data(),
            0,
        )
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

// ************* CreateEcCurve ************* //

#[derive(Clone, Debug)]
pub struct CreateEcCurve {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub curve: EcCurve,
}

impl CreateEcCurve {
    fn data(&self) -> Tlv<EcCurve> {
        Tlv::new(TAG_1, self.curve)
    }
    fn command(&self) -> CommandBuilder<Tlv<EcCurve>> {
        CommandBuilder::new(NO_SM_CLA, INS_WRITE, P1_CURVE, P2_CREATE, self.data(), 0)
    }
}

impl DataSource for CreateEcCurve {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for CreateEcCurve {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<W: Writer> Se050Command<W> for CreateEcCurve {
    type Response<'rdata> = ();
}

// ************* SetEcCurveParam ************* //

#[derive(Clone, Debug)]
pub struct SetEcCurveParam<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub curve: EcCurve,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub param: EcCurveParam,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub value: &'data [u8],
}

impl<'data> SetEcCurveParam<'data> {
    fn data(&self) -> (Tlv<EcCurve>, Tlv<EcCurveParam>, Tlv<&'data [u8]>) {
        (
            Tlv::new(TAG_1, self.curve),
            Tlv::new(TAG_2, self.param),
            Tlv::new(TAG_3, self.value),
        )
    }
    fn command(&self) -> CommandBuilder<(Tlv<EcCurve>, Tlv<EcCurveParam>, Tlv<&'data [u8]>)> {
        CommandBuilder::new(NO_SM_CLA, INS_WRITE, P1_CURVE, P2_PARAM, self.data(), 0)
    }
}

impl<'data> DataSource for SetEcCurveParam<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for SetEcCurveParam<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<'data, W: Writer> Se050Command<W> for SetEcCurveParam<'data> {
    type Response<'rdata> = ();
}

// ************* GetEcCurveId ************* //

#[derive(Clone, Debug)]
pub struct GetEcCurveId {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
}

impl GetEcCurveId {
    fn data(&self) -> Tlv<ObjectId> {
        Tlv::new(TAG_1, self.object_id)
    }
    fn command(&self) -> CommandBuilder<Tlv<ObjectId>> {
        CommandBuilder::new(NO_SM_CLA, INS_READ, P1_CURVE, P2_ID, self.data(), 0)
    }
}

impl DataSource for GetEcCurveId {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for GetEcCurveId {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct GetEcCurveIdResponse {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub curve: EcCurve,
}

impl<'data> Se050Response<'data> for GetEcCurveIdResponse {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (curve, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value.try_into()?, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { curve })
    }
}

impl<W: Writer> Se050Command<W> for GetEcCurveId {
    type Response<'rdata> = GetEcCurveIdResponse;
}

// ************* ReadEcCurveList ************* //

#[derive(Clone, Debug)]
pub struct ReadEcCurveList {}

impl ReadEcCurveList {
    fn data(&self) -> () {
        ()
    }
    fn command(&self) -> CommandBuilder<()> {
        CommandBuilder::new(NO_SM_CLA, INS_READ, P1_CURVE, P2_LIST, self.data(), 0)
    }
}

impl DataSource for ReadEcCurveList {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for ReadEcCurveList {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct ReadEcCurveListResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub ids: &'data [u8],
}

impl<'data> Se050Response<'data> for ReadEcCurveListResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (ids, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value.try_into()?, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { ids })
    }
}

impl<W: Writer> Se050Command<W> for ReadEcCurveList {
    type Response<'rdata> = ReadEcCurveListResponse<'rdata>;
}

// ************* DeleteEcCurve ************* //

#[derive(Clone, Debug)]
pub struct DeleteEcCurve {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub curve: EcCurve,
}

impl DeleteEcCurve {
    fn data(&self) -> Tlv<EcCurve> {
        Tlv::new(TAG_1, self.curve)
    }
    fn command(&self) -> CommandBuilder<Tlv<EcCurve>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_CURVE,
            P2_DELETE_OBJECT,
            self.data(),
            0,
        )
    }
}

impl DataSource for DeleteEcCurve {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for DeleteEcCurve {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<W: Writer> Se050Command<W> for DeleteEcCurve {
    type Response<'rdata> = ();
}

// ************* GetRandom ************* //

#[derive(Clone, Debug)]
pub struct GetRandom {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub length: Be<u16>,
}

impl GetRandom {
    fn data(&self) -> Tlv<Be<u16>> {
        Tlv::new(TAG_1, self.length)
    }
    fn command(&self) -> CommandBuilder<Tlv<Be<u16>>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_DEFAULT,
            P2_RANDOM,
            self.data(),
            ExpectedLen::Max,
        )
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
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
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

// ************* DeleteAll ************* //

#[derive(Clone, Debug)]
pub struct DeleteAll {}

impl DeleteAll {
    fn data(&self) -> () {
        ()
    }
    fn command(&self) -> CommandBuilder<()> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_DEFAULT,
            P2_DELETE_ALL,
            self.data(),
            ExpectedLen::Max,
        )
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
