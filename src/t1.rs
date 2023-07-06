use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
use hex_literal::hex;
use iso7816::command::writer::IntoWriter;
use iso7816::command::Writer;

pub type Crc = crc16::State<crc16::X_25>;

use core::fmt::{self, Debug};
use core::ops::Not;

use crate::macros::enum_u8;

mod i2cimpl;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Atr<'a> {
    /// Protocol version only `01` is supported
    pub pver: u8,
    /// VendorID,
    pub vid: &'a [u8; 5],
    /// Block waiting time
    pub bwt: u16,
    /// Maximum Information Field Size of the SE
    pub ifsc: u16,
    /// Maximum I2C clock frequency (kHz)
    pub mcf: u16,
    pub config: u8,
    /// Minimum polling time (ms)
    pub mpot: u8,
    /// Secure element guard time (microseconds)
    pub segt: u16,
    /// Wake-up time (microseconds)
    pub wut: u16,
    pub historical_bytes: &'a [u8],
}

impl<'a> Default for Atr<'a> {
    fn default() -> Self {
        Self {
            pver: 1,
            vid: &hex!("FFFFFFFFFF"),
            bwt: 0,
            ifsc: MAX_FRAME_DATA_LEN as _,
            mcf: 0,
            config: 0,
            mpot: 1,
            segt: SEGT_US as _,
            wut: 0,
            historical_bytes: &[],
        }
    }
}

