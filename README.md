<!--
Copyright (C) 2023 Nitrokey GmbH
SPDX-License-Identifier: CC0-1.0
-->

SE05X driver
===========

This crate contains a Rust driver for the SE05x series of secure elements from NXP.
It contains an implementation of the T=1 protocol and the ISO7816-4 APDUs that are used to communicate with the SE050.

This crate is under heavy development.

```rust,ignore
let i2c: impl I2CForT1 = todo!();
let delay: impl DelayUs<u32> = todo!();
let mut se050 = Se050::new(i2c, address, delay);
let user_id = ObjectId(hex!("01020304"));

let atr = se050.enable();

// Running a WriteUserId command:
se050.run_command(&WriteUserId {
    policy: None,
    max_attempts: None,
    object_id: user_id,
    value: b"Some value"
})?;

// Creating a file with a policy
let policy = &[Policy {
    object_id: user_id,
    access_rule: ObjectAccessRule::from_flags(
        ObjectPolicyFlags::ALLOW_READ,
    ),
}];

se050.run_command(
    &WriteBinary {
        transient: false,
        policy: Some(PolicySet(policy)),
        object_id,
        offset: None,
        file_length: Some(9.into()),
        data: Some(&b"Some data"),
    },
    &mut buf,
)?;

// Opening a session with teh UserID
let session = se050.run_command(&CreateSession { object_id: user_id }, &mut buf)?;

// Verifying the UserId
se050.run_command(
    &ProcessSessionCmd {
        session_id: session.session_id,
        apdu: VerifySessionUserId {
            user_id: b"Some value",
        },
    },
    &mut buf,
)?;
// Reading the data with the verified session
let data = se050.run_command(
    &ProcessSessionCmd {
        session_id: session.session_id,
        apdu: ReadObject {
            object_id,
            offset: Some(0.into()),
            length: Some(9.into()),
            rsa_key_component: None,
        },
    },
    &mut buf,
)?;
```

Architecture
------------

### T=1

This driver communicates with the SE050 over the T=1 protocol over I2C, as described in [UM11225](https://www.nxp.com/webapp/Download?colCode=UM11225).

To do so and be compatible with most embedded controlers, it depends on the I2C [Read](https://docs.rs/embedded-hal/latest/embedded_hal/blocking/i2c/trait.Read.html) and [Write](https://docs.rs/embedded-hal/latest/embedded_hal/blocking/i2c/trait.Write.html) from [embedded-hal](https://docs.rs/embedded-hal/latest/embedded_hal).
However these traits do not expose the enough, as the T=1 protocol requires detecting I2C NACKs, which are not exposed in this protocol.

Nacks are exposed in the `Error` types for each `HAL` crate. As such an extension to the embedded-hal traits is defined as `I2CErrorNack`, exposing the missing information.
It is implemented for the NRF and LPC55 Hals in `src/t1/i2cimpl.rs`, gated by the features `nrf` and `lpc55` respectively.

This may not be necessary with future releases of `embedded-hal`, which [adds the missing information](https://docs.rs/embedded-hal/1.0.0-alpha.11/embedded_hal/i2c/enum.ErrorKind.html).


### Iso7816

This driver uses the [`iso7816`](https://docs.rs/iso7816/latest/iso7816/) crate to implement serialization of APDUs.

### Generation of commands

To simplify implementation, all supported SE050 APDUs are described in `src/se050/commands.toml`.
The python script `generate_commands.py` parses the `command.toml` file and generates `src/se050/commands.rs`, which implements all the APDUs.

Funding
-------

[<img src="https://nlnet.nl/logo/banner.svg" width="200" alt="Logo NLnet: abstract logo of four people seen from above" hspace="20">](https://nlnet.nl/)
[<img src="https://nlnet.nl/image/logos/NGIAssure_tag.svg" width="200" alt="Logo NGI Assure: letterlogo shaped like a tag" hspace="20">](https://nlnet.nl/assure/)

This project was funded through the [NGI Assure](https://nlnet.nl/assure/) Fund, a fund established by [NLnet](https://nlnet.nl/) with financial support from the European Commission's [Next Generation Internet programme](https://ngi.eu/), under the aegis of DG Communications Networks, Content and Technology under grant agreement No 957073.
