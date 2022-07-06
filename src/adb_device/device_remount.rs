use crate::adb_device::{
    device_connection, exec_device_command, DeviceConnectionInfo, SyncDeviceCommand, SyncDeviceProtocol,
};
use crate::error::adb::AdbError;

pub struct DeviceRemountCommand {
    pub connection_info: DeviceConnectionInfo,
}

impl SyncDeviceCommand for DeviceRemountCommand {
    fn execute(&mut self) -> Result<SyncDeviceProtocol, AdbError> {
        let mut tcp_stream = device_connection(&self.connection_info)?;
        exec_device_command(&mut tcp_stream, "remount:".to_string())
    }
}

#[cfg(test)]
mod tests {

    use crate::adb_device::device_remount::DeviceRemountCommand;
    use crate::adb_device::{DeviceConnectionInfo, SyncDeviceCommand, SyncDeviceProtocol};

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let conn = DeviceConnectionInfo::new(&String::from("127.0.0.1"), &5037, &String::from("emulator-5554"));
        let mut command = DeviceRemountCommand {
            connection_info: conn,
        };
        let resp = command.execute().unwrap();
        match resp {
            | SyncDeviceProtocol::OKAY { content, .. } => {
                println!("remount ok {}", content)
            }
            | SyncDeviceProtocol::FAIL { content, .. } => {
                println!("remount failed {}", content)
            }
        }
    }
}