impl<'a> Atr<'a> {
    /// If fails to parse, returns default values
    pub fn parse(data: &'a [u8]) -> Result<Self, Error> {
        // let atr = hex!("00a0000003960403e800fe020b03e80801000000006400000a4a434f5034204154504f");
        debug!("Parsing atr: {data:02x?}");
        if data.len() < 7 {
            error!("ATR Error 1");
            return Err(Error::Unknown);
        }
        let pver = data[0];
        let vid: &[u8; 5] = (&data[1..][..5]).try_into().unwrap();
        let dllp_len = data[6];

        let rem = &data[7..];

        if rem.len() < dllp_len as usize || dllp_len < 2 {
            error!("ATR Error 2");
            return Err(Error::Unknown);
        }
        let (dllp, rem) = rem.split_at(dllp_len as usize);

        let [bwt1, bwt2, ifsc1, ifsc2, ..] = dllp else {
            error!("ATR Error 3");
            return Err(Error::Unknown);
        };
        let bwt = u16::from_be_bytes([*bwt1, *bwt2]);
        let ifsc = u16::from_be_bytes([*ifsc1, *ifsc2]);

        if rem.is_empty() {
            error!("ATR Error 4");
            return Err(Error::Unknown);
        }

        let plid_len = rem[0];
        let rem = &rem[1..];
        if rem.len() < plid_len as usize {
            error!("ATR Error 6");
            return Err(Error::Unknown);
        }
        let (plid, rem) = rem.split_at(plid_len as usize);
        let [mcf1,mcf2, config, mpot,_rfu1, _rfu2,_rfu3,segt1,segt2,wut1,wut2,..] = plid else {
            error!("ATR Error 7");
            return Err(Error::Unknown);
        };
        let mcf = u16::from_be_bytes([*mcf1, *mcf2]);
        let segt = u16::from_be_bytes([*segt1, *segt2]);
        let wut = u16::from_be_bytes([*wut1, *wut2]);

        if rem.is_empty() {
            error!("ATR Error 8");
            return Err(Error::Unknown);
        }
        let hb_len = rem[0];
        let rem = &rem[1..];
        if rem.len() < hb_len as usize {
            error!("ATR Error 9");
            return Err(Error::Unknown);
        }

        let historical_bytes = &rem[..hb_len as usize];

        Ok(Self {
            pver,
            vid,
            bwt,
            ifsc,
            mcf,
            config: *config,
            mpot: *mpot,
            segt,
            wut,
            historical_bytes,
        })
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Seq(bool);

impl Seq {
    pub const ZERO: Self = Seq(false);
    pub const ONE: Self = Seq(true);
}

impl Not for Seq {
    type Output = Self;
    fn not(self) -> Self::Output {
        Seq(!self.0)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Pcb {
    I(Seq, bool),        // seq, multi
    S(SBlock),           // code, response?
    R(Seq, RBlockError), // seq, err
}

enum_u8!(
    #[rustfmt::skip]
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    pub enum SBlock {
        ResyncRequest =              0b11000000,
        ResyncResponse =             0b11100000,
        IfsRequest =                 0b11000001,
        IfsResponse =                0b11100001,
        AbortRequest =               0b11000010,
        AbortResponse =              0b11100010,
        WtxRequest =                 0b11000011,
        WtxResponse =                0b11100011,
        InterfaceSoftResetRequest =  0b11001111,
        InterfaceSoftResetResponse = 0b11101111,
        EndOfApduSessionRequest =    0b11000101,
        EndOfApduSessionResponse =   0b11100101,
        SeChipResetRequest =         0b11000110,
        SeChipResetResponse =        0b11100110,
        GetAtrRequest =              0b11000111,
        GetAtrResponse =             0b11100111,
    }
);

enum_u8!(
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    pub enum RBlockError {
        #![mask(0b11)]
        NoError = 0b00,
        CrcError = 0b01,
        OtherError = 0b10,
    }
);

/// PCB Mask
const I_BLOCK_PCB_MASK: u8 = 0b10011111;
/// PCB template
const I_BLOCK_PCB: u8 = 0b00000000;
/// SEQ mask
const I_BLOCK_SEQ: u8 = 0b01000000;
/// MORE mask
const I_BLOCK_MOR: u8 = 0b00100000;

/// PCB template
const R_BLOCK_PCB: u8 = 0b10000000;
/// PCB template
const R_BLOCK_PCB_MASK: u8 = 0b11101100;
/// SEQ mask
const R_BLOCK_SEQ: u8 = 0b00010000;
/// CRC mask
const R_BLOCK_ERROR_MASK: u8 = 0b00000011;

impl Pcb {
    pub fn to_byte(self) -> u8 {
        match self {
            Self::I(seq, multi) => Self::i_pcb(seq, multi),
            Self::S(block) => Self::s_pcb(block),
            Self::R(seq, err) => Self::r_pcb(seq, err),
        }
    }

    pub fn i_pcb(seq: Seq, multi: bool) -> u8 {
        let mut pcb = I_BLOCK_PCB;
        if multi {
            pcb |= I_BLOCK_MOR;
        }

        if seq == Seq::ONE {
            pcb |= I_BLOCK_SEQ;
        }
        pcb
    }
    pub fn s_pcb(block: SBlock) -> u8 {
        block.into()
    }
    pub fn r_pcb(seq: Seq, error: RBlockError) -> u8 {
        let mut pcb = R_BLOCK_PCB;
        if seq == Seq::ONE {
            pcb |= R_BLOCK_SEQ;
        }

        pcb |= error as u8;
        pcb
    }

    pub fn parse(value: u8) -> Result<Self, Error> {
        if value & I_BLOCK_PCB_MASK == I_BLOCK_PCB {
            let seq = if value & I_BLOCK_SEQ == 0 {
                Seq::ZERO
            } else {
                Seq::ONE
            };

            let more = !(value & I_BLOCK_MOR == 0);
            return Ok(Self::I(seq, more));
        }

        if value & R_BLOCK_PCB_MASK == R_BLOCK_PCB {
            let seq = if value & R_BLOCK_SEQ == 0 {
                Seq::ZERO
            } else {
                Seq::ONE
            };
            let error = (value & R_BLOCK_ERROR_MASK)
                .try_into()
                .map_err(|_| Error::BadPcb)?;
            return Ok(Self::R(seq, error));
        }

        if let Ok(sblock) = value.try_into() {
            return Ok(Self::S(sblock));
        }

        Err(Error::BadPcb)
    }
}

pub trait I2CErrorNack: Debug {
    fn is_address_nack(&self) -> bool;
    fn is_data_nack(&self) -> bool;
}
pub trait I2CForT1:
    Read<u8, Error = <Self as I2CForT1>::Error>
    + Write<u8, Error = <Self as I2CForT1>::Error>
    + WriteRead<u8, Error = <Self as I2CForT1>::Error>
{
    type Error: I2CErrorNack;
}

impl<T> I2CForT1 for T
where
    T: Read<u8>
        + Write<u8, Error = <T as Read<u8>>::Error>
        + WriteRead<u8, Error = <T as Read<u8>>::Error>,
    <T as Read<u8>>::Error: I2CErrorNack,
{
    type Error = <T as Read<u8>>::Error;
}

#[derive(Debug)]
pub enum Error {
    Unknown,
    AddressNack,
    DataNack,
    BadCrc,
    BadPcb,
    BadAddress,
    ReceptionBuffer,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => f.write_str("Unknown T=1 error"),
            Self::AddressNack => f.write_str("NACK on from the device address"),
            Self::DataNack => f.write_str("Nack for data write"),
            Self::BadCrc => f.write_str("CRC error"),
            Self::BadPcb => f.write_str("Received invalid PCB"),
            Self::BadAddress => f.write_str("Bad address"),
            Self::ReceptionBuffer => f.write_str("Reception buffer is too small"),
        }
    }
}

impl iso7816::command::writer::Error for Error {
    fn failed_serialization(_cause: &'static str) -> Self {
        Self::Unknown
    }
}

pub struct T1oI2C<Twi, D> {
    twi: Twi,
    se_address: u8,
    nad_hd2se: u8,
    nad_se2hd: u8,
    iseq_snd: Seq,
    iseq_rcv: Seq,
    /// microseconds
    mpot: u32,
    delay: D,
    segt: u32,
}

// const TWI_RETRIES: usize = 128;
// const TWI_RETRY_DELAY_MS: u32 = 2;
// const TWI_RETRY_DELAY_US: u32 = TWI_RETRY_DELAY_MS * 1000;
/// SEGT value in microseconds
/// Minimun time between reading attempts
const SEGT_US: u32 = 10;

/// See table 4 of UM1225
const NAD_HD_TO_SE: u8 = 0x5A;
/// See table 4 of UM1225
const NAD_SE_TO_HD: u8 = 0xA5;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataReceived {
    /// Received one or more IBlocks
    ///
    /// Returns the size of data written to the reception buffer
    IBlocks(usize),
    SBlock {
        block: SBlock,
        /// Any data written prior to receiving the s block
        i_data: usize,
        s_data: usize,
    },
}

impl<Twi: I2CForT1, D: DelayUs<u32>> T1oI2C<Twi, D> {
    pub fn new(twi: Twi, se_address: u8, delay: D) -> Self {
        // Default MPOT value.
        // TODO: get from ATR
        const DMPOT_MS: u32 = 1;
        Self {
            twi,
            se_address,
            // See table 4
            nad_hd2se: NAD_HD_TO_SE,
            nad_se2hd: NAD_SE_TO_HD,
            iseq_snd: Seq::ZERO,
            iseq_rcv: Seq::ZERO,
            mpot: DMPOT_MS * 1000,
            segt: SEGT_US as _,
            delay,
        }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), Error> {
        match self.twi.write(self.se_address, data) {
            Ok(_) => return Ok(()),
            Err(err) if err.is_address_nack() => Err(Error::AddressNack),
            Err(err) if err.is_data_nack() => Err(Error::DataNack),
            Err(err) => {
                warn!("Unknown error when writing: {:?}", err);
                Err(Error::Unknown)
            }
        }
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        match self.twi.read(self.se_address, buffer) {
            Ok(_) => return Ok(()),
            Err(err) if err.is_address_nack() => Err(Error::AddressNack),
            Err(err) if err.is_data_nack() => Err(Error::DataNack),
            Err(err) => {
                warn!("Unknown error when writing: {:?}", err);
                Err(Error::Unknown)
            }
        }
    }

    // Not actually used as discouraged by 3.1.1.1
    pub fn write_read(&mut self, data: &[u8], buffer: &mut [u8]) -> Result<(), Error> {
        match self.twi.write_read(self.se_address, data, buffer) {
            Ok(_) => return Ok(()),
            Err(err) if err.is_address_nack() => Err(Error::AddressNack),
            Err(err) if err.is_data_nack() => Err(Error::DataNack),
            Err(err) => {
                warn!("Unknown error when writing: {:?}", err);
                Err(Error::Unknown)
            }
        }
    }

    pub fn receive_data(&mut self, buffer: &mut [u8]) -> Result<DataReceived, Error> {
        let mut header_buffer = [0; HEADER_LEN];
        let mut written = 0;
        let mut crc_buf = [0; TRAILER_LEN];
        loop {
            trace!("Receive data loop, currently written {written}");
            match self.read(&mut header_buffer) {
                Ok(()) => {}
                Err(Error::AddressNack) => {
                    self.delay.delay_us(self.mpot);
                    continue;
                }
                Err(err) => return Err(err),
            }

            let [nad, pcb, len] = header_buffer;

            if buffer.len() < len as usize {
                error!("Buffer too small");
                return Err(Error::ReceptionBuffer);
            }

            let current_buf = &mut buffer[written..][..len as usize];

            debug!("Received header: {:02x?}", header_buffer);
            if nad != self.nad_se2hd {
                error!("Received bad nad: {:02x}", nad);
                return Err(Error::BadAddress);
            }

            if len != 0 {
                self.read(current_buf)?;
                written += len as usize;
            }
            self.read(&mut crc_buf)?;

            let pcb = Pcb::parse(pcb).map_err(|_| Error::BadPcb)?;

            let mut crc = Crc::new();
            crc.update(&header_buffer);
            crc.update(&current_buf);
            let crc = crc.get().to_le_bytes();
            if crc_buf != crc {
                error!(
                    "Got bad crc: {:02x?} expected {:02x?}",
                    &current_buf[..len as usize][..2],
                    crc
                );
                // TODO: write R-Block with error
                return Err(Error::BadCrc);
            }

            let (seq, more) = match pcb {
                Pcb::S(block) => {
                    return Ok(DataReceived::SBlock {
                        block,
                        i_data: written - len as usize,
                        s_data: len as usize,
                    })
                }
                Pcb::R(_, _) => {
                    error!("Got unexpected R-Block in receive");
                    return Err(Error::Unknown);
                }
                Pcb::I(seq, more) => (seq, more),
            };
            if !more {
                return Ok(DataReceived::IBlocks(written));
            }
            if seq != self.iseq_rcv {
                warn!("Got bad seq");
            }
            self.iseq_rcv = !seq;

            let frame = [
                self.nad_hd2se,
                Pcb::R(!seq, RBlockError::NoError).to_byte(),
                0,
            ];
            let [crc1, crc2] = Crc::calculate(&frame).to_le_bytes();
            self.write(&[frame[0], frame[1], frame[2], crc1, crc2])?;
        }
    }

    pub fn resync(&mut self) -> Result<(), Error> {
        trace!("Resync");
        let header = [self.nad_hd2se, Pcb::S(SBlock::ResyncRequest).to_byte(), 0];
        let [crc1, crc2] = Crc::calculate(&header).to_le_bytes();
        self.write(&[header[0], header[1], header[2], crc1, crc2])?;
        self.delay.delay_us(self.segt);
        let data = self.receive_data(&mut [])?;
        if !matches!(
            data,
            DataReceived::SBlock {
                block: SBlock::ResyncResponse,
                i_data: 0,
                s_data: 0
            }
        ) {
            error!("Got unexpected error: {data:?}");
            return Err(Error::BadPcb);
        }
        Ok(())
    }

    // TODO: find proper length for buffer
    pub fn interface_soft_reset<'buf>(
        &mut self,
        buffer: &'buf mut [u8; 64],
    ) -> Result<Atr<'buf>, Error> {
        trace!("Interface Soft Reset");
        let header = [
            self.nad_hd2se,
            Pcb::S(SBlock::InterfaceSoftResetRequest).to_byte(),
            0,
        ];
        let [crc1, crc2] = Crc::calculate(&header).to_le_bytes();
        self.write(&[header[0], header[1], header[2], crc1, crc2])?;
        self.delay.delay_us(self.segt);
        let data = self.receive_data(buffer)?;
        let received = if let DataReceived::SBlock {
            block: SBlock::InterfaceSoftResetResponse,
            i_data: 0,
            s_data,
        } = data
        {
            s_data
        } else {
            error!("Got unexpected error: {data:?}");
            return Err(Error::BadPcb);
        };
        let atr = Atr::parse(&buffer[..received]);
        if let Ok(atr) = &atr {
            self.mpot = atr.mpot.into();
            self.segt = atr.segt.into();
        };
        self.iseq_snd = Seq::ZERO;
        self.iseq_rcv = Seq::ZERO;
        Ok(atr.unwrap_or_default())
    }
}

