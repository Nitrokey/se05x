// Copyright (C) 2023 Nitrokey GmbH
// SPDX-License-Identifier: LGPL-3.0-only

// Generated Automatically by `generate_commands.py DO NOT MODIFY MANUALLY

use super::policies::*;
use super::*;
use iso7816::command::{CommandBuilder, ExpectedLen};
use iso7816::tlv::{take_do, Tlv};

// ************* CreateSession ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CreateSession {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
}

type CreateSessionData = Tlv<ObjectId>;

impl CreateSession {
    fn data(&self) -> CreateSessionData {
        Tlv::new(TAG_1, self.object_id)
    }

    fn command(&self) -> CommandBuilder<CreateSessionData> {
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

impl<'data> Se05XResponse<'data> for CreateSessionResponse {
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

impl<W: Writer> Se05XCommand<W> for CreateSession {
    type Response<'rdata> = CreateSessionResponse;
}

// ************* ExchangeSessionData ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ExchangeSessionData<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub session_policy: SessionPolicy,
    /// Serialized to remaining data
    pub c_mac: &'data [u8],
}

type ExchangeSessionDataData<'data> = (Tlv<SessionPolicy>, &'data [u8]);

impl<'data> ExchangeSessionData<'data> {
    fn data(&self) -> ExchangeSessionDataData<'data> {
        (Tlv::new(TAG_1, self.session_policy), self.c_mac)
    }

    fn command(&self) -> CommandBuilder<ExchangeSessionDataData<'data>> {
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

impl<'data> Se05XResponse<'data> for ExchangeSessionDataResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let r_mac = rem;
        let _ = rem;
        Ok(Self { r_mac })
    }
}

impl<'data, W: Writer> Se05XCommand<W> for ExchangeSessionData<'data> {
    type Response<'rdata> = ExchangeSessionDataResponse<'rdata>;
}

// ************* RefreshSession ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct RefreshSession {
    /// Serialized to TLV tag [`TAG_POLICY`](TAG_POLICY)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub policy: Option<SessionPolicy>,
}

type RefreshSessionData = Option<Tlv<SessionPolicy>>;

impl RefreshSession {
    fn data(&self) -> RefreshSessionData {
        self.policy.map(|data| Tlv::new(TAG_POLICY, data))
    }

    fn command(&self) -> CommandBuilder<RefreshSessionData> {
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

impl<'data> Se05XResponse<'data> for RefreshSessionResponse {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let _ = rem;
        Ok(Self {})
    }
}

impl<W: Writer> Se05XCommand<W> for RefreshSession {
    type Response<'rdata> = RefreshSessionResponse;
}

// ************* CloseSession ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CloseSession {}

type CloseSessionData = ();

impl CloseSession {
    fn command(&self) -> CommandBuilder<CloseSessionData> {
        CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_DEFAULT, P2_SESSION_CLOSE, (), 0)
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

impl<'data> Se05XResponse<'data> for CloseSessionResponse {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let _ = rem;
        Ok(Self {})
    }
}

impl<W: Writer> Se05XCommand<W> for CloseSession {
    type Response<'rdata> = CloseSessionResponse;
}

// ************* VerifySessionUserId ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct VerifySessionUserId<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub user_id: &'data [u8],
}

type VerifySessionUserIdData<'data> = Tlv<&'data [u8]>;

impl<'data> VerifySessionUserId<'data> {
    fn data(&self) -> VerifySessionUserIdData<'data> {
        Tlv::new(TAG_1, self.user_id)
    }

    fn command(&self) -> CommandBuilder<VerifySessionUserIdData<'data>> {
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

impl<'data> Se05XResponse<'data> for VerifySessionUserIdResponse {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let _ = rem;
        Ok(Self {})
    }
}

impl<'data, W: Writer> Se05XCommand<W> for VerifySessionUserId<'data> {
    type Response<'rdata> = VerifySessionUserIdResponse;
}

// ************* ScpInitializeUpdate ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ScpInitializeUpdate {
    /// Serialized to remaining data
    pub host_challenge: [u8; 8],
}

type ScpInitializeUpdateData = [u8; 8];

impl ScpInitializeUpdate {
    fn data(&self) -> ScpInitializeUpdateData {
        self.host_challenge
    }

    fn command(&self) -> CommandBuilder<ScpInitializeUpdateData> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_INITIALIZE_UPDATE,
            P1_DEFAULT,
            P2_DEFAULT,
            self.data(),
            256,
        )
    }
}

impl DataSource for ScpInitializeUpdate {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for ScpInitializeUpdate {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct ScpInitializeUpdateResponse {
    /// Parsed from remaining data
    pub se05x_challenge: Se05xChallenge,
}

impl<'data> Se05XResponse<'data> for ScpInitializeUpdateResponse {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let se05x_challenge = rem.try_into()?;
        let _ = rem;
        Ok(Self { se05x_challenge })
    }
}

impl<W: Writer> Se05XCommand<W> for ScpInitializeUpdate {
    type Response<'rdata> = ScpInitializeUpdateResponse;
}

// ************* ScpExternalAuthenticate ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ScpExternalAuthenticate {
    /// Serialized to remaining data
    pub host_cryptogram: [u8; 8],
    /// Serialized to remaining data
    pub mac: [u8; 8],
}

type ScpExternalAuthenticateData = ([u8; 8], [u8; 8]);

impl ScpExternalAuthenticate {
    fn data(&self) -> ScpExternalAuthenticateData {
        (self.host_cryptogram, self.mac)
    }

    fn command(&self) -> CommandBuilder<ScpExternalAuthenticateData> {
        CommandBuilder::new(
            SM_CLA,
            INS_EXTERNAL_AUTHENTICATE,
            P1_DEFAULT,
            P2_DEFAULT,
            self.data(),
            0,
        )
    }
}

impl DataSource for ScpExternalAuthenticate {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for ScpExternalAuthenticate {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct ScpExternalAuthenticateResponse {}

impl<'data> Se05XResponse<'data> for ScpExternalAuthenticateResponse {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let _ = rem;
        Ok(Self {})
    }
}

impl<W: Writer> Se05XCommand<W> for ScpExternalAuthenticate {
    type Response<'rdata> = ScpExternalAuthenticateResponse;
}

// ************* SetLockState ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct SetLockState {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub lock_indicator: TransientIndicator,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub lock_state: LockState,
}

type SetLockStateData = (Tlv<TransientIndicator>, Tlv<LockState>);

impl SetLockState {
    fn data(&self) -> SetLockStateData {
        (
            Tlv::new(TAG_1, self.lock_indicator),
            Tlv::new(TAG_2, self.lock_state),
        )
    }

    fn command(&self) -> CommandBuilder<SetLockStateData> {
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

impl<W: Writer> Se05XCommand<W> for SetLockState {
    type Response<'rdata> = ();
}

// ************* WriteEcKey ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct WriteEcKey<'data> {
    #[cfg_attr(feature = "builder", builder(default))]
    pub transient: bool,
    #[cfg_attr(feature = "builder", builder(default))]
    pub is_auth: bool,
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub key_type: Option<P1KeyType>,
    /// Serialized to TLV tag [`TAG_POLICY`](TAG_POLICY)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub policy: Option<PolicySet<'data>>,
    /// Serialized to TLV tag [`TAG_MAX_ATTEMPTS`](TAG_MAX_ATTEMPTS)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub max_attempts: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub curve: Option<EcCurve>,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub private_key: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_4`](TAG_4)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub public_key: Option<&'data [u8]>,
}

type WriteEcKeyData<'data> = (
    Option<Tlv<PolicySet<'data>>>,
    Option<Tlv<Be<u16>>>,
    Tlv<ObjectId>,
    Option<Tlv<EcCurve>>,
    Option<Tlv<&'data [u8]>>,
    Option<Tlv<&'data [u8]>>,
);

impl<'data> WriteEcKey<'data> {
    fn data(&self) -> WriteEcKeyData<'data> {
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

    fn command(&self) -> CommandBuilder<WriteEcKeyData<'data>> {
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

impl<'data, W: Writer> Se05XCommand<W> for WriteEcKey<'data> {
    type Response<'rdata> = ();
}

// ************* WriteRsaKey ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct WriteRsaKey<'data> {
    #[cfg_attr(feature = "builder", builder(default))]
    pub transient: bool,
    #[cfg_attr(feature = "builder", builder(default))]
    pub is_auth: bool,
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub key_type: Option<P1KeyType>,
    /// Serialized to TLV tag [`TAG_POLICY`](TAG_POLICY)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub policy: Option<PolicySet<'data>>,
    /// Serialized to TLV tag [`TAG_MAX_ATTEMPTS`](TAG_MAX_ATTEMPTS)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub max_attempts: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub key_size: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub p: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_4`](TAG_4)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub q: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_5`](TAG_5)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub dp: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_6`](TAG_6)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub dq: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_7`](TAG_7)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub inv_q: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_8`](TAG_8)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub e: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_9`](TAG_9)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub d: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_10`](TAG_10)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub n: Option<&'data [u8]>,
}

type WriteRsaKeyData<'data> = (
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
);

