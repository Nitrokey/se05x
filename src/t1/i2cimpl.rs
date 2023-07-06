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
