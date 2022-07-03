use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub enum AdbError {
    TcpConnectError { source: Box<dyn Error> },
    TcpWriteError { source: Box<dyn Error> },
    TcpReadError { source: Box<dyn Error> },
    ParseResponseError{ source:  Box<dyn Error> },
    ResponseStatusError { message: String },
}

impl Debug for AdbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("adb error")
    }
}

impl Display for AdbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("adb error")
    }
}