impl<'data> WriteRsaKey<'data> {
    fn data(&self) -> WriteRsaKeyData<'data> {
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

    fn command(&self) -> CommandBuilder<WriteRsaKeyData<'data>> {
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

impl<'data, W: Writer> Se05XCommand<W> for WriteRsaKey<'data> {
    type Response<'rdata> = ();
}

// ************* GenRsaKey ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct GenRsaKey<'data> {
    #[cfg_attr(feature = "builder", builder(default))]
    pub transient: bool,
    #[cfg_attr(feature = "builder", builder(default))]
    pub is_auth: bool,
    /// Serialized to TLV tag [`TAG_POLICY`](TAG_POLICY)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub policy: Option<PolicySet<'data>>,
    /// Serialized to TLV tag [`TAG_MAX_ATTEMPTS`](TAG_MAX_ATTEMPTS)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub max_attempts: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub key_size: Option<Be<u16>>,
}

type GenRsaKeyData<'data> = (
    Option<Tlv<PolicySet<'data>>>,
    Option<Tlv<Be<u16>>>,
    Tlv<ObjectId>,
    Option<Tlv<Be<u16>>>,
);

impl<'data> GenRsaKey<'data> {
    fn data(&self) -> GenRsaKeyData<'data> {
        (
            self.policy.map(|data| Tlv::new(TAG_POLICY, data)),
            self.max_attempts
                .map(|data| Tlv::new(TAG_MAX_ATTEMPTS, data)),
            Tlv::new(TAG_1, self.object_id),
            self.key_size.map(|data| Tlv::new(TAG_2, data)),
        )
    }

    fn command(&self) -> CommandBuilder<GenRsaKeyData<'data>> {
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

impl<'data, W: Writer> Se05XCommand<W> for GenRsaKey<'data> {
    type Response<'rdata> = ();
}

// ************* WriteSymmKey ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct WriteSymmKey<'data> {
    #[cfg_attr(feature = "builder", builder(default))]
    pub transient: bool,
    #[cfg_attr(feature = "builder", builder(default))]
    pub is_auth: bool,
    pub key_type: SymmKeyType,
    /// Serialized to TLV tag [`TAG_POLICY`](TAG_POLICY)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub policy: Option<PolicySet<'data>>,
    /// Serialized to TLV tag [`TAG_MAX_ATTEMPTS`](TAG_MAX_ATTEMPTS)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub max_attempts: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub kek_id: Option<ObjectId>,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub value: &'data [u8],
}

type WriteSymmKeyData<'data> = (
    Option<Tlv<PolicySet<'data>>>,
    Option<Tlv<Be<u16>>>,
    Tlv<ObjectId>,
    Option<Tlv<ObjectId>>,
    Tlv<&'data [u8]>,
);

impl<'data> WriteSymmKey<'data> {
    fn data(&self) -> WriteSymmKeyData<'data> {
        (
            self.policy.map(|data| Tlv::new(TAG_POLICY, data)),
            self.max_attempts
                .map(|data| Tlv::new(TAG_MAX_ATTEMPTS, data)),
            Tlv::new(TAG_1, self.object_id),
            self.kek_id.map(|data| Tlv::new(TAG_2, data)),
            Tlv::new(TAG_3, self.value),
        )
    }

    fn command(&self) -> CommandBuilder<WriteSymmKeyData<'data>> {
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

impl<'data, W: Writer> Se05XCommand<W> for WriteSymmKey<'data> {
    type Response<'rdata> = ();
}

// ************* WriteBinary ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct WriteBinary<'data> {
    #[cfg_attr(feature = "builder", builder(default))]
    pub transient: bool,
    /// Serialized to TLV tag [`TAG_POLICY`](TAG_POLICY)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub policy: Option<PolicySet<'data>>,
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub offset: Option<Be<u16>>,
    /// Only when the object does not yet exists
    ///
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub file_length: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_4`](TAG_4)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub data: Option<&'data [u8]>,
}

type WriteBinaryData<'data> = (
    Option<Tlv<PolicySet<'data>>>,
    Tlv<ObjectId>,
    Option<Tlv<Be<u16>>>,
    Option<Tlv<Be<u16>>>,
    Option<Tlv<&'data [u8]>>,
);

impl<'data> WriteBinary<'data> {
    fn data(&self) -> WriteBinaryData<'data> {
        (
            self.policy.map(|data| Tlv::new(TAG_POLICY, data)),
            Tlv::new(TAG_1, self.object_id),
            self.offset.map(|data| Tlv::new(TAG_2, data)),
            self.file_length.map(|data| Tlv::new(TAG_3, data)),
            self.data.map(|data| Tlv::new(TAG_4, data)),
        )
    }

    fn command(&self) -> CommandBuilder<WriteBinaryData<'data>> {
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

impl<'data, W: Writer> Se05XCommand<W> for WriteBinary<'data> {
    type Response<'rdata> = ();
}

// ************* WriteUserId ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct WriteUserId<'data> {
    /// Serialized to TLV tag [`TAG_POLICY`](TAG_POLICY)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub policy: Option<PolicySet<'data>>,
    /// Serialized to TLV tag [`TAG_MAX_ATTEMPTS`](TAG_MAX_ATTEMPTS)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub max_attempts: Option<Be<u8>>,
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub data: &'data [u8],
}

type WriteUserIdData<'data> = (
    Option<Tlv<PolicySet<'data>>>,
    Option<Tlv<Be<u8>>>,
    Tlv<ObjectId>,
    Tlv<&'data [u8]>,
);

impl<'data> WriteUserId<'data> {
    fn data(&self) -> WriteUserIdData<'data> {
        (
            self.policy.map(|data| Tlv::new(TAG_POLICY, data)),
            self.max_attempts
                .map(|data| Tlv::new(TAG_MAX_ATTEMPTS, data)),
            Tlv::new(TAG_1, self.object_id),
            Tlv::new(TAG_2, self.data),
        )
    }

    fn command(&self) -> CommandBuilder<WriteUserIdData<'data>> {
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

impl<'data, W: Writer> Se05XCommand<W> for WriteUserId<'data> {
    type Response<'rdata> = ();
}

// ************* WriteCounter ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct WriteCounter<'data> {
    #[cfg_attr(feature = "builder", builder(default))]
    pub transient: bool,
    /// Serialized to TLV tag [`TAG_POLICY`](TAG_POLICY)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub policy: Option<PolicySet<'data>>,
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub data: Option<CounterSize>,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub value: Option<Be<u64>>,
}

type WriteCounterData<'data> = (
    Option<Tlv<PolicySet<'data>>>,
    Tlv<ObjectId>,
    Option<Tlv<CounterSize>>,
    Option<Tlv<Be<u64>>>,
);

impl<'data> WriteCounter<'data> {
    fn data(&self) -> WriteCounterData<'data> {
        (
            self.policy.map(|data| Tlv::new(TAG_POLICY, data)),
            Tlv::new(TAG_1, self.object_id),
            self.data.map(|data| Tlv::new(TAG_2, data)),
            self.value.map(|data| Tlv::new(TAG_3, data)),
        )
    }

    fn command(&self) -> CommandBuilder<WriteCounterData<'data>> {
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

impl<'data, W: Writer> Se05XCommand<W> for WriteCounter<'data> {
    type Response<'rdata> = ();
}

// ************* WritePcr ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct WritePcr<'data> {
    #[cfg_attr(feature = "builder", builder(default))]
    pub transient: bool,
    /// Serialized to TLV tag [`TAG_POLICY`](TAG_POLICY)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub policy: Option<PolicySet<'data>>,
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub initial_value: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub extend: Option<&'data [u8]>,
}

type WritePcrData<'data> = (
    Option<Tlv<PolicySet<'data>>>,
    Tlv<ObjectId>,
    Option<Tlv<&'data [u8]>>,
    Option<Tlv<&'data [u8]>>,
);

impl<'data> WritePcr<'data> {
    fn data(&self) -> WritePcrData<'data> {
        (
            self.policy.map(|data| Tlv::new(TAG_POLICY, data)),
            Tlv::new(TAG_1, self.object_id),
            self.initial_value.map(|data| Tlv::new(TAG_2, data)),
            self.extend.map(|data| Tlv::new(TAG_3, data)),
        )
    }

    fn command(&self) -> CommandBuilder<WritePcrData<'data>> {
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

impl<'data, W: Writer> Se05XCommand<W> for WritePcr<'data> {
    type Response<'rdata> = ();
}

// ************* ImportObject ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ImportObject<'data> {
    #[cfg_attr(feature = "builder", builder(default))]
    pub transient: bool,
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
    /// Unlike [`ExportObject::rsa_key_component`][], use None if not importing an RSA key
    ///
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub rsa_key_component: Option<RsaKeyComponent>,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub serialized_object: &'data [u8],
}

type ImportObjectData<'data> = (
    Tlv<ObjectId>,
    Option<Tlv<RsaKeyComponent>>,
    Tlv<&'data [u8]>,
);

impl<'data> ImportObject<'data> {
    fn data(&self) -> ImportObjectData<'data> {
        (
            Tlv::new(TAG_1, self.object_id),
            self.rsa_key_component.map(|data| Tlv::new(TAG_2, data)),
            Tlv::new(TAG_3, self.serialized_object),
        )
    }

    fn command(&self) -> CommandBuilder<ImportObjectData<'data>> {
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

impl<'data, W: Writer> Se05XCommand<W> for ImportObject<'data> {
    type Response<'rdata> = ();
}

// ************* ReadObject ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ReadObject {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub offset: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub length: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_4`](TAG_4)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub rsa_key_component: Option<RsaKeyComponent>,
}

