// traits to be used internally for I2C write
pub mod i2c {
    use crate::embedded_hal;

    pub trait Write<A> {
        type Error;

        fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error>;
    }

    #[cfg(feature = "embedded-hal-v0.2.7")]
    impl<T, A> Write<A> for T
    where
        T: embedded_hal_v0_2_7::blocking::i2c::Write,
    {
        type Error = T::Error;

        fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), T::Error> {
            self.write(address, bytes)
        }
    }

    #[cfg(feature = "embedded-hal-v1.0")]
    impl<T, A> Write<A> for T
    where
        T: embedded_hal_v1_0::i2c::I2c,
    {
        type Error = T::Error;

        fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), T::Error> {
            self.write(address, bytes)
        }
    }

    pub trait Read<A> {
        type Error;

        fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error>;
    }

    #[cfg(feature = "embedded-hal-v0.2.7")]
    impl<T, A> Read<A> for T
    where
        T: embedded_hal_v0_2_7::blocking::i2c::Read,
    {
        type Error = T::Error;

        fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), T::Error> {
            self.read(address, buffer)
        }
    }

    #[cfg(feature = "embedded-hal-v1.0")]
    impl<T, A> Read<A> for T
    where
        T: embedded_hal_v1_0::i2c::I2c,
    {
        type Error = T::Error;

        fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), T::Error> {
            self.read(address, buffer)
        }
    }

    pub trait WriteRead<A> {
        type Error;

        fn write_read(
            &mut self,
            address: u8,
            bytes: &[u8],
            buffer: &mut [u8],
        ) -> Result<(), Self::Error>;
    }

    #[cfg(feature = "embedded-hal-v0.2.7")]
    impl<T, A> WriteRead<A> for T
    where
        T: embedded_hal_v0_2_7::blocking::i2c::WriteRead,
    {
        type Error = T::Error;

        fn write_read(
            &mut self,
            address: u8,
            bytes: &[u8],
            buffer: &mut [u8],
        ) -> Result<(), T::Error> {
            self.write_read(address, bytes, buffer)
        }
    }

    #[cfg(feature = "embedded-hal-v1.0")]
    impl<T, A> WriteRead<A> for T
    where
        T: embedded_hal_v1_0::i2c::I2c,
    {
        type Error = T::Error;

        fn write_read(
            &mut self,
            address: u8,
            bytes: &[u8],
            buffer: &mut [u8],
        ) -> Result<(), T::Error> {
            self.write_read(address, bytes, buffer)
        }
    }
}

// trait to be used internally for Delay
pub trait Delay {
    fn delay_us(&mut self, us: u32);
}

#[cfg(feature = "embedded-hal-v0.2.7")]
impl<T> Delay for T
where
    T: embedded_hal_v0_2_7::blocking::delay::DelayUs<u32>,
{
    fn delay_us(&mut self, us: u32) {
        self.delay_us(us);
    }
}

#[cfg(feature = "embedded-hal-v1.0")]
impl<T> Delay for T
where
    T: embedded_hal_v1_0::delay::DelayNs,
{
    fn delay_us(&mut self, us: u32) {
        self.delay_us(us);
    }
}
