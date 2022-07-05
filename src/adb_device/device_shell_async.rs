use crate::adb_device::{
    device_connection, exec_device_command, DeviceConnectionInfo, SyncDeviceCommand,
    SyncDeviceProtocol,
};
use crate::error::adb::AdbError;

pub struct DeviceShellCommand {
    pub command: String,
    pub connection_info: DeviceConnectionInfo,
}

impl SyncDeviceCommand for DeviceShellCommand {
    fn execute(&mut self) -> Result<SyncDeviceProtocol, AdbError> {
        let mut tcp_stream = device_connection(&self.connection_info)?;
        exec_device_command(&mut tcp_stream, "pm list features 2>/dev/null".to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::adb_device::device_get_features::DeviceGetFeaturesCommand;

    use crate::adb_device::{DeviceConnectionInfo, SyncDeviceCommand, SyncDeviceProtocol};

    use crate::adb_host::SyncHostCommand;

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