type ReadObjectData = (
    Tlv<ObjectId>,
    Option<Tlv<Be<u16>>>,
    Option<Tlv<Be<u16>>>,
    Option<Tlv<RsaKeyComponent>>,
);

impl ReadObject {
    fn data(&self) -> ReadObjectData {
        (
            Tlv::new(TAG_1, self.object_id),
            self.offset.map(|data| Tlv::new(TAG_2, data)),
            self.length.map(|data| Tlv::new(TAG_3, data)),
            self.rsa_key_component.map(|data| Tlv::new(TAG_4, data)),
        )
    }

    fn command(&self) -> CommandBuilder<ReadObjectData> {
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

impl DataSource for ReadObject {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for ReadObject {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct ReadObjectResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub data: &'data [u8],
}

impl<'data> Se05XResponse<'data> for ReadObjectResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (data, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { data })
    }
}

impl<W: Writer> Se05XCommand<W> for ReadObject {
    type Response<'rdata> = ReadObjectResponse<'rdata>;
}

// ************* ReadAttestObject ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ReadAttestObject<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub offset: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub length: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_4`](TAG_4)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub rsa_key_component: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_5`](TAG_5)
    pub attestation_object: ObjectId,
    /// Serialized to TLV tag [`TAG_6`](TAG_6)
    pub attestation_algo: AttestationAlgo,
    /// Serialized to TLV tag [`TAG_7`](TAG_7)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub freshness_random: Option<&'data [u8; 16]>,
}

type ReadAttestObjectData<'data> = (
    Tlv<ObjectId>,
    Option<Tlv<Be<u16>>>,
    Option<Tlv<Be<u16>>>,
    Option<Tlv<&'data [u8]>>,
    Tlv<ObjectId>,
    Tlv<AttestationAlgo>,
    Option<Tlv<&'data [u8; 16]>>,
);

impl<'data> ReadAttestObject<'data> {
    fn data(&self) -> ReadAttestObjectData<'data> {
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

    fn command(&self) -> CommandBuilder<ReadAttestObjectData<'data>> {
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
    pub attributes: ObjectAttributes,
    /// Parsed from TLV tag [`TAG_3`](TAG_3)
    pub timestamp: &'data [u8; 12],
    /// Parsed from TLV tag [`TAG_4`](TAG_4)
    pub freshness_random: &'data [u8; 16],
    /// Parsed from TLV tag [`TAG_5`](TAG_5)
    pub chip_unique_id: &'data [u8; 18],
    /// Parsed from TLV tag [`TAG_6`](TAG_6)
    pub signature: &'data [u8],
}

impl<'data> Se05XResponse<'data> for ReadAttestObjectResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (data, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value, rem_inner);
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
                break (value, rem_inner);
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

impl<'data, W: Writer> Se05XCommand<W> for ReadAttestObject<'data> {
    type Response<'rdata> = ReadAttestObjectResponse<'rdata>;
}

// ************* ReadAttributes ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ReadAttributes<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub offset: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub length: Option<Be<u16>>,
    /// Serialized to TLV tag [`TAG_4`](TAG_4)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub rsa_key_component: Option<&'data [u8]>,
}

type ReadAttributesData<'data> = (
    Tlv<ObjectId>,
    Option<Tlv<Be<u16>>>,
    Option<Tlv<Be<u16>>>,
    Option<Tlv<&'data [u8]>>,
);

impl<'data> ReadAttributes<'data> {
    fn data(&self) -> ReadAttributesData<'data> {
        (
            Tlv::new(TAG_1, self.object_id),
            self.offset.map(|data| Tlv::new(TAG_2, data)),
            self.length.map(|data| Tlv::new(TAG_3, data)),
            self.rsa_key_component.map(|data| Tlv::new(TAG_4, data)),
        )
    }

    fn command(&self) -> CommandBuilder<ReadAttributesData<'data>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_READ,
            P1_DEFAULT,
            P2_ATTRIBUTES,
            self.data(),
            0,
        )
    }
}

impl<'data> DataSource for ReadAttributes<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for ReadAttributes<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct ReadAttributesResponse {
    /// Parsed from TLV tag [`TAG_2`](TAG_2)
    pub attributes: ObjectAttributes,
}

impl<'data> Se05XResponse<'data> for ReadAttributesResponse {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (attributes, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_2 {
                break (value.try_into()?, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { attributes })
    }
}

impl<'data, W: Writer> Se05XCommand<W> for ReadAttributes<'data> {
    type Response<'rdata> = ReadAttributesResponse;
}

// ************* ExportObject ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ExportObject {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
    /// Always present. Use [`RsaKeyComponent::Na`][] if not exporting an RSA key. It is the default value with the builder API
    ///
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    #[cfg_attr(feature = "builder", builder(default=RsaKeyComponent::Na))]
    pub rsa_key_component: RsaKeyComponent,
}

type ExportObjectData = (Tlv<ObjectId>, Tlv<RsaKeyComponent>);

impl ExportObject {
    fn data(&self) -> ExportObjectData {
        (
            Tlv::new(TAG_1, self.object_id),
            Tlv::new(TAG_2, self.rsa_key_component),
        )
    }

    fn command(&self) -> CommandBuilder<ExportObjectData> {
        CommandBuilder::new(NO_SM_CLA, INS_READ, P1_DEFAULT, P2_EXPORT, self.data(), 256)
    }
}

impl DataSource for ExportObject {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for ExportObject {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct ExportObjectResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub data: &'data [u8],
}

impl<'data> Se05XResponse<'data> for ExportObjectResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (data, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { data })
    }
}

impl<W: Writer> Se05XCommand<W> for ExportObject {
    type Response<'rdata> = ExportObjectResponse<'rdata>;
}

// ************* ReadType ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ReadType {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
}

type ReadTypeData = Tlv<ObjectId>;

impl ReadType {
    fn data(&self) -> ReadTypeData {
        Tlv::new(TAG_1, self.object_id)
    }

