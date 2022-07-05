use crate::adb_device::{
    device_connection, exec_device_command, DeviceConnectionInfo, SyncDeviceCommand,
    SyncDeviceProtocol,
};
use crate::error::adb::AdbError;

pub struct DeviceRebootCommand {
    pub connection_info: DeviceConnectionInfo,
}

impl SyncDeviceCommand for DeviceRebootCommand {
    fn execute(&mut self) -> Result<SyncDeviceProtocol, AdbError> {
        let mut tcp_stream = device_connection(&self.connection_info)?;
        match exec_device_command(&mut tcp_stream, "reboot:".to_string()) {
            Ok(response) => Ok(response),
            Err(error) => match error {
                AdbError::TcpReadError { .. } => Ok(SyncDeviceProtocol::OKAY {
                    content: "".to_string(),
                    length: 0,
                }),
                _ => Err(error),
            },
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::adb_device::device_reboot::DeviceRebootCommand;

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
        let mut command = DeviceRebootCommand {
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
