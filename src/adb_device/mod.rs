use std::net::TcpStream;
use crate::adb_device::client::DeviceClient;
use crate::error::adb::AdbError;

mod client;

pub trait DeviceService {
    fn get_connection(&mut self) -> Result<TcpStream, AdbError>;
}

pub fn new_device_client(host:String,port:i32,serial_no:String) -> impl DeviceService {
    DeviceClient {host, port,serial_no}
}