    fn command(&self) -> CommandBuilder<ReadTypeData> {
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

impl<'data> Se05XResponse<'data> for ReadTypeResponse {
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

impl<W: Writer> Se05XCommand<W> for ReadType {
    type Response<'rdata> = ReadTypeResponse;
}

// ************* ReadSize ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ReadSize {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
}

type ReadSizeData = Tlv<ObjectId>;

impl ReadSize {
    fn data(&self) -> ReadSizeData {
        Tlv::new(TAG_1, self.object_id)
    }

    fn command(&self) -> CommandBuilder<ReadSizeData> {
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

impl<'data> Se05XResponse<'data> for ReadSizeResponse {
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

impl<W: Writer> Se05XCommand<W> for ReadSize {
    type Response<'rdata> = ReadSizeResponse;
}

// ************* ReadIdList ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ReadIdList {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub offset: Be<u16>,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub filter: SecureObjectFilter,
}

type ReadIdListData = (Tlv<Be<u16>>, Tlv<SecureObjectFilter>);

impl ReadIdList {
    fn data(&self) -> ReadIdListData {
        (Tlv::new(TAG_1, self.offset), Tlv::new(TAG_2, self.filter))
    }

    fn command(&self) -> CommandBuilder<ReadIdListData> {
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

impl<'data> Se05XResponse<'data> for ReadIdListResponse<'data> {
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
                break (value, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { more, ids })
    }
}

impl<W: Writer> Se05XCommand<W> for ReadIdList {
    type Response<'rdata> = ReadIdListResponse<'rdata>;
}

// ************* CheckObjectExists ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CheckObjectExists {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
}

type CheckObjectExistsData = Tlv<ObjectId>;

impl CheckObjectExists {
    fn data(&self) -> CheckObjectExistsData {
        Tlv::new(TAG_1, self.object_id)
    }

    fn command(&self) -> CommandBuilder<CheckObjectExistsData> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
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
    pub result: Se05XResult,
}

impl<'data> Se05XResponse<'data> for CheckObjectExistsResponse {
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

impl<W: Writer> Se05XCommand<W> for CheckObjectExists {
    type Response<'rdata> = CheckObjectExistsResponse;
}

// ************* DeleteSecureObject ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct DeleteSecureObject {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
}

type DeleteSecureObjectData = Tlv<ObjectId>;

impl DeleteSecureObject {
    fn data(&self) -> DeleteSecureObjectData {
        Tlv::new(TAG_1, self.object_id)
    }

    fn command(&self) -> CommandBuilder<DeleteSecureObjectData> {
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

impl<W: Writer> Se05XCommand<W> for DeleteSecureObject {
    type Response<'rdata> = ();
}

// ************* CreateEcCurve ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CreateEcCurve {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub curve: EcCurve,
}

type CreateEcCurveData = Tlv<EcCurve>;

impl CreateEcCurve {
    fn data(&self) -> CreateEcCurveData {
        Tlv::new(TAG_1, self.curve)
    }

    fn command(&self) -> CommandBuilder<CreateEcCurveData> {
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

impl<W: Writer> Se05XCommand<W> for CreateEcCurve {
    type Response<'rdata> = ();
}

// ************* SetEcCurveParam ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct SetEcCurveParam<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub curve: EcCurve,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub param: EcCurveParam,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub value: &'data [u8],
}

type SetEcCurveParamData<'data> = (Tlv<EcCurve>, Tlv<EcCurveParam>, Tlv<&'data [u8]>);

impl<'data> SetEcCurveParam<'data> {
    fn data(&self) -> SetEcCurveParamData<'data> {
        (
            Tlv::new(TAG_1, self.curve),
            Tlv::new(TAG_2, self.param),
            Tlv::new(TAG_3, self.value),
        )
    }

    fn command(&self) -> CommandBuilder<SetEcCurveParamData<'data>> {
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

impl<'data, W: Writer> Se05XCommand<W> for SetEcCurveParam<'data> {
    type Response<'rdata> = ();
}

// ************* GetEcCurveId ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct GetEcCurveId {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub object_id: ObjectId,
}

type GetEcCurveIdData = Tlv<ObjectId>;

impl GetEcCurveId {
    fn data(&self) -> GetEcCurveIdData {
        Tlv::new(TAG_1, self.object_id)
    }

    fn command(&self) -> CommandBuilder<GetEcCurveIdData> {
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

impl<'data> Se05XResponse<'data> for GetEcCurveIdResponse {
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

impl<W: Writer> Se05XCommand<W> for GetEcCurveId {
    type Response<'rdata> = GetEcCurveIdResponse;
}

// ************* ReadEcCurveList ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ReadEcCurveList {}

type ReadEcCurveListData = ();

impl ReadEcCurveList {
    fn command(&self) -> CommandBuilder<ReadEcCurveListData> {
        CommandBuilder::new(NO_SM_CLA, INS_READ, P1_CURVE, P2_LIST, (), 0)
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

impl<'data> Se05XResponse<'data> for ReadEcCurveListResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (ids, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { ids })
    }
}

impl<W: Writer> Se05XCommand<W> for ReadEcCurveList {
    type Response<'rdata> = ReadEcCurveListResponse<'rdata>;
}

// ************* DeleteEcCurve ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct DeleteEcCurve {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub curve: EcCurve,
}

type DeleteEcCurveData = Tlv<EcCurve>;

impl DeleteEcCurve {
    fn data(&self) -> DeleteEcCurveData {
        Tlv::new(TAG_1, self.curve)
    }

    fn command(&self) -> CommandBuilder<DeleteEcCurveData> {
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

impl<W: Writer> Se05XCommand<W> for DeleteEcCurve {
    type Response<'rdata> = ();
}

// ************* CreateDigestObject ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CreateDigestObject {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub id: CryptoObjectId,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub subtype: Digest,
}

type CreateDigestObjectData = (Tlv<CryptoObjectId>, Tlv<CryptoContext>, Tlv<Digest>);

impl CreateDigestObject {
    fn data(&self) -> CreateDigestObjectData {
        (
            Tlv::new(TAG_1, self.id),
            Tlv::new(TAG_2, CryptoContext::Digest),
            Tlv::new(TAG_3, self.subtype),
        )
    }

    fn command(&self) -> CommandBuilder<CreateDigestObjectData> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_WRITE,
            P1_CRYPTO_OBJ,
            P2_DEFAULT,
            self.data(),
            0,
        )
    }
}

impl DataSource for CreateDigestObject {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for CreateDigestObject {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for CreateDigestObject {
    type Response<'rdata> = ();
}

// ************* CreateCipherObject ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CreateCipherObject {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub id: CryptoObjectId,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub subtype: CipherMode,
}

type CreateCipherObjectData = (Tlv<CryptoObjectId>, Tlv<CryptoContext>, Tlv<CipherMode>);

impl CreateCipherObject {
    fn data(&self) -> CreateCipherObjectData {
        (
            Tlv::new(TAG_1, self.id),
            Tlv::new(TAG_2, CryptoContext::Cipher),
            Tlv::new(TAG_3, self.subtype),
        )
    }

    fn command(&self) -> CommandBuilder<CreateCipherObjectData> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_WRITE,
            P1_CRYPTO_OBJ,
            P2_DEFAULT,
            self.data(),
            0,
        )
    }
}

impl DataSource for CreateCipherObject {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for CreateCipherObject {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for CreateCipherObject {
    type Response<'rdata> = ();
}

// ************* CreateSignatureObject ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CreateSignatureObject {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub id: CryptoObjectId,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub subtype: MacAlgo,
}

type CreateSignatureObjectData = (Tlv<CryptoObjectId>, Tlv<CryptoContext>, Tlv<MacAlgo>);

impl CreateSignatureObject {
    fn data(&self) -> CreateSignatureObjectData {
        (
            Tlv::new(TAG_1, self.id),
            Tlv::new(TAG_2, CryptoContext::Signature),
            Tlv::new(TAG_3, self.subtype),
        )
    }

    fn command(&self) -> CommandBuilder<CreateSignatureObjectData> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_WRITE,
            P1_CRYPTO_OBJ,
            P2_DEFAULT,
            self.data(),
            0,
        )
    }
}

impl DataSource for CreateSignatureObject {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for CreateSignatureObject {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for CreateSignatureObject {
    type Response<'rdata> = ();
}

// ************* ReadCryptoObjList ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct ReadCryptoObjList {}

type ReadCryptoObjListData = ();

impl ReadCryptoObjList {
    fn command(&self) -> CommandBuilder<ReadCryptoObjListData> {
        CommandBuilder::new(NO_SM_CLA, INS_READ, P1_CRYPTO_OBJ, P2_LIST, (), 0)
    }
}

impl DataSource for ReadCryptoObjList {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for ReadCryptoObjList {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct ReadCryptoObjListResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub list: &'data [u8],
}

impl<'data> Se05XResponse<'data> for ReadCryptoObjListResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (list, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { list })
    }
}

impl<W: Writer> Se05XCommand<W> for ReadCryptoObjList {
    type Response<'rdata> = ReadCryptoObjListResponse<'rdata>;
}

// ************* DeleteCryptoObj ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct DeleteCryptoObj {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub id: CryptoObjectId,
}

type DeleteCryptoObjData = Tlv<CryptoObjectId>;

impl DeleteCryptoObj {
    fn data(&self) -> DeleteCryptoObjData {
        Tlv::new(TAG_1, self.id)
    }

    fn command(&self) -> CommandBuilder<DeleteCryptoObjData> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_CRYPTO_OBJ,
            P2_DELETE_OBJECT,
            self.data(),
            0,
        )
    }
}

impl DataSource for DeleteCryptoObj {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for DeleteCryptoObj {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for DeleteCryptoObj {
    type Response<'rdata> = ();
}

// ************* EcdsaSign ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct EcdsaSign<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub algo: EcDsaSignatureAlgo,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub data: &'data [u8],
}

type EcdsaSignData<'data> = (Tlv<ObjectId>, Tlv<EcDsaSignatureAlgo>, Tlv<&'data [u8]>);

impl<'data> EcdsaSign<'data> {
    fn data(&self) -> EcdsaSignData<'data> {
        (
            Tlv::new(TAG_1, self.key_id),
            Tlv::new(TAG_2, self.algo),
            Tlv::new(TAG_3, self.data),
        )
    }

    fn command(&self) -> CommandBuilder<EcdsaSignData<'data>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_SIGNATURE,
            P2_SIGN,
            self.data(),
            ExpectedLen::Max,
        )
    }
}

impl<'data> DataSource for EcdsaSign<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for EcdsaSign<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct EcdsaSignResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub signature: &'data [u8],
}

impl<'data> Se05XResponse<'data> for EcdsaSignResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (signature, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { signature })
    }
}

impl<'data, W: Writer> Se05XCommand<W> for EcdsaSign<'data> {
    type Response<'rdata> = EcdsaSignResponse<'rdata>;
}

// ************* EddsaSign ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct EddsaSign<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub data: &'data [u8],
}

type EddsaSignData<'data> = (Tlv<ObjectId>, Tlv<EdDsaSignatureAlgo>, Tlv<&'data [u8]>);

impl<'data> EddsaSign<'data> {
    fn data(&self) -> EddsaSignData<'data> {
        (
            Tlv::new(TAG_1, self.key_id),
            Tlv::new(TAG_2, EdDsaSignatureAlgo::Pure),
            Tlv::new(TAG_3, self.data),
        )
    }

    fn command(&self) -> CommandBuilder<EddsaSignData<'data>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_SIGNATURE,
            P2_SIGN,
            self.data(),
            ExpectedLen::Max,
        )
    }
}

impl<'data> DataSource for EddsaSign<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for EddsaSign<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct EddsaSignResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub signature: &'data [u8],
}

impl<'data> Se05XResponse<'data> for EddsaSignResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (signature, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { signature })
    }
}