/// UM1225 2.1.1
const MAX_FRAME_DATA_LEN: usize = 0xFE;
const HEADER_LEN: usize = 3;
const TRAILER_LEN: usize = 2;
const MAX_FRAME_LEN: usize = MAX_FRAME_DATA_LEN + HEADER_LEN + TRAILER_LEN;

pub struct FrameSender<'writer, Twi, D> {
    writer: &'writer mut T1oI2C<Twi, D>,
    /// Total amount of application data that will be written
    data: usize,
    /// Amount of application data already written, includes data currently in `current_frame_buffer`
    written: usize,
    current_frame_buffer: [u8; MAX_FRAME_LEN],
}

impl<'writer, Twi: I2CForT1, D: DelayUs<u32>> FrameSender<'writer, Twi, D> {
    fn current_offset(&self) -> usize {
        self.written % MAX_FRAME_DATA_LEN
    }

    pub fn new(writer: &'writer mut T1oI2C<Twi, D>, data: usize) -> Self {
        Self {
            writer,
            data,
            written: 0,
            current_frame_buffer: [0; MAX_FRAME_LEN],
        }
    }

    pub fn write_data(&mut self, data: &[u8]) -> Result<usize, Error> {
        debug!("Writing data: {:02x?}", data);
        if data.is_empty() {
            return Ok(0);
        }
        if data.len() + self.written > self.data {
            error!("Writing more data than expected");
            return Err(Error::Unknown);
        }

        let mut rem = data;
        while !rem.is_empty() {
            let current_offset = self.current_offset();
            let available_in_frame = MAX_FRAME_DATA_LEN - current_offset;
            let chunk_len = available_in_frame.min(rem.len());
            let (chunk, next_rem) = rem.split_at(chunk_len);
            rem = next_rem;
            self.written += chunk_len;
            self.current_frame_buffer[HEADER_LEN + current_offset..][..chunk_len]
                .copy_from_slice(chunk);

            if chunk_len == available_in_frame {
                // frame is full
                self.send_current_frame()?;
            } else if self.written == self.data {
                // if fully written, send remaining buffered data
                self.send_current_frame()?;
            }
        }

        Ok(data.len())
    }

