use crate::client::DeviceService;

pub struct DeviceClient {
    pub host: String,
    pub port: i32,
    pub serial_no: String,
}

impl DeviceClient {
    pub fn new(host: &String, port: &i32, serial_no: &String) -> DeviceClient {
        DeviceClient {
            host:host.clone(),
            port:port.clone(),
            serial_no:serial_no.clone(),
        }
    }
}

impl DeviceService for DeviceClient {}
