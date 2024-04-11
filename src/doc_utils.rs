// utilities for doctests
//
// Not included in the crate, meant to be used with `include!`

use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};

#[derive(Debug)]
pub struct DummyI2c;
#[derive(Debug)]
pub struct DummyI2cError;

impl se05x::t1::I2CErrorNack for DummyI2cError {
    fn is_address_nack(&self) -> bool {
        false
    }
    fn is_data_nack(&self) -> bool {
        false
    }
}

impl Read<u8> for DummyI2c {
    type Error = DummyI2cError;
    fn read(&mut self, _: u8, _: &mut [u8]) -> Result<(), DummyI2cError> {
        unimplemented!()
    }
}
impl Write<u8> for DummyI2c {
    type Error = DummyI2cError;
    fn write(&mut self, _: u8, _: &[u8]) -> Result<(), DummyI2cError> {
        unimplemented!()
    }
}
impl WriteRead<u8> for DummyI2c {
    type Error = DummyI2cError;
    fn write_read(&mut self, _: u8, _: &[u8], _: &mut [u8]) -> Result<(), Self::Error> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct DummyDelay;

impl DelayUs<u32> for DummyDelay {
    fn delay_us(&mut self, _: u32) {
        unimplemented!()
    }
}

pub fn get_i2c() -> impl se05x::t1::I2CForT1 {
    unimplemented!();
    DummyI2c
}

pub fn get_delay() -> impl DelayUs<u32> {
    unimplemented!();
    DummyDelay
}