    pub fn send_current_frame(&mut self) -> Result<(), Error> {
        let data_len = self.current_offset();
        let is_last = self.written == self.data;
        let pcb = Pcb::I(self.writer.iseq_snd, !is_last).to_byte();

        self.writer.iseq_snd = !self.writer.iseq_snd;

        let header = [self.writer.nad_hd2se, pcb, data_len as u8];
        self.current_frame_buffer[0..HEADER_LEN].copy_from_slice(&header);
        let trailer =
            Crc::calculate(&self.current_frame_buffer[..HEADER_LEN + data_len]).to_le_bytes();
        self.current_frame_buffer[HEADER_LEN + data_len..][..TRAILER_LEN].copy_from_slice(&trailer);
        trace!(
            "Sending:\n\tHeader: {:02x?}\n\tData: {:02x?}\n\tTrailer: {:02x?}",
            &self.current_frame_buffer[..HEADER_LEN],
            &self.current_frame_buffer[HEADER_LEN..][..data_len],
            &self.current_frame_buffer[HEADER_LEN + data_len..][..TRAILER_LEN],
        );
        self.writer
            .write(&self.current_frame_buffer[..data_len + HEADER_LEN + TRAILER_LEN])?;

        if is_last {
            // No R-BLOCK expected for non chained I block
            return Ok(());
        }

        let mut resp_buf = [0u8; 5];
        self.writer.read(&mut resp_buf)?;
        debug!("Got R-Block: {:02x?}", resp_buf);
        let [nad, pcb, len, crc1, crc2] = resp_buf;

        if nad != self.writer.nad_se2hd {
            error!("Received bad nad: {:02x}", nad);
            return Err(Error::BadAddress);
        }

        let pcb = Pcb::parse(pcb);

        match pcb {
            Ok(Pcb::R(seq, RBlockError::NoError)) if seq == self.writer.iseq_snd => {}
            Ok(Pcb::R(_, RBlockError::NoError)) => {
                warn!("Got incorrect expected sequence");
            }
            Ok(Pcb::R(_, RBlockError::CrcError)) => {
                error!("Got CrcError");
                return Err(Error::BadCrc);
            }
            _ => {
                error!("Got bad PCB: {pcb:?}");
                return Err(Error::BadPcb);
            }
        }

        if len != 0 {
            error!("Received R-block with bad len: {}", len);
            return Err(Error::BadAddress);
        }

        let crc = Crc::calculate(&resp_buf[0..HEADER_LEN]).to_le_bytes();
        if [crc1, crc2] != crc {
            error!(
                "Got bad crc. Got {:02x?}, expected {:02x?}",
                [crc1, crc2],
                crc
            );
            return Err(Error::BadCrc);
        }

        Ok(())
    }
}

