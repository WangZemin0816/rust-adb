use crate::adb_device::{device_connection, DeviceConnectionInfo, exec_device_command, SyncDeviceCommand, SyncDeviceProtocol};
use crate::adb_host::{HostConnectionInfo, exec_command, SyncHostCommand, SyncHostProtocol};

use crate::error::adb::AdbError;

pub struct DeviceGetPropertiesCommand {
    pub connection_info: DeviceConnectionInfo,
}

impl SyncDeviceCommand for DeviceGetPropertiesCommand {
    fn execute(&mut self) -> Result<SyncDeviceProtocol, AdbError> {
        let mut tcp_stream = device_connection(&self.connection_info)?;
        exec_device_command(&mut tcp_stream, String::from("shell:getprop"))
    }
}

#[cfg(test)]
mod tests {
    use crate::adb_device::device_get_properties::DeviceGetPropertiesCommand;
    use crate::adb_device::{DeviceConnectionInfo, SyncDeviceCommand, SyncDeviceProtocol};

    use crate::adb_host::HostConnectionInfo;
    use crate::adb_host::SyncHostCommand;
    use crate::adb_host::SyncHostProtocol;

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let conn = DeviceConnectionInfo::new(&String::from("127.0.0.1"), &5037,&"emulator-5554".to_string());
        let mut command = DeviceGetPropertiesCommand {
            connection_info: conn,
        };
        let resp = command.execute().unwrap();
        match resp {
            SyncDeviceProtocol::OKAY { content, .. } => {
                println!("devpath ok {}", content)
            }
        }
    }
}