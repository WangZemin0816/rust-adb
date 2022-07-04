use crate::error::adb::AdbError;
use std::net::TcpStream;

pub mod client;

pub trait DeviceService {
    fn get_connection(&mut self) -> Result<TcpStream, AdbError>;
}
