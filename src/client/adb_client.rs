use std::net::TcpStream;
use std::thread;
use std::thread::JoinHandle;

use crate::adb_host::{
    connect, read_response_content, read_response_length, AsyncHostCommand, HostConnectionInfo,
    SyncHostCommand,
};

use crate::adb_host::host_disconnect::AdbHostDisconnectCommand;
use crate::adb_host::host_kill::AdbHostKillCommand;
use crate::adb_host::host_list_device::AdbHostListDevicesCommand;
use crate::adb_host::host_list_device_l::AdbHostListDeviceLCommand;
use crate::adb_host::host_start::AdbHostStartCommand;
use crate::adb_host::host_track_devices::AdbHostTrackDeviceCommand;

use crate::adb_host::host_version::AdbHostVersionCommand;
use crate::client::device_client::DeviceClientImpl;
use crate::client::{AdbClient, Device, DeviceService, DeviceWithPath};
use crate::error::adb::AdbError;

pub struct AdbClientImpl {
    pub host: String,
    pub port: i32,
    pub bin_path: String,
}

impl AdbClient for AdbClientImpl {
    fn start_server(&mut self) -> Result<(), AdbError> {
        let mut command = AdbHostStartCommand::new(&self.host, &self.port, &self.bin_path);
        command.execute()?;
        Ok(())
    }

    fn kill_server(&mut self) -> Result<(), AdbError> {
        let mut command = AdbHostKillCommand::new(&self.host, &self.port);
        match command.execute() {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    fn restart_server(&mut self) -> Result<(), AdbError> {
        self.kill_server()?;
        self.start_server()
    }

    fn get_connection(&mut self) -> Result<TcpStream, AdbError> {
        connect(&HostConnectionInfo::new(&self.host, &self.port))
    }

    fn get_version(&mut self) -> Result<String, AdbError> {
        let mut command = AdbHostVersionCommand::new(&self.host, &self.port);
        match command.execute() {
            Ok(response) => Ok(response.content),
            Err(error) => Err(error),
        }
    }

    fn disconnect(&mut self, host: String, port: i32) -> Result<(), AdbError> {
        let mut command = AdbHostDisconnectCommand::new(&self.host, &self.port, &host, &port);
        match command.execute() {
            Ok(_response) => Ok(()),
            Err(error) => Err(error),
        }
    }

    fn list_devices(&mut self) -> Result<Vec<Device>, AdbError> {
        let mut command = AdbHostListDevicesCommand::new(&self.host, &self.port);
        let sync_host_response = command.execute()?;
        let mut devices = vec![];
        for line in sync_host_response.content.lines() {
            let contents: Vec<&str> = line.trim().split_whitespace().collect();
            if contents.len() >= 2 {
                devices.push(Device {
                    serial_no: String::from(contents[0]),
                    status: String::from(contents[1]),
                })
            }
        }
        Ok(devices)
    }

    fn list_devices_with_path(&mut self) -> Result<Vec<DeviceWithPath>, AdbError> {
        let mut command = AdbHostListDeviceLCommand::new(&self.host, &self.port);
        let sync_host_response = command.execute()?;
        let mut devices = vec![];
        for line in sync_host_response.content.lines() {
            let contents: Vec<&str> = line.trim().split_whitespace().collect();
            if contents.len() >= 6 {
                devices.push(DeviceWithPath {
                    serial_no: String::from(contents[0]),
                    status: String::from(contents[1]),
                    product: String::from(contents[2]),
                    model: String::from(contents[3]),
                    device: String::from(contents[4]),
                    transport_id: String::from(contents[5]),
                });
                continue;
            }
        }
        Ok(devices)
    }

    fn get_device(&mut self, serial_no: String) -> Result<Box<dyn DeviceService>, AdbError> {
        Ok(Box::new(DeviceClientImpl::new(&self.host, &self.port, &serial_no)))
    }

    fn track_devices(
        &mut self, on_change: fn(Vec<Device>), on_error: fn(AdbError),
    ) -> Result<JoinHandle<()>, AdbError> {
        let mut command = AdbHostTrackDeviceCommand::new(&self.host, &self.port);
        let mut tcp_stream = command.execute()?.tcp_stream;
        let handler = thread::spawn(move || loop {
            let length = match read_response_length(&mut tcp_stream) {
                Ok(length) => length,
                Err(error) => {
                    on_error(error);
                    return;
                }
            };

            let content = match read_response_content(&mut tcp_stream, length) {
                Ok(content) => content,
                Err(error) => {
                    on_error(error);
                    return;
                }
            };
            let mut devices = vec![];
            for line in content.lines() {
                let contents: Vec<&str> = line.trim().split_whitespace().collect();
                if contents.len() >= 2 {
                    devices.push(Device {
                        serial_no: String::from(contents[0]),
                        status: String::from(contents[1]),
                    });
                }
            }
            on_change(devices)
        });
        Ok(handler)
    }
}
