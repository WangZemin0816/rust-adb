use std::net::TcpStream;
use crate::basic::AsyncCommand;
use crate::adb_host::host_transport::AdbHostTransportCommand;
use crate::basic::AsyncProtocol;
use crate::client::DeviceService;
use crate::error::adb::AdbError;

pub struct DeviceClient {
    pub host: String,
    pub port: i32,
    pub serial_no: String,
}

impl DeviceClient {
    pub fn new(host: String, port: i32, serial_no: String) -> DeviceClient {
        DeviceClient {
            host,
            port,
            serial_no,
        }
    }
}

impl DeviceService for DeviceClient {

}
