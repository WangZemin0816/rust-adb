use crate::adb_device::device_shell_async::DeviceAsyncShellCommand;
use crate::adb_device::device_shell_sync::DeviceSyncShellCommand;
use crate::adb_device::{AsyncDeviceCommand, SyncDeviceCommand};
use crate::client::{DeviceService, LogEntry};
use crate::error::adb::AdbError;
use std::collections::HashMap;
use std::fs::File;
use std::net::TcpStream;

pub struct DeviceClientImpl {
    pub host: String,
    pub port: i32,
    pub serial_no: String,
}

impl DeviceClientImpl {
    pub fn new(host: &String, port: &i32, serial_no: &String) -> DeviceClientImpl {
        DeviceClientImpl {
            host: host.clone(),
            port: port.clone(),
            serial_no: serial_no.clone(),
        }
    }
}

impl DeviceService for DeviceClientImpl {
    fn push(&mut self, _content: File, _path: String, _mode: i32) -> Result<String, AdbError> {
        todo!()
    }

    fn shell_sync(&mut self, command: &String) -> Result<String, AdbError> {
        let mut command =
            DeviceSyncShellCommand::new0(&self.host, &self.port, &self.serial_no, &command);
        match command.execute() {
            Ok(response) => Ok(response.content),
            Err(error) => Err(error),
        }
    }

    fn shell_async(&mut self, command: &String) -> Result<TcpStream, AdbError> {
        let mut command =
            DeviceAsyncShellCommand::new0(&self.host, &self.port, &self.serial_no, &command);
        match command.execute() {
            Ok(response) => Ok(response.tcp_stream),
            Err(error) => Err(error),
        }
    }

    fn get_packages(&mut self, _params: &String) -> Result<Vec<String>, AdbError> {
        todo!()
    }

    fn get_features(&mut self) -> Result<HashMap<String, String>, AdbError> {
        todo!()
    }

    fn get_properties(&mut self) -> Result<HashMap<String, String>, AdbError> {
        todo!()
    }

    fn logcat(
        &mut self, _params: &String, _consumer: fn(LogEntry), _error_handler: fn(AdbError),
    ) -> Result<(), AdbError> {
        todo!()
    }
}
