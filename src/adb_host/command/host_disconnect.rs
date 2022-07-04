use crate::adb_host::command::basic_command::{exec_command, exec_command_sync};
use crate::adb_host::command::{AsyncHostCommand, SyncHostCommand};
use crate::adb_host::protocol::{AsyncProtocol, SyncProtocol};
use crate::conn::connection::{connect, ConnectionInfo};
use crate::error::adb::AdbError;

pub struct AdbHostDisconnectCommand {
    pub host: String,
    pub port: i32,
    pub connection_info: ConnectionInfo,
}

impl AsyncHostCommand for AdbHostDisconnectCommand {
    fn execute(&mut self) -> Result<AsyncProtocol, AdbError> {
        let mut tcp_stream = connect(&self.connection_info)?;
        let command = format!(
            "host:disconnect:{}:{}",
            self.host.clone(),
            self.port.clone()
        );
         exec_command_sync(tcp_stream, command)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use crate::adb_host::command::host_disconnect::AdbHostDisconnectCommand;
    use crate::adb_host::command::AsyncHostCommand;
    use crate::adb_host::protocol::{AsyncProtocol, SyncProtocol};
    use crate::conn::connection::ConnectionInfo;

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let conn = ConnectionInfo::new(&String::from("127.0.0.1"), 5037);
        let mut command = AdbHostDisconnectCommand {
            connection_info: conn,
            host: String::from("127.0.0.1"),
            port: 5037,
        };
        let resp = command.execute().unwrap();
        match resp {
            AsyncProtocol::OKAY {  .. } => {
                println!("adb disconnect OKAY")
            }
            AsyncProtocol::FAIL { content,length } => {
                println!("adb disconnect FAIL {}",content)
            }
        }
    }
}
