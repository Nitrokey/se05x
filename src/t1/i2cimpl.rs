// Copyright (C) 2023 Nitrokey GmbH
// SPDX-License-Identifier: LGPL-3.0-only

#[cfg(all(feature = "nrf", feature = "embedded-hal-v0.2.7"))]
mod nrf52832 {
    use crate::t1::I2CErrorNack;

    use nrf_hal_common::twim::Error;

    impl I2CErrorNack for Error {
        fn is_address_nack(&self) -> bool {
            matches!(self, Error::AddressNack)
        }
        fn is_data_nack(&self) -> bool {
            matches!(self, Error::DataNack)
        }
    }
}

#[cfg(all(feature = "lpc55-v0.3", feature = "embedded-hal-v0.2.7"))]
mod lpc55_03 {
    use crate::t1::I2CErrorNack;

    use lpc55_hal::drivers::i2c::Error;

    impl I2CErrorNack for Error {
        fn is_address_nack(&self) -> bool {
            matches!(self, Error::NackAddress)
        }
        fn is_data_nack(&self) -> bool {
            matches!(self, Error::NackData)
        }
    }
}

#[cfg(all(feature = "lpc55-v0.4", feature = "embedded-hal-v0.2.7"))]
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

#[cfg(feature = "embedded-hal-v1.0")]
impl crate::t1::I2CErrorNack for embedded_hal_v1_0::i2c::ErrorKind {
    fn is_address_nack(&self) -> bool {
        matches!(
            self,
            embedded_hal_v1_0::i2c::ErrorKind::NoAcknowledge(
                embedded_hal_v1_0::i2c::NoAcknowledgeSource::Address
            )
        )
    }
    fn is_data_nack(&self) -> bool {
        matches!(
            self,
            embedded_hal_v1_0::i2c::ErrorKind::NoAcknowledge(
                embedded_hal_v1_0::i2c::NoAcknowledgeSource::Data
            )
        )
    }
}