impl<'data, W: Writer> Se05XCommand<W> for EddsaSign<'data> {
    type Response<'rdata> = EddsaSignResponse<'rdata>;
}

// ************* EcdaaSign ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct EcdaaSign {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub data: [u8; 32],
    /// Serialized to TLV tag [`TAG_4`](TAG_4)
    pub random_data: [u8; 32],
}

type EcdaaSignData = (
    Tlv<ObjectId>,
    Tlv<EcDaaSignatureAlgo>,
    Tlv<[u8; 32]>,
    Tlv<[u8; 32]>,
);

impl EcdaaSign {
    fn data(&self) -> EcdaaSignData {
        (
            Tlv::new(TAG_1, self.key_id),
            Tlv::new(TAG_2, EcDaaSignatureAlgo::EcDaa),
            Tlv::new(TAG_3, self.data),
            Tlv::new(TAG_4, self.random_data),
        )
    }

    fn command(&self) -> CommandBuilder<EcdaaSignData> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_SIGNATURE,
            P2_SIGN,
            self.data(),
            ExpectedLen::Max,
        )
    }
}

impl DataSource for EcdaaSign {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for EcdaaSign {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct EcdaaSignResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub signature: &'data [u8],
}

impl<'data> Se05XResponse<'data> for EcdaaSignResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (signature, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { signature })
    }
}

impl<W: Writer> Se05XCommand<W> for EcdaaSign {
    type Response<'rdata> = EcdaaSignResponse<'rdata>;
}

// ************* EcdsaVerify ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct EcdsaVerify<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub algo: EcDsaSignatureAlgo,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub data: &'data [u8],
    /// Serialized to TLV tag [`TAG_5`](TAG_5)
    pub signature: &'data [u8],
}

type EcdsaVerifyData<'data> = (
    Tlv<ObjectId>,
    Tlv<EcDsaSignatureAlgo>,
    Tlv<&'data [u8]>,
    Tlv<&'data [u8]>,
);

impl<'data> EcdsaVerify<'data> {
    fn data(&self) -> EcdsaVerifyData<'data> {
        (
            Tlv::new(TAG_1, self.key_id),
            Tlv::new(TAG_2, self.algo),
            Tlv::new(TAG_3, self.data),
            Tlv::new(TAG_5, self.signature),
        )
    }

    fn command(&self) -> CommandBuilder<EcdsaVerifyData<'data>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_SIGNATURE,
            P2_VERIFY,
            self.data(),
            3,
        )
    }
}

impl<'data> DataSource for EcdsaVerify<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for EcdsaVerify<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct EcdsaVerifyResponse {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub result: Se05XResult,
}

impl<'data> Se05XResponse<'data> for EcdsaVerifyResponse {
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

impl<'data, W: Writer> Se05XCommand<W> for EcdsaVerify<'data> {
    type Response<'rdata> = EcdsaVerifyResponse;
}

// ************* EddsaVerify ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct EddsaVerify<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub data: &'data [u8],
    /// Serialized to TLV tag [`TAG_5`](TAG_5)
    pub signature: &'data [u8],
}

type EddsaVerifyData<'data> = (
    Tlv<ObjectId>,
    Tlv<EdDsaSignatureAlgo>,
    Tlv<&'data [u8]>,
    Tlv<&'data [u8]>,
);

impl<'data> EddsaVerify<'data> {
    fn data(&self) -> EddsaVerifyData<'data> {
        (
            Tlv::new(TAG_1, self.key_id),
            Tlv::new(TAG_2, EdDsaSignatureAlgo::Pure),
            Tlv::new(TAG_3, self.data),
            Tlv::new(TAG_5, self.signature),
        )
    }

    fn command(&self) -> CommandBuilder<EddsaVerifyData<'data>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_SIGNATURE,
            P2_VERIFY,
            self.data(),
            3,
        )
    }
}

impl<'data> DataSource for EddsaVerify<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for EddsaVerify<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct EddsaVerifyResponse {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub result: Se05XResult,
}

impl<'data> Se05XResponse<'data> for EddsaVerifyResponse {
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

impl<'data, W: Writer> Se05XCommand<W> for EddsaVerify<'data> {
    type Response<'rdata> = EddsaVerifyResponse;
}

// ************* EcdhGenerateSharedSecret ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct EcdhGenerateSharedSecret<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub public_key: &'data [u8],
}

type EcdhGenerateSharedSecretData<'data> = (Tlv<ObjectId>, Tlv<&'data [u8]>);

impl<'data> EcdhGenerateSharedSecret<'data> {
    fn data(&self) -> EcdhGenerateSharedSecretData<'data> {
        (
            Tlv::new(TAG_1, self.key_id),
            Tlv::new(TAG_2, self.public_key),
        )
    }

    fn command(&self) -> CommandBuilder<EcdhGenerateSharedSecretData<'data>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_EC,
            P2_DH,
            self.data(),
            ExpectedLen::Max,
        )
    }
}

impl<'data> DataSource for EcdhGenerateSharedSecret<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for EcdhGenerateSharedSecret<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct EcdhGenerateSharedSecretResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub shared_secret: &'data [u8],
}

impl<'data> Se05XResponse<'data> for EcdhGenerateSharedSecretResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (shared_secret, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { shared_secret })
    }
}

impl<'data, W: Writer> Se05XCommand<W> for EcdhGenerateSharedSecret<'data> {
    type Response<'rdata> = EcdhGenerateSharedSecretResponse<'rdata>;
}

// ************* RsaSign ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct RsaSign<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub algo: RsaSignatureAlgo,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub data: &'data [u8],
}

type RsaSignData<'data> = (Tlv<ObjectId>, Tlv<RsaSignatureAlgo>, Tlv<&'data [u8]>);

impl<'data> RsaSign<'data> {
    fn data(&self) -> RsaSignData<'data> {
        (
            Tlv::new(TAG_1, self.key_id),
            Tlv::new(TAG_2, self.algo),
            Tlv::new(TAG_3, self.data),
        )
    }

    fn command(&self) -> CommandBuilder<RsaSignData<'data>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_SIGNATURE,
            P2_SIGN,
            self.data(),
            ExpectedLen::Max,
        )
    }
}

impl<'data> DataSource for RsaSign<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for RsaSign<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct RsaSignResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub signature: &'data [u8],
}

impl<'data> Se05XResponse<'data> for RsaSignResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (signature, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { signature })
    }
}

impl<'data, W: Writer> Se05XCommand<W> for RsaSign<'data> {
    type Response<'rdata> = RsaSignResponse<'rdata>;
}

// ************* RsaVerify ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct RsaVerify<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub algo: RsaSignatureAlgo,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub data: &'data [u8],
    /// Serialized to TLV tag [`TAG_5`](TAG_5)
    pub signature: &'data [u8],
}

type RsaVerifyData<'data> = (
    Tlv<ObjectId>,
    Tlv<RsaSignatureAlgo>,
    Tlv<&'data [u8]>,
    Tlv<&'data [u8]>,
);

impl<'data> RsaVerify<'data> {
    fn data(&self) -> RsaVerifyData<'data> {
        (
            Tlv::new(TAG_1, self.key_id),
            Tlv::new(TAG_2, self.algo),
            Tlv::new(TAG_3, self.data),
            Tlv::new(TAG_5, self.signature),
        )
    }

    fn command(&self) -> CommandBuilder<RsaVerifyData<'data>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_SIGNATURE,
            P2_VERIFY,
            self.data(),
            3,
        )
    }
}

impl<'data> DataSource for RsaVerify<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for RsaVerify<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct RsaVerifyResponse {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub result: Se05XResult,
}

impl<'data> Se05XResponse<'data> for RsaVerifyResponse {
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

impl<'data, W: Writer> Se05XCommand<W> for RsaVerify<'data> {
    type Response<'rdata> = RsaVerifyResponse;
}

// ************* RsaEncrypt ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct RsaEncrypt<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub algo: RsaEncryptionAlgo,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub plaintext: &'data [u8],
}

type RsaEncryptData<'data> = (Tlv<ObjectId>, Tlv<RsaEncryptionAlgo>, Tlv<&'data [u8]>);

impl<'data> RsaEncrypt<'data> {
    fn data(&self) -> RsaEncryptData<'data> {
        (
            Tlv::new(TAG_1, self.key_id),
            Tlv::new(TAG_2, self.algo),
            Tlv::new(TAG_3, self.plaintext),
        )
    }

    fn command(&self) -> CommandBuilder<RsaEncryptData<'data>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_RSA,
            P2_ENCRYPT_ONESHOT,
            self.data(),
            ExpectedLen::Max,
        )
    }
}

impl<'data> DataSource for RsaEncrypt<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for RsaEncrypt<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct RsaEncryptResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub ciphertext: &'data [u8],
}

impl<'data> Se05XResponse<'data> for RsaEncryptResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (ciphertext, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { ciphertext })
    }
}

impl<'data, W: Writer> Se05XCommand<W> for RsaEncrypt<'data> {
    type Response<'rdata> = RsaEncryptResponse<'rdata>;
}

