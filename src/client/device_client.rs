use crate::adb_device::device_shell_async::DeviceAsyncShellCommand;
use crate::adb_device::device_shell_sync::DeviceSyncShellCommand;
use crate::adb_device::{AsyncDeviceCommand, SyncDeviceCommand, SyncDeviceProtocol};
use crate::client::{DeviceService, LogEntry};
use crate::error::adb::AdbError;
use std::collections::HashMap;
use std::fs::File;
use std::net::TcpStream;
use crate::adb_device::device_get_features::DeviceGetFeaturesCommand;
use crate::adb_device::device_get_packages::DeviceGetPackagesCommand;
use crate::adb_device::device_get_properties::DeviceGetPropertiesCommand;

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

    fn get_packages(&mut self, params: &String) -> Result<Vec<String>, AdbError> {
        let mut command = DeviceGetPackagesCommand::new(
            &self.host, &self.port, &self.serial_no, &params);
        let mut content = match command.execute() {
            Ok(response) => { response.content }
            Err(error) => { return Err(error); }
        };
        let mut packages = vec![];
        let lines: Vec<&str> = content.split_whitespace().collect();
        for line in lines {
            if !line.contains("package:") {
                continue;
            }
            let package = line.replace("package:", "");
            packages.push(package)
        }
        Ok(packages)
    }

    fn get_features(&mut self) -> Result<HashMap<String, String>, AdbError> {
        let mut command = DeviceGetFeaturesCommand::new(&self.host, &self.port, &self.serial_no);
        let mut content = match command.execute() {
            Ok(response) => { response.content.clone() }
            Err(error) => { return Err(error); }
        };
        let mut features = HashMap::new();
        let lines: Vec<&str> = content.split("\n").collect();
        for line in lines {
            let replace_item = line.replace("feature:", "");
            let line_item: Vec<&str> = replace_item.trim().split("=").collect();
            if line_item.len() < 2 {
                features.insert(String::from(line_item[0]), String::from("true"));
                continue;
            }
            features.insert(String::from(line_item[0]), String::from(line_item[1]));
        }
        Ok(features)
    }

    fn get_properties(&mut self, params: &String) -> Result<HashMap<String, String>, AdbError> {
        let mut command = DeviceGetPropertiesCommand::new(
            &self.host, &self.port, &self.serial_no, &params);
        let mut content = match command.execute() {
            Ok(response) => { response.content.clone() }
            Err(error) => { return Err(error); }
        };
        let mut properties = HashMap::new();
        let lines: Vec<&str> = content.split("\n").collect();
        for line in lines {
            let replace_item = line.replace("[", "").replace("]","");
            let line_item: Vec<&str> = replace_item.trim().split(":").collect();
            if line_item.len() < 2 {
                continue;
            }
            properties.insert(String::from(line_item[0].trim()), String::from(line_item[1].trim()));
        }
        Ok(properties)
    }

    fn logcat(
        &mut self, _params: &String, consumer: fn(LogEntry), error_handler: fn(AdbError),
    ) -> Result<(), AdbError> {
        todo!()
    }
}
