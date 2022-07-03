use std::error::Error;

#[derive(Debug)]
pub enum AdbError {
    TcpConnectError { source: Box<dyn Error> },
    TcpWriteError { source: Box<dyn Error> },
    TcpReadError { source: Box<dyn Error> },
    ParseResponseError { source: Box<dyn Error> },
    ResponseStatusError { message: String },
}