// ************* RsaDecrypt ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct RsaDecrypt<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub algo: RsaEncryptionAlgo,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub ciphertext: &'data [u8],
}

type RsaDecryptData<'data> = (Tlv<ObjectId>, Tlv<RsaEncryptionAlgo>, Tlv<&'data [u8]>);

impl<'data> RsaDecrypt<'data> {
    fn data(&self) -> RsaDecryptData<'data> {
        (
            Tlv::new(TAG_1, self.key_id),
            Tlv::new(TAG_2, self.algo),
            Tlv::new(TAG_3, self.ciphertext),
        )
    }

    fn command(&self) -> CommandBuilder<RsaDecryptData<'data>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_RSA,
            P2_DECRYPT_ONESHOT,
            self.data(),
            ExpectedLen::Max,
        )
    }
}

impl<'data> DataSource for RsaDecrypt<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for RsaDecrypt<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct RsaDecryptResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub plaintext: &'data [u8],
}

impl<'data> Se05XResponse<'data> for RsaDecryptResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (plaintext, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { plaintext })
    }
}

impl<'data, W: Writer> Se05XCommand<W> for RsaDecrypt<'data> {
    type Response<'rdata> = RsaDecryptResponse<'rdata>;
}

// ************* CipherEncryptInit ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CipherEncryptInit<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub cipher_id: CryptoObjectId,
    /// Serialized to TLV tag [`TAG_4`](TAG_4)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub initialization_vector: Option<&'data [u8]>,
}

type CipherEncryptInitData<'data> = (Tlv<ObjectId>, Tlv<CryptoObjectId>, Option<Tlv<&'data [u8]>>);

impl<'data> CipherEncryptInit<'data> {
    fn data(&self) -> CipherEncryptInitData<'data> {
        (
            Tlv::new(TAG_1, self.key_id),
            Tlv::new(TAG_2, self.cipher_id),
            self.initialization_vector.map(|data| Tlv::new(TAG_4, data)),
        )
    }

    fn command(&self) -> CommandBuilder<CipherEncryptInitData<'data>> {
        CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_CIPHER, P2_ENCRYPT, self.data(), 0)
    }
}

impl<'data> DataSource for CipherEncryptInit<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for CipherEncryptInit<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<'data, W: Writer> Se05XCommand<W> for CipherEncryptInit<'data> {
    type Response<'rdata> = ();
}

// ************* CipherDecryptInit ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CipherDecryptInit<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub cipher_id: CryptoObjectId,
    /// Serialized to TLV tag [`TAG_4`](TAG_4)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub initialization_vector: Option<&'data [u8]>,
}

type CipherDecryptInitData<'data> = (Tlv<ObjectId>, Tlv<CryptoObjectId>, Option<Tlv<&'data [u8]>>);

impl<'data> CipherDecryptInit<'data> {
    fn data(&self) -> CipherDecryptInitData<'data> {
        (
            Tlv::new(TAG_1, self.key_id),
            Tlv::new(TAG_2, self.cipher_id),
            self.initialization_vector.map(|data| Tlv::new(TAG_4, data)),
        )
    }

    fn command(&self) -> CommandBuilder<CipherDecryptInitData<'data>> {
        CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_CIPHER, P2_DECRYPT, self.data(), 0)
    }
}

impl<'data> DataSource for CipherDecryptInit<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for CipherDecryptInit<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<'data, W: Writer> Se05XCommand<W> for CipherDecryptInit<'data> {
    type Response<'rdata> = ();
}

// ************* CipherUpdate ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CipherUpdate<'data> {
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub cipher_id: CryptoObjectId,
    /// input data, can be either plaintext or ciphertext depending on whether cipher_decrypt_init or cipher_encrypt_init was used
    ///
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub data: &'data [u8],
}

type CipherUpdateData<'data> = (Tlv<CryptoObjectId>, Tlv<&'data [u8]>);

impl<'data> CipherUpdate<'data> {
    fn data(&self) -> CipherUpdateData<'data> {
        (Tlv::new(TAG_2, self.cipher_id), Tlv::new(TAG_3, self.data))
    }

    fn command(&self) -> CommandBuilder<CipherUpdateData<'data>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_CIPHER,
            P2_UPDATE,
            self.data(),
            ExpectedLen::Max,
        )
    }
}

impl<'data> DataSource for CipherUpdate<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for CipherUpdate<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct CipherUpdateResponse<'data> {
    /// output data
    ///
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub data: &'data [u8],
}

impl<'data> Se05XResponse<'data> for CipherUpdateResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (data, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { data })
    }
}

impl<'data, W: Writer> Se05XCommand<W> for CipherUpdate<'data> {
    type Response<'rdata> = CipherUpdateResponse<'rdata>;
}

// ************* CipherFinal ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CipherFinal<'data> {
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub cipher_id: CryptoObjectId,
    /// input data, can be either plaintext or ciphertext depending on whether cipher_decrypt_init or cipher_encrypt_init was used
    ///
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub data: &'data [u8],
}

type CipherFinalData<'data> = (Tlv<CryptoObjectId>, Tlv<&'data [u8]>);

impl<'data> CipherFinal<'data> {
    fn data(&self) -> CipherFinalData<'data> {
        (Tlv::new(TAG_2, self.cipher_id), Tlv::new(TAG_3, self.data))
    }

    fn command(&self) -> CommandBuilder<CipherFinalData<'data>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_CIPHER,
            P2_FINAL,
            self.data(),
            ExpectedLen::Max,
        )
    }
}

impl<'data> DataSource for CipherFinal<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for CipherFinal<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct CipherFinalResponse<'data> {
    /// output data
    ///
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub data: &'data [u8],
}

impl<'data> Se05XResponse<'data> for CipherFinalResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (data, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { data })
    }
}

impl<'data, W: Writer> Se05XCommand<W> for CipherFinal<'data> {
    type Response<'rdata> = CipherFinalResponse<'rdata>;
}

// ************* CipherOneShotEncrypt ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CipherOneShotEncrypt<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub mode: CipherMode,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub plaintext: &'data [u8],
    /// Serialized to TLV tag [`TAG_4`](TAG_4)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub initialization_vector: Option<&'data [u8]>,
}

type CipherOneShotEncryptData<'data> = (
    Tlv<ObjectId>,
    Tlv<CipherMode>,
    Tlv<&'data [u8]>,
    Option<Tlv<&'data [u8]>>,
);

impl<'data> CipherOneShotEncrypt<'data> {
    fn data(&self) -> CipherOneShotEncryptData<'data> {
        (
            Tlv::new(TAG_1, self.key_id),
            Tlv::new(TAG_2, self.mode),
            Tlv::new(TAG_3, self.plaintext),
            self.initialization_vector.map(|data| Tlv::new(TAG_4, data)),
        )
    }

    fn command(&self) -> CommandBuilder<CipherOneShotEncryptData<'data>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_CIPHER,
            P2_ENCRYPT_ONESHOT,
            self.data(),
            ExpectedLen::Max,
        )
    }
}

impl<'data> DataSource for CipherOneShotEncrypt<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for CipherOneShotEncrypt<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct CipherOneShotEncryptResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub ciphertext: &'data [u8],
}

impl<'data> Se05XResponse<'data> for CipherOneShotEncryptResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (ciphertext, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { ciphertext })
    }
}

impl<'data, W: Writer> Se05XCommand<W> for CipherOneShotEncrypt<'data> {
    type Response<'rdata> = CipherOneShotEncryptResponse<'rdata>;
}

// ************* CipherOneShotDecrypt ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct CipherOneShotDecrypt<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub mode: CipherMode,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub ciphertext: &'data [u8],
    /// Serialized to TLV tag [`TAG_4`](TAG_4)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub initialization_vector: Option<&'data [u8]>,
}

type CipherOneShotDecryptData<'data> = (
    Tlv<ObjectId>,
    Tlv<CipherMode>,
    Tlv<&'data [u8]>,
    Option<Tlv<&'data [u8]>>,
);

impl<'data> CipherOneShotDecrypt<'data> {
    fn data(&self) -> CipherOneShotDecryptData<'data> {
        (
            Tlv::new(TAG_1, self.key_id),
            Tlv::new(TAG_2, self.mode),
            Tlv::new(TAG_3, self.ciphertext),
            self.initialization_vector.map(|data| Tlv::new(TAG_4, data)),
        )
    }

    fn command(&self) -> CommandBuilder<CipherOneShotDecryptData<'data>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_CIPHER,
            P2_DECRYPT_ONESHOT,
            self.data(),
            ExpectedLen::Max,
        )
    }
}

impl<'data> DataSource for CipherOneShotDecrypt<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for CipherOneShotDecrypt<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct CipherOneShotDecryptResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub plaintext: &'data [u8],
}

impl<'data> Se05XResponse<'data> for CipherOneShotDecryptResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (plaintext, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { plaintext })
    }
}

impl<'data, W: Writer> Se05XCommand<W> for CipherOneShotDecrypt<'data> {
    type Response<'rdata> = CipherOneShotDecryptResponse<'rdata>;
}

// ************* MacGenerateInit ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct MacGenerateInit {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub mac_id: CryptoObjectId,
}

