use std::net::TcpStream;
use std::thread::JoinHandle;

use crate::error::adb::AdbError;

mod adb_client;
mod device_client;

pub trait AdbServer {
    fn start_server(&mut self);
    fn kill_server(&mut self);
    fn restart_server(&mut self);
}

pub trait HostServer {
    fn get_connection(&mut self) -> Result<TcpStream, AdbError>;
    fn get_version(&mut self) -> Result<String, AdbError>;
    fn disconnect(&mut self, host: String, port: i32) -> Result<(), AdbError>;
    fn list_devices(&mut self) -> Result<Vec<Device>, AdbError>;
    fn list_devices_with_path(&mut self) -> Result<Vec<DeviceWithPath>, AdbError>;
    fn track_devices(
        &mut self,
        on_change: fn(Vec<Device>),
        on_error: fn(AdbError),
    ) -> Result<JoinHandle<()>, AdbError>;
    fn kill(&mut self) -> Result<(), AdbError>;
    fn get_device(&mut self, serial_no: String) -> Result<Box<dyn DeviceService>, AdbError>;
}

pub trait DeviceService {
    fn get_connection(&mut self) -> Result<TcpStream, AdbError>;
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
