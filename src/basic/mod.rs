use crate::error::adb::AdbError;
use std::net::TcpStream;

pub mod connection;

pub trait SyncCommand {
    fn execute(&mut self) -> Result<SyncProtocol, AdbError>;
}

pub trait AsyncCommand {
    fn execute(&mut self) -> Result<AsyncProtocol, AdbError>;
}

pub enum SyncProtocol {
    OKAY { length: usize, content: String },
    FAIL { length: usize, content: String },
}

pub enum AsyncProtocol {
    OKAY { tcp_stream: TcpStream },
    FAIL { length: usize, content: String },
}