type MacGenerateInitData = (Tlv<ObjectId>, Tlv<CryptoObjectId>);

impl MacGenerateInit {
    fn data(&self) -> MacGenerateInitData {
        (Tlv::new(TAG_1, self.key_id), Tlv::new(TAG_2, self.mac_id))
    }

    fn command(&self) -> CommandBuilder<MacGenerateInitData> {
        CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_MAC, P2_GENERATE, self.data(), 0)
    }
}

impl DataSource for MacGenerateInit {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for MacGenerateInit {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for MacGenerateInit {
    type Response<'rdata> = ();
}

// ************* MacValidateInit ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct MacValidateInit {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub mac_id: CryptoObjectId,
}

type MacValidateInitData = (Tlv<ObjectId>, Tlv<CryptoObjectId>);

impl MacValidateInit {
    fn data(&self) -> MacValidateInitData {
        (Tlv::new(TAG_1, self.key_id), Tlv::new(TAG_2, self.mac_id))
    }

    fn command(&self) -> CommandBuilder<MacValidateInitData> {
        CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_MAC, P2_VALIDATE, self.data(), 0)
    }
}

impl DataSource for MacValidateInit {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for MacValidateInit {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for MacValidateInit {
    type Response<'rdata> = ();
}

// ************* MacUpdate ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct MacUpdate<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub data: &'data [u8],
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub mac_id: CryptoObjectId,
}

type MacUpdateData<'data> = (Tlv<&'data [u8]>, Tlv<CryptoObjectId>);

impl<'data> MacUpdate<'data> {
    fn data(&self) -> MacUpdateData<'data> {
        (Tlv::new(TAG_1, self.data), Tlv::new(TAG_2, self.mac_id))
    }

    fn command(&self) -> CommandBuilder<MacUpdateData<'data>> {
        CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_MAC, P2_UPDATE, self.data(), 0)
    }
}

impl<'data> DataSource for MacUpdate<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for MacUpdate<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<'data, W: Writer> Se05XCommand<W> for MacUpdate<'data> {
    type Response<'rdata> = ();
}

// ************* MacGenerateFinal ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct MacGenerateFinal<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub data: &'data [u8],
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub mac_id: CryptoObjectId,
}

type MacGenerateFinalData<'data> = (Tlv<&'data [u8]>, Tlv<CryptoObjectId>);

impl<'data> MacGenerateFinal<'data> {
    fn data(&self) -> MacGenerateFinalData<'data> {
        (Tlv::new(TAG_1, self.data), Tlv::new(TAG_2, self.mac_id))
    }

    fn command(&self) -> CommandBuilder<MacGenerateFinalData<'data>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_MAC,
            P2_FINAL,
            self.data(),
            ExpectedLen::Max,
        )
    }
}

impl<'data> DataSource for MacGenerateFinal<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for MacGenerateFinal<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct MacGenerateFinalResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub tag: &'data [u8],
}

impl<'data> Se05XResponse<'data> for MacGenerateFinalResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (tag, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { tag })
    }
}

impl<'data, W: Writer> Se05XCommand<W> for MacGenerateFinal<'data> {
    type Response<'rdata> = MacGenerateFinalResponse<'rdata>;
}

// ************* MacValidateFinal ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct MacValidateFinal<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub data: &'data [u8],
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub mac_id: CryptoObjectId,
    /// Tag to validate
    ///
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub tag: &'data [u8],
}

type MacValidateFinalData<'data> = (Tlv<&'data [u8]>, Tlv<CryptoObjectId>, Tlv<&'data [u8]>);

impl<'data> MacValidateFinal<'data> {
    fn data(&self) -> MacValidateFinalData<'data> {
        (
            Tlv::new(TAG_1, self.data),
            Tlv::new(TAG_2, self.mac_id),
            Tlv::new(TAG_3, self.tag),
        )
    }

    fn command(&self) -> CommandBuilder<MacValidateFinalData<'data>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_MAC,
            P2_FINAL,
            self.data(),
            ExpectedLen::Max,
        )
    }
}

impl<'data> DataSource for MacValidateFinal<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for MacValidateFinal<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct MacValidateFinalResponse {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub result: Se05XResult,
}

impl<'data> Se05XResponse<'data> for MacValidateFinalResponse {
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

impl<'data, W: Writer> Se05XCommand<W> for MacValidateFinal<'data> {
    type Response<'rdata> = MacValidateFinalResponse;
}

// ************* MacOneShotGenerate ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct MacOneShotGenerate<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub algo: MacAlgo,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub data: &'data [u8],
}

type MacOneShotGenerateData<'data> = (Tlv<ObjectId>, Tlv<MacAlgo>, Tlv<&'data [u8]>);

impl<'data> MacOneShotGenerate<'data> {
    fn data(&self) -> MacOneShotGenerateData<'data> {
        (
            Tlv::new(TAG_1, self.key_id),
            Tlv::new(TAG_2, self.algo),
            Tlv::new(TAG_3, self.data),
        )
    }

    fn command(&self) -> CommandBuilder<MacOneShotGenerateData<'data>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_MAC,
            P2_GENERATE_ONESHOT,
            self.data(),
            ExpectedLen::Max,
        )
    }
}

impl<'data> DataSource for MacOneShotGenerate<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for MacOneShotGenerate<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct MacOneShotGenerateResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub tag: &'data [u8],
}

impl<'data> Se05XResponse<'data> for MacOneShotGenerateResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (tag, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { tag })
    }
}

impl<'data, W: Writer> Se05XCommand<W> for MacOneShotGenerate<'data> {
    type Response<'rdata> = MacOneShotGenerateResponse<'rdata>;
}

// ************* MacOneShotValidate ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct MacOneShotValidate<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub key_id: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub algo: MacAlgo,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub data: &'data [u8],
    /// tag to validate
    ///
    /// Serialized to TLV tag [`TAG_5`](TAG_5)
    pub tag: &'data [u8],
}

type MacOneShotValidateData<'data> = (
    Tlv<ObjectId>,
    Tlv<MacAlgo>,
    Tlv<&'data [u8]>,
    Tlv<&'data [u8]>,
);

impl<'data> MacOneShotValidate<'data> {
    fn data(&self) -> MacOneShotValidateData<'data> {
        (
            Tlv::new(TAG_1, self.key_id),
            Tlv::new(TAG_2, self.algo),
            Tlv::new(TAG_3, self.data),
            Tlv::new(TAG_5, self.tag),
        )
    }

    fn command(&self) -> CommandBuilder<MacOneShotValidateData<'data>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_MAC,
            P2_VALIDATE_ONESHOT,
            self.data(),
            ExpectedLen::Max,
        )
    }
}

impl<'data> DataSource for MacOneShotValidate<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for MacOneShotValidate<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct MacOneShotValidateResponse {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub result: Se05XResult,
}

impl<'data> Se05XResponse<'data> for MacOneShotValidateResponse {
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

impl<'data, W: Writer> Se05XCommand<W> for MacOneShotValidate<'data> {
    type Response<'rdata> = MacOneShotValidateResponse;
}

// ************* Hkdf ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct Hkdf<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub ikm: ObjectId,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub digest: Digest,
    /// up to 64 bytes
    ///
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub salt: Option<&'data [u8]>,
    /// Serialized to TLV tag [`TAG_4`](TAG_4)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub info: Option<&'data [u8]>,
    /// Up to MAX_APDU_PAYLOAD_LENGTH (= 889)
    ///
    /// Serialized to TLV tag [`TAG_5`](TAG_5)
    pub requested_len: Be<u16>,
}

type HkdfData<'data> = (
    Tlv<ObjectId>,
    Tlv<Digest>,
    Option<Tlv<&'data [u8]>>,
    Option<Tlv<&'data [u8]>>,
    Tlv<Be<u16>>,
);

impl<'data> Hkdf<'data> {
    fn data(&self) -> HkdfData<'data> {
        (
            Tlv::new(TAG_1, self.ikm),
            Tlv::new(TAG_2, self.digest),
            self.salt.map(|data| Tlv::new(TAG_3, data)),
            self.info.map(|data| Tlv::new(TAG_4, data)),
            Tlv::new(TAG_5, self.requested_len),
        )
    }

    fn command(&self) -> CommandBuilder<HkdfData<'data>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_DEFAULT,
            P2_HKDF,
            self.data(),
            ExpectedLen::Max,
        )
    }
}

impl<'data> DataSource for Hkdf<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for Hkdf<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct HkdfResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub data: &'data [u8],
}

impl<'data> Se05XResponse<'data> for HkdfResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (data, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { data })
    }
}

impl<'data, W: Writer> Se05XCommand<W> for Hkdf<'data> {
    type Response<'rdata> = HkdfResponse<'rdata>;
}

// ************* Pbkdf2 ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct Pbkdf2<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub password: ObjectId,
    /// up to 64 bytes
    ///
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub salt: Option<&'data [u8]>,
    /// Up to 0x7FFF
    ///
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub iterations: Be<u16>,
    /// Up to 512
    ///
    /// Serialized to TLV tag [`TAG_4`](TAG_4)
    pub requested_len: Be<u16>,
}

