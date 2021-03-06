use crate::adb_device::device_shell_sync::DeviceSyncShellCommand;
use crate::adb_device::{DeviceConnectionInfo, SyncDeviceCommand, SyncDeviceProtocol};
use crate::error::adb::AdbError;

pub struct DeviceGetFeaturesCommand {
    pub connection_info: DeviceConnectionInfo,
}

impl SyncDeviceCommand for DeviceGetFeaturesCommand {
    fn execute(&mut self) -> Result<SyncDeviceProtocol, AdbError> {
        let command = "shell:pm list features 2>/dev/null".to_string();
        DeviceSyncShellCommand::new(&self.connection_info, &command).execute()
    }
}

impl DeviceGetFeaturesCommand {
    pub fn new(host: &String, port: &i32, serial_no: &String) -> DeviceGetFeaturesCommand {
        DeviceGetFeaturesCommand {
            connection_info: DeviceConnectionInfo::new(host, port, serial_no),
        }
    }
}
