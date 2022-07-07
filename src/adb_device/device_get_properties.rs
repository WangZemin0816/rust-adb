use crate::adb_device::device_shell_sync::DeviceSyncShellCommand;
use crate::adb_device::{DeviceConnectionInfo, SyncDeviceCommand, SyncDeviceProtocol};

use crate::error::adb::AdbError;

pub struct DeviceGetPropertiesCommand {
    pub params: String,
    pub connection_info: DeviceConnectionInfo,
}

impl SyncDeviceCommand for DeviceGetPropertiesCommand {
    fn execute(&mut self) -> Result<SyncDeviceProtocol, AdbError> {
        let command = format!("shell:getprop {}", self.params);
        DeviceSyncShellCommand::new(&self.connection_info, &command).execute()
    }
}
