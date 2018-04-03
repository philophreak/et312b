use std::convert::From;
use std::time::Duration;

extern crate serial;
use serial::prelude::*;

enum Error {
    SerialError(serial::Error),
    SerialNotConnected,
}

impl From<serial::Error> for Error {
    fn from(error: serial::Error) -> Self {
        Error::SerialError(error)
    }
}

trait Estim<'a, T> {
    fn new(serial_port: &'a mut T) -> Self;
    fn init_connection(&mut self, &str) -> Result<(), Error>;
}

struct ET312B<'a, T: 'a + SerialPort> {
    serial: &'a mut T,
}

impl<'a, T: SerialPort> Estim<'a, T> for ET312B<'a, T> {
    fn new(serial_port: &'a mut T) -> Self {
        Self {
            serial: serial_port,
        }
    }

    fn init_connection(&mut self, serial_path: &str) -> Result<(), Error> {
        self.serial.reconfigure(&|settings| {
            settings.set_baud_rate(serial::Baud19200)?;
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowNone);
            Ok(())
        })?;

        self.serial.set_timeout(Duration::from_millis(1000))?;

        Ok(())
    }
}
