// Copyright (C) 2023 Nitrokey GmbH
// SPDX-License-Identifier: LGPL-3.0-only
#![cfg_attr(not(test), no_std)]

//!
//! SE05X driver
//! ===========
//!
//! This crate contains a Rust driver for the SE05x series of secure elements from NXP.
//! It contains an implementation of the T=1 protocol and the ISO7816-4 APDUs that are used to communicate with the se05x.
//!
//! ```rust,no_run
//! # include!("doc_utils.rs");
//! # #[cfg(feature = "builder")]
//! # fn main() -> Result<(), se05x::se05x::Error>{
//! use se05x::se05x::commands::*;
//! use se05x::se05x::policies::*;
//! use se05x::se05x::*;
//! let i2c = get_i2c();
//! let delay = get_delay();
//! let address = 0x48;
//! let mut se05x = Se05X::new(i2c, address, delay);
//! let user_id = ObjectId([0x01, 0x00, 0x00, 0x00]);
//! let object_id = ObjectId([0x01, 0x02, 0x03, 0x04]);
//! let buf = &mut [0; 128];
//!
//! let atr = se05x.enable();
//!
//! // Running a WriteUserId command:
//! se05x.run_command(
//!     &WriteUserId::builder()
//!         .object_id(user_id)
//!         .data(b"Some value")
//!         .build(),
//!     buf,
//! )?;
//!
//! // Creating a file with a policy
//! let policy = &[Policy {
//!     object_id: user_id,
//!     access_rule: ObjectAccessRule::from_flags(ObjectPolicyFlags::ALLOW_READ),
//! }];
//!
//! se05x.run_command(
//!     &WriteBinary::builder()
//!         .policy(PolicySet(policy))
//!         .object_id(object_id)
//!         .file_length(9.into())
//!         .data(b"Some data")
//!         .build(),
//!     buf,
//! )?;
//!
//! // Opening a session with teh UserID
//! let session_id = se05x
//!     .run_command(&CreateSession { object_id: user_id }, buf)?
//!     .session_id;
//!
//! // Verifying the UserId
//! se05x.run_session_command(
//!     session_id,
//!     &VerifySessionUserId {
//!         user_id: b"Some value",
//!     },
//!     buf,
//! )?;
//! // Reading the data with the verified session
//! let data = se05x.run_session_command(
//!     session_id,
//!     &ReadObject::builder()
//!         .object_id(object_id)
//!         .offset(0.into())
//!         .length(9.into())
//!         .build(),
//!     buf,
//! )?;
//! # Ok(())
//! # }
//! # #[cfg(not(feature = "builder"))]
//! # fn main() {}
//! ```
//!
//! Architecture
//! ------------
//!
//! ### T=1
//!
//! This driver communicates with the se05x over the T=1 protocol over I2C, as described in [UM11225](https://www.nxp.com/webapp/Download?colCode=UM11225).
//!
//! To do so and be compatible with most embedded controlers, it depends on the I2C [Read](https://docs.rs/embedded-hal/latest/embedded_hal/blocking/i2c/trait.Read.html) and [Write](https://docs.rs/embedded-hal/latest/embedded_hal/blocking/i2c/trait.Write.html) from [embedded-hal](https://docs.rs/embedded-hal/latest/embedded_hal).
//! However these traits do not expose the enough, as the T=1 protocol requires detecting I2C NACKs, which are not exposed in this protocol.
//!
//! Nacks are exposed in the `Error` types for each `HAL` crate. As such an extension to the embedded-hal traits is defined as `I2CErrorNack`, exposing the missing information.
//! It is implemented for the NRF and LPC55 Hals in `src/t1/i2cimpl.rs`, gated by the features `nrf` and `lpc55` respectively.
//!
//! This may not be necessary with future releases of `embedded-hal`, which [adds the missing information](https://docs.rs/embedded-hal/1.0.0-alpha.11/embedded_hal/i2c/enum.ErrorKind.html).
//!
//!
//! ### Iso7816
//!
//! This driver uses the [`iso7816`](https://docs.rs/iso7816/latest/iso7816/) crate to implement serialization of APDUs.
//!
//! ### Generation of commands
//!
//! To simplify implementation, all supported se05x APDUs are described in `src/se05x/commands.toml`.
//! The python script `generate_commands.py` parses the `command.toml` file and generates `src/se05x/commands.rs`, which implements all the APDUs.
//!
//! Funding
//! -------
//!
//! [<img src="https://nlnet.nl/logo/banner.svg" width="200" alt="Logo NLnet: abstract logo of four people seen from above" hspace="20">](https://nlnet.nl/)
//! [<img src="https://nlnet.nl/image/logos/NGIAssure_tag.svg" width="200" alt="Logo NGI Assure: letterlogo shaped like a tag" hspace="20">](https://nlnet.nl/assure/)
//!
//! This project was funded through the [NGI Assure](https://nlnet.nl/assure/) Fund, a fund established by [NLnet](https://nlnet.nl/) with financial support from the European Commission's [Next Generation Internet programme](https://ngi.eu/), under the aegis of DG Communications Networks, Content and Technology under grant agreement No 957073.
extern crate delog;
delog::generate_macros!();

mod macros;
pub mod embedded_hal;

pub mod se05x;
pub mod t1;
