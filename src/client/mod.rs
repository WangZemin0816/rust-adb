use std::collections::HashMap;
use std::fs::File;

use std::net::TcpStream;
use std::thread::JoinHandle;

use crate::error::adb::AdbError;

mod adb_client;
mod device_client;
mod tests;

pub trait AdbClient {
    fn start_server(&mut self) -> Result<(), AdbError>;
    fn kill_server(&mut self) -> Result<(), AdbError>;
    fn restart_server(&mut self) -> Result<(), AdbError>;
    fn get_connection(&mut self) -> Result<TcpStream, AdbError>;
    fn get_version(&mut self) -> Result<String, AdbError>;
    fn disconnect(&mut self, host: String, port: i32) -> Result<(), AdbError>;
    fn list_devices(&mut self) -> Result<Vec<Device>, AdbError>;
    fn list_devices_with_path(&mut self) -> Result<Vec<DeviceWithPath>, AdbError>;
    fn get_device(&mut self, serial_no: String) -> Result<Box<dyn DeviceService>, AdbError>;
    fn track_devices(
        &mut self, on_change: fn(Vec<Device>), on_error: fn(AdbError),
    ) -> Result<JoinHandle<()>, AdbError>;
}

pub trait DeviceService {
    fn push(&mut self, content: File, path: String, mode: i32) -> Result<String, AdbError>;
    fn shell_sync(&mut self, command: &String) -> Result<String, AdbError>;
    fn shell_async(&mut self, command: &String) -> Result<TcpStream, AdbError>;
    fn get_packages(&mut self, params: &String) -> Result<Vec<String>, AdbError>;
    fn get_features(&mut self) -> Result<HashMap<String, String>, AdbError>;
    fn get_properties(&mut self, params: &String) -> Result<HashMap<String, String>, AdbError>;
    fn logcat(
        &mut self, params: &String, consumer: fn(LogEntry), error_handler: fn(AdbError),
    ) -> Result<JoinHandle<()>, AdbError>;
}

#[derive(Debug)]
pub struct Device {
    pub serial_no: String,
    pub status: String,
}

#[derive(Debug)]
pub struct DeviceWithPath {
    pub serial_no: String,
    pub status: String,
    pub product: String,
    pub model: String,
    pub device: String,
    pub transport_id: String,
}

#[derive(Debug)]
pub struct LogEntry {
    pub pid: u32,
    pub tid: u32,
    pub tag: u32,
    pub sec: u32,
    pub nsec: u32,
    pub priority: u32,
    pub header: Vec<u8>,
    pub log: Vec<u8>,
}
