use std::convert::From;
use std::io;

extern crate serial;

#[derive(Debug)]
pub enum Error {
    SerialError(serial::Error),
    IOError(io::Error),
    ChecksumError,
    UnexpectedValue(u8),
}

impl From<serial::Error> for Error {
    fn from(error: serial::Error) -> Self {
        Error::SerialError(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IOError(error)
    }
}
