use crate::adb_device::{
    device_connection, exec_device_command_sync, AsyncDeviceCommand,
    AsyncDeviceProtocol, DeviceConnectionInfo,
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
