// Copyright (C) 2023 Nitrokey GmbH
// SPDX-License-Identifier: LGPL-3.0-only

use bitflags::bitflags;
use iso7816::command::{DataSource, DataStream, Writer};

use crate::se05x::ObjectId;

bitflags! {
    #[derive(Clone, Copy,PartialEq,Eq, Debug)]
    pub struct ObjectPolicyFlags: u32 {
        /// Reserved for future use
        const RFU1                            = 0b10000000_00000000_00000000_00000000;
        /// Reserved for future use
        const RFU2                            = 0b01000000_00000000_00000000_00000000;
        /// Explicitely forbid all  operations
        const FORBID_ALL                      = 0b00100000_00000000_00000000_00000000;
        /// Allow signature or MAC  generation
        const ALLOW_SIGN                      = 0b00010000_00000000_00000000_00000000;
        /// Allow signature or MAC  verification
        const ALLOW_VERIFY                    = 0b00001000_00000000_00000000_00000000;
        /// Allow key agreement
        const ALLOW_KA                        = 0b00000100_00000000_00000000_00000000;
        /// Allow encryption
        const ALLOW_ENC                       = 0b00000010_00000000_00000000_00000000;
        /// Allow decryption
        const ALLOW_DEC                       = 0b00000001_00000000_00000000_00000000;
        /// Allow key derivation
        const ALLOW_KDF                       = 0b00000000_10000000_00000000_00000000;
        /// Allow key wrapping (master key)
        const ALLOW_WRAP                      = 0b00000000_01000000_00000000_00000000;
        /// Allow to read the object
        const ALLOW_READ                      = 0b00000000_00100000_00000000_00000000;
        /// Allow to write the object
        const ALLOW_WRITE                     = 0b00000000_00010000_00000000_00000000;
        /// Allow to (re)generate the object (only internally)
        const ALLOW_GEN                       = 0b00000000_00001000_00000000_00000000;
        /// Allow to delete the object
        const ALLOW_DELETE                    = 0b00000000_00000100_00000000_00000000;
        /// Require SCP03 or ECKey session secure messaging where secure messaging requires C_MAC and C_ DECRYPTION set.
        const REQUIRE_SM                      = 0b00000000_00000010_00000000_00000000;
        /// Indicates that access to the object is allowed only if the given PCR object contains a certain value
        #[doc(hidden)]
        const REQUIRE_PCR_VALUE               = 0b00000000_00000001_00000000_00000000;
        /// Indicates that this object may be used to create attestation statements (i.e. perform attestation of other objects)
        const ALLOW_ATTESTATION               = 0b00000000_00000000_10000000_00000000;
        /// Indicates that this object may be used to perform DESFire authentication
        const ALLOW_DESFIRE_AUTHENTICATION    = 0b00000000_00000000_01000000_00000000;
        /// Indicates that the DESFire session keys may be dumped to host
        const ALLOW_DESFIRE_DUMP_SESSION_KEYS = 0b00000000_00000000_00100000_00000000;
        /// Indicates that this object can be imported or exported
        const ALLOW_IMPORT_EXPORT             = 0b00000000_00000000_00010000_00000000;

    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PcrExtension {
    object_id: ObjectId,
    pcr_value: [u8; 32],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ObjectAccessRule {
    flags: ObjectPolicyFlags,
    require_pcr_value: Option<PcrExtension>,
}

impl ObjectAccessRule {
    pub const fn from_flags(flags: ObjectPolicyFlags) -> Self {
        assert!(!flags.contains(ObjectPolicyFlags::REQUIRE_PCR_VALUE));
        Self {
            flags,
            require_pcr_value: None,
        }
    }

    /// Indicates that access to the object is allowed only if the given PCR object contains a certain value
    pub fn require_pcr_value(self, extension: Option<PcrExtension>) -> Self {
        Self {
            flags: if extension.is_some() {
                self.flags | ObjectPolicyFlags::REQUIRE_PCR_VALUE
            } else {
                self.flags.difference(ObjectPolicyFlags::REQUIRE_PCR_VALUE)
            },
            require_pcr_value: extension,
        }
    }

    pub fn to_bytes(self) -> heapless::Vec<u8, 40> {
        if let Some(require_pcr_value) = self.require_pcr_value {
            self.flags
                .bits()
                .to_be_bytes()
                .into_iter()
                .chain(require_pcr_value.object_id.0)
                .chain(require_pcr_value.pcr_value)
                .collect()
        } else {
            self.flags.bits().to_be_bytes().into_iter().collect()
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Policy {
    pub object_id: ObjectId,
    pub access_rule: ObjectAccessRule,
}

impl Policy {
    pub fn to_bytes(self) -> heapless::Vec<u8, 41> {
        let ar = self.access_rule.to_bytes();
        self.object_id.0.into_iter().chain(ar).collect()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PolicySet<'a>(pub &'a [Policy]);

impl<'a> PolicySet<'a> {
    pub fn to_bytes(self, buffer: &mut [u8]) -> Option<&[u8]> {
        let mut offset = 0;
        for i in self.0 {
            let bytes = i.to_bytes();
            if buffer.len() - offset < bytes.len() + 1 {
                return None;
            }
            buffer[offset] = bytes.len() as u8;
            offset += 1;

            buffer[offset..][..bytes.len()].copy_from_slice(&bytes);
            offset += bytes.len();
        }
        Some(&buffer[..offset])
    }
}

impl<'a> DataSource for PolicySet<'a> {
    fn len(&self) -> usize {
        self.0.iter().map(|p| p.to_bytes().len() + 1).sum()
    }
}

impl<'a, W: Writer> DataStream<W> for PolicySet<'a> {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as Writer>::Error> {
        for p in self.0 {
            let b = p.to_bytes();
            writer.write_all(&[b.len() as u8])?;
            writer.write_all(&p.to_bytes())?;
        }
        Ok(())
    }
}

bitflags! {
    #[derive(Clone, Copy,PartialEq,Eq, Debug)]
    pub struct SessionPolicyFlags: u16 {
        ///  Defines the maximum number of APDUs allowed within the session. Note that the ExchangeSessionData command itself is also counted as APDU within the session.
        #[doc(hidden)]
        const MAX_APDU      = 0b10000000_00000000;
        /// Reserved for future use
        const RFU1          = 0b01000000_00000000;
        /// Defines whether this session can be refreshed without losing context.
        const ALLOW_REFRESH = 0b00100000_00000000;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SessionPolicy {
    flags: SessionPolicyFlags,
    max_apdu: Option<u16>,
}

impl SessionPolicy {
    pub const fn from_flags(flags: SessionPolicyFlags) -> Self {
        assert!(!flags.contains(SessionPolicyFlags::MAX_APDU));
        Self {
            flags,
            max_apdu: None,
        }
    }

    pub fn max_apdu(self, max_apdu: Option<u16>) -> Self {
        Self {
            flags: if max_apdu.is_some() {
                self.flags | SessionPolicyFlags::MAX_APDU
            } else {
                self.flags.difference(SessionPolicyFlags::MAX_APDU)
            },
            max_apdu,
        }
    }

    pub fn to_bytes(self) -> heapless::Vec<u8, 7> {
        if let Some(max_apdu) = self.max_apdu {
            self.flags
                .bits()
                .to_be_bytes()
                .into_iter()
                .chain(max_apdu.to_be_bytes())
                .collect()
        } else {
            self.flags.bits().to_be_bytes().into_iter().collect()
        }
    }
}

impl DataSource for SessionPolicy {
    fn len(&self) -> usize {
        self.to_bytes().len()
    }
    fn is_empty(&self) -> bool {
        false
    }
}

impl<W: Writer> DataStream<W> for SessionPolicy {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as Writer>::Error> {
        let bytes = self.to_bytes();
        writer.write_all(&bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy() {
        let policies = &[Policy {
            object_id: ObjectId::INVALID,
            access_rule: ObjectAccessRule::from_flags(ObjectPolicyFlags::ALLOW_DELETE),
        }];
        let policy = PolicySet(policies);

        let mut buf = [0; 100];
        let res = policy.to_bytes(&mut buf).unwrap();
        assert_eq!(res, hex_literal::hex!("08 00000000 00040000"));
    }
}