impl<'writer, Twi: I2CForT1, D: DelayUs<u32>> Writer for FrameSender<'writer, Twi, D> {
    type Error = Error;
    fn write(&mut self, data: &[u8]) -> Result<usize, Self::Error> {
        self.write_data(data)
    }
}

impl<'writer, Twi: I2CForT1, D: DelayUs<u32>> IntoWriter for &'writer mut T1oI2C<Twi, D> {
    type Writer = FrameSender<'writer, Twi, D>;
    fn into_writer(self, to_write: usize) -> Result<Self::Writer, <Self::Writer as Writer>::Error> {
        Ok(FrameSender::new(self, to_write))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_round_trip(value: u8, pcb: Pcb) {
        assert_eq!(
            value,
            pcb.to_byte(),
            "Expected 0b{value:08b}, got 0b{:08b}",
            pcb.to_byte()
        );
        assert_eq!(pcb, Pcb::parse(value).unwrap());
    }

    #[test]
    fn i_pcb() {
        assert_round_trip(0b01100000, Pcb::I(Seq::ONE, true));
        assert_round_trip(0b01000000, Pcb::I(Seq::ONE, false));
        assert_round_trip(0b00100000, Pcb::I(Seq::ZERO, true));
        assert_round_trip(0b00000000, Pcb::I(Seq::ZERO, false));
    }

    #[test]
    fn r_pcb() {
        assert_round_trip(0b10010000, Pcb::R(Seq::ONE, RBlockError::NoError));
        assert_round_trip(0b10000000, Pcb::R(Seq::ZERO, RBlockError::NoError));

        assert_round_trip(0b10010001, Pcb::R(Seq::ONE, RBlockError::CrcError));
        assert_round_trip(0b10000001, Pcb::R(Seq::ZERO, RBlockError::CrcError));

        assert_round_trip(0b10010010, Pcb::R(Seq::ONE, RBlockError::OtherError));
        assert_round_trip(0b10000010, Pcb::R(Seq::ZERO, RBlockError::OtherError));
    }

    #[test]
    fn atr() {
        let atr: [u8; 0x23] =
            hex!("00a0000003960403e800fe020b03e80801000000006400000a4a434f5034204154504f");
        assert_eq!(Atr::parse(&atr).unwrap(), Atr::default());
    }
}
