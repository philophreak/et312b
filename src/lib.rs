use std::io::{Read, Write};

extern crate serial;
use serial::prelude::*;

pub mod errors;
mod utils;

const ENCRYPTION_KEY_XOR: u8 = 0x55;

pub struct ET312B<T: Read + Write> {
    serial: T,
    encryption_key: u8,
}

/// Configure the serial connection with the parameters expected by the
/// ET312B, such as the serial baud rate and parity settings.
pub fn open_serial_connection(path: &str) -> Result<serial::SystemPort, errors::Error> {
    let mut port = serial::open(&path)?;
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud19200)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })?;
    Ok(port)
}

impl<T: Read + Write> ET312B<T> {
    pub fn new(serial_port: T) -> Self {
        Self {
            serial: serial_port,
            encryption_key: 0,
        }
    }

    /// Perform a handshake with the ET312B device.
    ///
    /// # Warning
    /// If the handshake is performed on a device that was previously in use,
    /// it is expected that the same encryption key is used again. Also, it is
    /// not clear what will happen if we start writing bytes on a connection
    /// that was previously in use or corrupted.
    pub fn handshake(&mut self) -> Result<(), errors::Error> {
        let mut buf = [0u8; 1];
        self.serial.write_all(&[0x00])?;
        self.serial.read_exact(&mut buf)?;

        if buf[0] != 0x07 {
            return Err(errors::Error::UnexpectedValue(buf[0]));
        }

        Ok(())
    }

    /// Send the provided bytes over the serial connection. This method will
    /// automatically perform encryption and add a checksum to the bytes you
    /// are sending.
    pub fn send_packet(&mut self, data: &[u8]) -> Result<(), errors::Error> {
        self.serial
            .write_all(&utils::encrypt(&data, self.encryption_key))?;
        self.serial.write_all(&utils::encrypt(
            &[utils::checksum(data)],
            self.encryption_key,
        ))?;
        Ok(())
    }

    /// Read the specified number of bytes from the serial connection. This
    /// function will also read an extra byte for the checksum, and return an
    /// error if checksum validation fails. So, be sure to not include the
    /// checksum in the number of bytes you are reading.
    pub fn read_packet(&mut self, size: usize) -> Result<Vec<u8>, errors::Error> {
        let mut buf: Vec<u8> = vec![0; size];
        let mut checksum = [0u8; 1];
        self.serial.read_exact(&mut buf)?;
        self.serial.read_exact(&mut checksum)?;

        if utils::checksum(&buf) != checksum[0] {
            return Err(errors::Error::ChecksumError);
        }

        Ok(buf)
    }

    /// Performs the ET312B key exchange using a static "host key" of 0x00.
    /// This allows us to simplify computation of the final encryption key.
    ///
    /// Note: there is no need to perform a key exchange if your application
    /// only needs read access.
    pub fn key_exchange(&mut self) -> Result<u8, errors::Error> {
        self.send_packet(&[0x2f, 0x00])?;
        let kex_response = self.read_packet(2)?;

        if kex_response[0] != 0x21 {
            return Err(errors::Error::UnexpectedValue(kex_response[0]));
        }

        Ok(kex_response[1] ^ ENCRYPTION_KEY_XOR)
    }

    pub fn read_address(&mut self, address: u16) -> Result<u8, errors::Error> {
        self.send_packet(&[0x3c, (address & 0xff) as u8, (address >> 8) as u8])?;
        let response = self.read_packet(2)?;
        if response[0] != 0x22 {
            return Err(errors::Error::UnexpectedValue(response[0]));
        }
        Ok(response[1])
    }

    pub fn write_address(&mut self, address: u16, values: &[u8]) -> Result<(), errors::Error> {
        if values.len() > 12 {
            return Err(errors::Error::MessageTooLong);
        }

        let length_byte = ((values.len() << 4) + 0x3d) as u8;
        let mut buf: [u8; 15] = [
            length_byte,
            (address & 0xff) as u8,
            (address >> 8) as u8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        buf[3..(values.len() + 3)].clone_from_slice(&values);

        self.send_packet(&buf[0..3 + values.len()])?;
        let resp = self.read_packet(1)?;
        if resp[0] != 0x06 {
            return Err(errors::Error::UnexpectedValue(resp[0]));
        }

        Ok(())
    }
}
