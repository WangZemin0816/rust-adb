use crate::adb_host::{connect, exec_command, HostConnectionInfo, SyncHostCommand, SyncHostResponse};
use crate::error::adb::AdbError;

pub struct HostDevicePathCommand {
    pub serial_no: String,
    pub connection_info: HostConnectionInfo,
}

impl SyncHostCommand for HostDevicePathCommand {
    fn execute(&mut self) -> Result<SyncHostResponse, AdbError> {
        let mut tcp_stream = connect(&self.connection_info)?;
        let command = format!("host-serial:{}:get-devpath", self.serial_no);
        exec_command(&mut tcp_stream, command)
    }
}

#[cfg(test)]
mod tests {
    use crate::adb_host::host_device_path::HostDevicePathCommand;
    use crate::adb_host::HostConnectionInfo;
    use crate::adb_host::SyncHostCommand;

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let conn = HostConnectionInfo::new(&String::from("127.0.0.1"), &5037);
        let mut command = HostDevicePathCommand {
            serial_no: "emulator-5554".to_string(),
            connection_info: conn,
        };
        let resp = command.execute().unwrap();
        println!("{:?}", resp)
    }
}
