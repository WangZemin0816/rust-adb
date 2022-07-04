use crate::adb_host::command::basic_command::exec_command;
use crate::adb_host::command::SyncHostCommand;
use crate::adb_host::protocol::SyncProtocol;
use crate::conn::connection::{connect, ConnectionInfo};
use crate::error::adb::AdbError;

pub struct AdbHostDisconnectCommand {
    pub host: String,
    pub port: i32,
    pub connection_info: ConnectionInfo,
}

impl SyncHostCommand for AdbHostDisconnectCommand {
    fn execute(&mut self) -> Result<SyncProtocol, AdbError> {
        let mut tcp_stream = connect(&self.connection_info)?;
        let command = format!(
            "host:disconnect:{}:{}",
            self.host.clone(),
            self.port.clone()
        );
        match exec_command(&mut tcp_stream, command) {
            Ok(_) => {
                let content = format!("{}:{}", self.host.clone(), self.port.clone());
                let length = content.len();
                Ok(SyncProtocol::OKAY { length, content })
            }
            Err(error) => Err(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::adb_host::command::host_disconnect::AdbHostDisconnectCommand;
    use crate::adb_host::command::SyncHostCommand;
    use crate::adb_host::protocol::SyncProtocol;
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
            SyncProtocol::OKAY { content, .. } => {
                println!("adb version {}", content)
            }
            SyncProtocol::FAIL { content, .. } => {
                println!("adb version {}", content)
            }
        }
    }
}
