use std::collections::HashMap;
use std::fs::File;
use std::net::TcpStream;
use crate::client::{DeviceService, LogEntry};
use crate::error::adb::AdbError;

pub struct DeviceClient {
    pub host: String,
    pub port: i32,
    pub serial_no: String,
}

impl DeviceClient {
    pub fn new(host: &String, port: &i32, serial_no: &String) -> DeviceClient {
        DeviceClient {
            host: host.clone(),
            port: port.clone(),
            serial_no: serial_no.clone(),
        }
    }
}

impl DeviceService for DeviceClient {
    fn push(&mut self, content: File, path: String, mode: i32) -> Result<String, AdbError> {
        todo!()
    }

    fn shell_sync(&mut self, command: &String) -> Result<String, AdbError> {
        todo!()
    }

    fn shell_async(&mut self, command: &String) -> Result<TcpStream, AdbError> {
        todo!()
    }

    fn get_packages(&mut self, params: &String) -> Result<Vec<String>, AdbError> {
        todo!()
    }

    fn get_features(&mut self) -> Result<HashMap<String, String>, AdbError> {
        todo!()
    }

    fn get_properties(&mut self) -> Result<HashMap<String, String>, AdbError> {
        todo!()
    }

    fn logcat(&mut self, params: &String, consumer: fn(LogEntry), error_handler: fn(AdbError)) -> Result<(), AdbError> {
        todo!()
    }
}
