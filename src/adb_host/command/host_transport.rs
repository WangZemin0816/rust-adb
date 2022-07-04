use crate::adb_host::command::basic_command::{exec_command_sync};
use crate::adb_host::command::{AsyncHostCommand};
use crate::adb_host::protocol::{AsyncProtocol};
use crate::conn::connection::{connect, ConnectionInfo};
use crate::error::adb::AdbError;


pub struct AdbHostTransportCommand {
    pub serial_no: String,
    pub connection_info: ConnectionInfo,
}

impl AsyncHostCommand for AdbHostTransportCommand {
    fn execute(&mut self) -> Result<AsyncProtocol, AdbError> {
        let tcp_stream = connect(&self.connection_info)?;
        let command = format!("host:transport:{}", self.serial_no.clone());
        exec_command_sync(tcp_stream, command)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use crate::adb_host::command::host_transport::AdbHostTransportCommand;
    use crate::adb_host::command::{AsyncHostCommand};
    use crate::adb_host::protocol::{AsyncProtocol};
    use crate::conn::connection::ConnectionInfo;

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let conn = ConnectionInfo::new(&String::from("127.0.0.1"), 5037);
        let mut command = AdbHostTransportCommand {
            serial_no: "emulator-5554".to_string(),
            connection_info: conn,
        };
        let resp = command.execute().unwrap();
        match resp {
            AsyncProtocol::OKAY { .. } => {println!("ok")}
            AsyncProtocol::FAIL { content,length } => {
                println!("adb transport FAIL {}",content)
            }
        }
    }
}