type Pbkdf2Data<'data> = (
    Tlv<ObjectId>,
    Option<Tlv<&'data [u8]>>,
    Tlv<Be<u16>>,
    Tlv<Be<u16>>,
);

impl<'data> Pbkdf2<'data> {
    fn data(&self) -> Pbkdf2Data<'data> {
        (
            Tlv::new(TAG_1, self.password),
            self.salt.map(|data| Tlv::new(TAG_2, data)),
            Tlv::new(TAG_3, self.iterations),
            Tlv::new(TAG_4, self.requested_len),
        )
    }

    fn command(&self) -> CommandBuilder<Pbkdf2Data<'data>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_DEFAULT,
            P2_PBKDF,
            self.data(),
            ExpectedLen::Max,
        )
    }
}

impl<'data> DataSource for Pbkdf2<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for Pbkdf2<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct Pbkdf2Response<'data> {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub data: &'data [u8],
}

impl<'data> Se05XResponse<'data> for Pbkdf2Response<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (data, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { data })
    }
}

impl<'data, W: Writer> Se05XCommand<W> for Pbkdf2<'data> {
    type Response<'rdata> = Pbkdf2Response<'rdata>;
}

// ************* DigestInit ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct DigestInit {
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub digest_id: CryptoObjectId,
}

type DigestInitData = Tlv<CryptoObjectId>;

impl DigestInit {
    fn data(&self) -> DigestInitData {
        Tlv::new(TAG_2, self.digest_id)
    }

    fn command(&self) -> CommandBuilder<DigestInitData> {
        CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_DEFAULT, P2_INIT, self.data(), 0)
    }
}

impl DataSource for DigestInit {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for DigestInit {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<W: Writer> Se05XCommand<W> for DigestInit {
    type Response<'rdata> = ();
}

// ************* DigestUpdate ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct DigestUpdate<'data> {
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub digest_id: CryptoObjectId,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub data: &'data [u8],
}

type DigestUpdateData<'data> = (Tlv<CryptoObjectId>, Tlv<&'data [u8]>);

impl<'data> DigestUpdate<'data> {
    fn data(&self) -> DigestUpdateData<'data> {
        (Tlv::new(TAG_2, self.digest_id), Tlv::new(TAG_3, self.data))
    }

    fn command(&self) -> CommandBuilder<DigestUpdateData<'data>> {
        CommandBuilder::new(NO_SM_CLA, INS_CRYPTO, P1_DEFAULT, P2_UPDATE, self.data(), 0)
    }
}

impl<'data> DataSource for DigestUpdate<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for DigestUpdate<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<'data, W: Writer> Se05XCommand<W> for DigestUpdate<'data> {
    type Response<'rdata> = ();
}

// ************* DigestFinal ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct DigestFinal<'data> {
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub digest_id: CryptoObjectId,
    /// Serialized to TLV tag [`TAG_3`](TAG_3)
    pub data: &'data [u8],
}

type DigestFinalData<'data> = (Tlv<CryptoObjectId>, Tlv<&'data [u8]>);

impl<'data> DigestFinal<'data> {
    fn data(&self) -> DigestFinalData<'data> {
        (Tlv::new(TAG_2, self.digest_id), Tlv::new(TAG_3, self.data))
    }

    fn command(&self) -> CommandBuilder<DigestFinalData<'data>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_DEFAULT,
            P2_FINAL,
            self.data(),
            ExpectedLen::Max,
        )
    }
}

impl<'data> DataSource for DigestFinal<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for DigestFinal<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct DigestFinalResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub digest: &'data [u8],
}

impl<'data> Se05XResponse<'data> for DigestFinalResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (digest, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { digest })
    }
}

impl<'data, W: Writer> Se05XCommand<W> for DigestFinal<'data> {
    type Response<'rdata> = DigestFinalResponse<'rdata>;
}

// ************* DigestOneShot ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct DigestOneShot<'data> {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub algo: Digest,
    /// Serialized to TLV tag [`TAG_2`](TAG_2)
    pub data: &'data [u8],
}

type DigestOneShotData<'data> = (Tlv<Digest>, Tlv<&'data [u8]>);

impl<'data> DigestOneShot<'data> {
    fn data(&self) -> DigestOneShotData<'data> {
        (Tlv::new(TAG_1, self.algo), Tlv::new(TAG_2, self.data))
    }

    fn command(&self) -> CommandBuilder<DigestOneShotData<'data>> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_CRYPTO,
            P1_DEFAULT,
            P2_ONESHOT,
            self.data(),
            ExpectedLen::Max,
        )
    }
}

impl<'data> DataSource for DigestOneShot<'data> {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<'data, W: Writer> DataStream<W> for DigestOneShot<'data> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct DigestOneShotResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub digest: &'data [u8],
}

impl<'data> Se05XResponse<'data> for DigestOneShotResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (digest, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { digest })
    }
}

impl<'data, W: Writer> Se05XCommand<W> for DigestOneShot<'data> {
    type Response<'rdata> = DigestOneShotResponse<'rdata>;
}

// ************* GetVersion ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct GetVersion {}

type GetVersionData = ();

impl GetVersion {
    fn command(&self) -> CommandBuilder<GetVersionData> {
        CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_DEFAULT, P2_VERSION, (), 11)
    }
}

impl DataSource for GetVersion {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for GetVersion {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct GetVersionResponse {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub version_info: VersionInfo,
}

impl<'data> Se05XResponse<'data> for GetVersionResponse {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (version_info, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value.try_into()?, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { version_info })
    }
}

impl<W: Writer> Se05XCommand<W> for GetVersion {
    type Response<'rdata> = GetVersionResponse;
}

// ************* GetTimestamp ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct GetTimestamp {}

type GetTimestampData = ();

impl GetTimestamp {
    fn command(&self) -> CommandBuilder<GetTimestampData> {
        CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_DEFAULT, P2_TIME, (), 20)
    }
}

impl DataSource for GetTimestamp {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for GetTimestamp {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct GetTimestampResponse<'data> {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub timestamp: &'data [u8; 12],
}

impl<'data> Se05XResponse<'data> for GetTimestampResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (timestamp, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value.try_into()?, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { timestamp })
    }
}

impl<W: Writer> Se05XCommand<W> for GetTimestamp {
    type Response<'rdata> = GetTimestampResponse<'rdata>;
}

// ************* GetFreeMemory ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct GetFreeMemory {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub memory: Memory,
}

type GetFreeMemoryData = Tlv<Memory>;

impl GetFreeMemory {
    fn data(&self) -> GetFreeMemoryData {
        Tlv::new(TAG_1, self.memory)
    }

    fn command(&self) -> CommandBuilder<GetFreeMemoryData> {
        CommandBuilder::new(NO_SM_CLA, INS_MGMT, P1_DEFAULT, P2_MEMORY, self.data(), 6)
    }
}

impl DataSource for GetFreeMemory {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        self.command().is_empty()
    }
}
impl<W: Writer> DataStream<W> for GetFreeMemory {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as iso7816::command::Writer>::Error> {
        self.command().to_writer(writer)
    }
}
#[derive(Clone, Debug)]
pub struct GetFreeMemoryResponse {
    /// Parsed from TLV tag [`TAG_1`](TAG_1)
    pub available: Be<u16>,
}

impl<'data> Se05XResponse<'data> for GetFreeMemoryResponse {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (available, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value.try_into()?, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { available })
    }
}

impl<W: Writer> Se05XCommand<W> for GetFreeMemory {
    type Response<'rdata> = GetFreeMemoryResponse;
}

// ************* GetRandom ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct GetRandom {
    /// Serialized to TLV tag [`TAG_1`](TAG_1)
    pub length: Be<u16>,
}

type GetRandomData = Tlv<Be<u16>>;

impl GetRandom {
    fn data(&self) -> GetRandomData {
        Tlv::new(TAG_1, self.length)
    }

    fn command(&self) -> CommandBuilder<GetRandomData> {
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

impl<'data> Se05XResponse<'data> for GetRandomResponse<'data> {
    fn from_response(rem: &'data [u8]) -> Result<Self, Error> {
        let (data, rem) = loop {
            let mut rem_inner = rem;
            let (tag, value, r) = take_do(rem_inner).ok_or(Error::Tlv)?;
            rem_inner = r;
            if tag == TAG_1 {
                break (value, rem_inner);
            }
        };
        let _ = rem;
        Ok(Self { data })
    }
}

impl<W: Writer> Se05XCommand<W> for GetRandom {
    type Response<'rdata> = GetRandomResponse<'rdata>;
}

// ************* DeleteAll ************* //

#[derive(Clone, Debug)]
#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
pub struct DeleteAll {}

type DeleteAllData = ();

impl DeleteAll {
    fn command(&self) -> CommandBuilder<DeleteAllData> {
        CommandBuilder::new(
            NO_SM_CLA,
            INS_MGMT,
            P1_DEFAULT,
            P2_DELETE_ALL,
            (),
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

impl<W: Writer> Se05XCommand<W> for DeleteAll {
    type Response<'rdata> = ();
}
