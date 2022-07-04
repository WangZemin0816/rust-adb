use crate::client::DeviceService;

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

impl DeviceService for DeviceClient {}
