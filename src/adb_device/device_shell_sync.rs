use crate::adb_device::{
    device_connection, exec_device_command, DeviceConnectionInfo, SyncDeviceCommand, SyncDeviceProtocol,
};
use crate::error::adb::AdbError;

pub struct DeviceSyncShellCommand {
    pub shell: String,
    pub connection_info: DeviceConnectionInfo,
}

impl SyncDeviceCommand for DeviceSyncShellCommand {
    fn execute(&mut self) -> Result<SyncDeviceProtocol, AdbError> {
        let mut tcp_stream = device_connection(&self.connection_info)?;
        exec_device_command(&mut tcp_stream, self.shell.clone())
    }
}

impl DeviceSyncShellCommand {
    pub fn new(connection_info: &DeviceConnectionInfo, shell: &String) -> DeviceSyncShellCommand {
        DeviceSyncShellCommand {
            connection_info: connection_info.clone(),
            shell: shell.clone(),
        }
    }

    pub fn new0(host: &String, port: &i32, serial_no: &String, shell: &String) -> DeviceSyncShellCommand {
        DeviceSyncShellCommand {
            connection_info: DeviceConnectionInfo::new(host, port, serial_no),
            shell: shell.clone(),
        }
    }
}
