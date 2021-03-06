use crate::adb_device::device_shell_sync::DeviceSyncShellCommand;
use crate::adb_device::{DeviceConnectionInfo, SyncDeviceCommand, SyncDeviceProtocol};
use crate::error::adb::AdbError;

pub struct DeviceGetPackagesCommand {
    pub params: String,
    pub connection_info: DeviceConnectionInfo,
}

impl SyncDeviceCommand for DeviceGetPackagesCommand {
    fn execute(&mut self) -> Result<SyncDeviceProtocol, AdbError> {
        let command = format!("shell:pm list packages {} 2>/dev/null", self.params);
        DeviceSyncShellCommand::new(&self.connection_info, &command).execute()
    }
}

impl DeviceGetPackagesCommand {
    pub fn new(
        host: &String, port: &i32, serial_no: &String, params: &String,
    ) -> DeviceGetPackagesCommand {
        DeviceGetPackagesCommand {
            params: params.clone(),
            connection_info: DeviceConnectionInfo::new(host, port, serial_no),
        }
    }
}
