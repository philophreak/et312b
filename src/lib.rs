use std::convert::From;
use std::time::Duration;

extern crate serial;
use serial::prelude::*;

enum Error {
    SerialError(serial::Error),
}

impl From<serial::Error> for Error {
    fn from(error: serial::Error) -> Self {
        Error::SerialError(error)
    }
}

trait Estim<T> {
    fn new(serial_port: T) -> Self;
    fn configure_connection(&mut self) -> Result<(), Error>;
}

struct ET312B<T: SerialPort> {
    serial: T,
}

impl<T: SerialPort> Estim<T> for ET312B<T> {
    fn new(serial_port: T) -> Self {
        Self {
            serial: serial_port,
        }
    }

    fn configure_connection(&mut self) -> Result<(), Error> {
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
