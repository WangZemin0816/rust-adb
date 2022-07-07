use crate::adb_device::{
    device_connection, exec_device_command_sync, AsyncDeviceCommand, AsyncDeviceProtocol, DeviceConnectionInfo,
};
use crate::error::adb::AdbError;

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
}
