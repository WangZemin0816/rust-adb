use crate::adb_host::AsyncHostCommand;
use crate::adb_host::AsyncHostResponse;
use crate::adb_host::{connect, exec_command_sync, HostConnectionInfo};
use crate::error::adb::AdbError;

pub struct AdbHostTransportCommand {
    pub serial_no: String,
    pub connection_info: HostConnectionInfo,
}

impl AsyncHostCommand for AdbHostTransportCommand {
    fn execute(&mut self) -> Result<AsyncHostResponse, AdbError> {
        let tcp_stream = connect(&self.connection_info)?;
        let command = format!("host:transport:{}", self.serial_no.clone());
        exec_command_sync(tcp_stream, command)
    }
}

impl AdbHostTransportCommand {
    pub fn new(
        connection_info: &HostConnectionInfo, serial_no: &String,
    ) -> AdbHostTransportCommand {
        AdbHostTransportCommand {
            connection_info: connection_info.clone(),
            serial_no: serial_no.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::adb_host::host_transport::AdbHostTransportCommand;
    use crate::adb_host::AsyncHostCommand;

    use crate::adb_host::HostConnectionInfo;

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("../../log4rs.yml", Default::default());
        let conn = HostConnectionInfo::new(&String::from("127.0.0.1"), &5037);
        let mut command = AdbHostTransportCommand {
            serial_no: "emulator-5554".to_string(),
            connection_info: conn,
        };
        let _resp = command.execute().unwrap();
    }
}
