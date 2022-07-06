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

#[cfg(test)]
mod tests {
    use crate::adb_device::device_get_packages::DeviceGetPackagesCommand;

    use crate::adb_device::{DeviceConnectionInfo, SyncDeviceCommand, SyncDeviceProtocol};
    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let conn = DeviceConnectionInfo::new(&String::from("127.0.0.1"), &5037, &"emulator-5554".to_string());
        let mut command = DeviceGetPackagesCommand {
            params: "-a".to_string(),
            connection_info: conn,
        };
        let resp = command.execute().unwrap();
        match resp {
            | SyncDeviceProtocol::OKAY { content, .. } => {
                println!("devpath ok {}", content)
            }
            | SyncDeviceProtocol::FAIL { content, .. } => {
                println!("devpath failed {}", content)
            }
        }
    }
}
