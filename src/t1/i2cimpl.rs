// Copyright (C) 2023 Nitrokey GmbH
// SPDX-License-Identifier: LGPL-3.0-only

#[cfg(feature = "lpc55-v0.4")]
mod lpc55_04 {
    use crate::t1::I2CErrorNack;

    use lpc55_hal_04::drivers::i2c::Error;

    impl I2CErrorNack for Error {
        fn is_address_nack(&self) -> bool {
            matches!(self, Error::NackAddress)
        }
        fn is_data_nack(&self) -> bool {
            matches!(self, Error::NackData)
        }
    }
}
