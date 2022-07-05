use crate::adb_device::device_shell_sync::DeviceSyncShellCommand;
use crate::adb_device::{ DeviceConnectionInfo, SyncDeviceCommand, SyncDeviceProtocol};
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

#[cfg(test)]
mod tests {
    use crate::adb_device::device_get_features::DeviceGetFeaturesCommand;
    use crate::adb_device::{DeviceConnectionInfo, SyncDeviceCommand, SyncDeviceProtocol};


    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let conn = DeviceConnectionInfo::new(
            &String::from("127.0.0.1"),
            &5037,
            &String::from("emulator-5554"),
        );
        let mut command = DeviceGetFeaturesCommand {
            connection_info: conn,
        };
        let resp = command.execute().unwrap();
        match resp {
            SyncDeviceProtocol::OKAY { content, .. } => {
                println!("devpath ok {}", content)
            }
            SyncDeviceProtocol::FAIL { content, .. } => {
                println!("devpath failed {}", content)
            }
        }
    }
}
