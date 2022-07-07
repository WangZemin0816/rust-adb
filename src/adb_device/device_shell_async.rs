use crate::adb_device::{
    device_connection, exec_device_command_sync, AsyncDeviceCommand, AsyncDeviceProtocol,
    DeviceConnectionInfo,
};
use crate::error::adb::AdbError;
use std::time::Duration;

pub struct DeviceAsyncShellCommand {
    pub shell: String,
    pub connection_info: DeviceConnectionInfo,
}

impl AsyncDeviceCommand for DeviceAsyncShellCommand {
    fn execute(&mut self) -> Result<AsyncDeviceProtocol, AdbError> {
        let tcp_stream = device_connection(&self.connection_info)?;
        exec_device_command_sync(tcp_stream, self.shell.clone())
    }
}

impl DeviceAsyncShellCommand {
    pub fn new(connection_info: &DeviceConnectionInfo, shell: &String) -> DeviceAsyncShellCommand {
        DeviceAsyncShellCommand {
            connection_info: connection_info.clone(),
            shell: shell.clone(),
        }
    }

    pub fn new0(
        host: &String, port: &i32, serial_no: &String, shell: &String,
    ) -> DeviceAsyncShellCommand {
        DeviceAsyncShellCommand {
            connection_info: DeviceConnectionInfo {
                host: host.clone(),
                port: port.clone(),
                serial_no: serial_no.clone(),
                read_timeout: None,
                write_timeout: Option::from(Duration::from_millis(1000)),
            },
            shell: shell.clone(),
        }
    }
}
