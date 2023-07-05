use embedded_hal::blocking::delay::DelayUs;
use hex_literal::hex;
use iso7816::{
    command::{class::ZERO_CLA, writer::IntoWriter, CommandBuilder},
    Status,
};

use crate::t1::{self, DataReceived, I2CForT1, T1oI2C};

pub struct Se050<Twi, D> {
    t1: T1oI2C<Twi, D>,
}

#[derive(Debug)]
pub enum Error {
    Unknown,
    T1(t1::Error),
    Status(Status),
}

impl From<t1::Error> for Error {
    fn from(value: t1::Error) -> Self {
        Self::T1(value)
    }
}

pub const APP_ID: &[u8] = &hex!("A0000003965453000000010300000000");

impl<Twi: I2CForT1, D: DelayUs<u32>> Se050<Twi, D> {
    pub fn new(twi: Twi, se_address: u8, delay: D) -> Self {
        Self {
            t1: T1oI2C::new(twi, se_address, delay),
        }
    }

    fn receive_apdu<'buf>(
        &mut self,
        buffer: &'buf mut [u8],
    ) -> Result<(&'buf [u8], Status), Error> {
        match self.t1.receive_data(buffer)? {
            DataReceived::IBlocks(len) if len >= 2 => Ok((
                &buffer[..len - 2],
                Status::from([buffer[len - 2], buffer[len - 1]]),
            )),
            DataReceived::SBlock { .. } => {
                error!("Got unexpected S-block");
                Err(Error::Unknown)
            }
            _ => {
                error!("Got too short apdu");
                Err(Error::Unknown)
            }
        }
    }

    pub fn enable(&mut self) -> Result<(), Error> {
        self.t1.resync()?;
        self.t1.interface_soft_reset(&mut [0; 64])?;
        let select_apdu = CommandBuilder::new(ZERO_CLA, 0xA4.into(), 0x04, 0x00, APP_ID, 0x00);
        let mut sender = self.t1.into_writer(select_apdu.required_len(true))?;
        select_apdu.serialize_into(&mut sender, true)?;
        let mut resp_buffer = [0; 9];
        let (atr, status) = self.receive_apdu(&mut resp_buffer)?;
        debug!("Got ATR: {atr:02x?}");
        if status != Status::SUCCESS {
            return Err(Error::Status(status));
        }
        Ok(())
    }
}
