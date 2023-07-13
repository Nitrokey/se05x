#[cfg(feature = "nrf")]
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

#[cfg(feature = "lpc55")]
mod lpc55 {
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
