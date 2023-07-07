use bitflags::bitflags;
use embedded_hal::blocking::delay::DelayUs;
use hex_literal::hex;
use iso7816::{
    command::{
        class::{Class, ZERO_CLA},
        writer::IntoWriter,
        CommandBuilder, DataSource, DataStream, Writer,
    },
    Instruction, Status,
};

use crate::t1::{self, DataReceived, FrameSender, I2CForT1, T1oI2C};
pub mod commands;

pub struct Se050<Twi, D> {
    t1: T1oI2C<Twi, D>,
}

#[derive(Debug)]
pub enum Error {
    Unknown,
    T1(t1::Error),
    Status(Status),
    Tlv,
}

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        match value {
            Error::Status(status) => status,
            _ => {
                error!("Unknown error {value:?}");
                Status::ERROR
            }
        }
    }
}

impl From<t1::Error> for Error {
    fn from(value: t1::Error) -> Self {
        Self::T1(value)
    }
}

pub trait Se050Command<W: Writer>: DataStream<W> {
    type Response<'a>: TryFrom<&'a [u8], Error = Error>;
}

pub const APP_ID: [u8; 0x10] = hex!("A0000003965453000000010300000000");

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
        let mut resp_buffer = [0; 9];
        let atr = self.run_command(&Select, &mut resp_buffer)?;
        debug!("Got ATR: {atr:02x?}");
        Ok(())
    }

    pub fn run_command<'buf, C: for<'a> Se050Command<FrameSender<'a, Twi, D>>>(
        &mut self,
        command: &C,
        response_buf: &'buf mut [u8],
    ) -> Result<<C as Se050Command<FrameSender<'_, Twi, D>>>::Response<'buf>, Error> {
        let mut sender = self.t1.into_writer(command.len())?;
        command.to_writer(&mut sender)?;
        let (response, status) = self.receive_apdu(response_buf)?;
        if status != Status::SUCCESS {
            return Err(Error::Status(status));
        }
        response.try_into()
    }
}

bitflags! {
    #[derive(Debug,Clone,Copy)]
    struct AppletConfig: u16 {
        const ECDAA = 0x0001;
        const ECDSA_ECDH_ECDHE = 0x0002;
        const EDDSA = 0x0004;
        const DH_MONT = 0x0008;
        const HMAC = 0x0010;
        const RSA_PLAIN = 0x0020;
        const RSA_CRT = 0x0040;
        const AES = 0x0080;
        const DES = 0x0100;
        const PBKDF = 0x0200;
        const TLS = 0x0400;
        const MIFARE = 0x0800;
        const FIPS_MODE_DISABLED = 0x1000;
        const I2CM = 0x2000;
        const ECC_ALL = 0x000F;
        const RSA_ALL = 0x0060;
        const ALL = 0x3FFF;
    }
}

pub struct Select;
#[derive(Debug, Clone, Copy)]
pub struct Atr {
    major: u8,
    minor: u8,
    patch: u8,
    secure_box_major: u8,
    secure_box_minor: u8,
    applet_config: AppletConfig,
}

impl Atr {
    fn parse(atr: &[u8]) -> Result<Self, Error> {
        debug!("Parsing SELECT atr");
        let [major, minor, patch, config1, config2, secure_box_major, secure_box_minor] = atr else {
            return Err(Error::Unknown);
        };

        let applet_config =
            AppletConfig::from_bits_retain(u16::from_be_bytes([*config1, *config2]));
        Ok(Atr {
            major: *major,
            minor: *minor,
            patch: *patch,
            secure_box_major: *secure_box_major,
            secure_box_minor: *secure_box_minor,
            applet_config,
        })
    }
}

impl<'a> TryFrom<&[u8]> for Atr {
    type Error = Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl Select {
    fn command(&self) -> CommandBuilder<'static, [u8]> {
        CommandBuilder::new(ZERO_CLA, 0xA4.into(), 0x04, 0x00, &APP_ID, 7)
    }
}

impl DataSource for Select {
    fn len(&self) -> usize {
        self.command().len()
    }
    fn is_empty(&self) -> bool {
        false
    }
}

impl<W: Writer> DataStream<W> for Select {
    fn to_writer(&self, writer: &mut W) -> Result<(), <W as Writer>::Error> {
        self.command().to_writer(writer)
    }
}

impl<W: Writer> Se050Command<W> for Select {
    type Response<'a> = Atr;
}
