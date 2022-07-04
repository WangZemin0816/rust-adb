use crate::adb_device::device_connection;

use crate::basic::connection::{exec_command, ConnectionInfo};
use crate::basic::{SyncCommand, SyncProtocol};
use crate::error::adb::AdbError;

pub struct DeviceGetPathCommand {
    pub serial_no: String,
    pub connection_info: ConnectionInfo,
}

impl SyncCommand for DeviceGetPathCommand {
    fn execute(&mut self) -> Result<SyncProtocol, AdbError> {
        let mut tcp_stream = device_connection(&self.connection_info, &self.serial_no)?;
        let command = format!("host-serial:{}:get-devpath", self.serial_no);
        exec_command(&mut tcp_stream, command)
    }
}

#[cfg(test)]
mod tests {
    use crate::adb_device::device_get_path::DeviceGetPathCommand;

    use crate::basic::connection::ConnectionInfo;
    use crate::basic::SyncCommand;
    use crate::basic::SyncProtocol;

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let conn = ConnectionInfo::new(&String::from("127.0.0.1"), &5037);
        let mut command = DeviceGetPathCommand {
            serial_no: "emulator-5554".to_string(),
            connection_info: conn,
        };
        let resp = command.execute().unwrap();
        match resp {
            SyncProtocol::OKAY { content, .. } => {
                println!("devpath ok {}", content)
            }
            SyncProtocol::FAIL { content, .. } => {
                println!("devpath failed {}", content)
            }
        }
    }
}
