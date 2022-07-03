use crate::adb_host::command::basic_sync::BasicSyncHostCommand;
use crate::adb_host::command::SyncHostCommand;
use crate::adb_host::protocol::SyncProtocol;
use crate::conn::connection::{connect, ConnectionInfo};
use crate::error::adb::AdbError;

pub struct AdbHostKillCommand {
    pub connection_info: ConnectionInfo,
}

impl SyncHostCommand for AdbHostKillCommand {
    fn execute(&mut self) -> Result<SyncProtocol, AdbError> {
        let tcp_stream = connect(&self.connection_info)?;
        let adb_command = BasicSyncHostCommand { tcp_stream };
        match adb_command.exec_command(String::from("host:kill")) {
            Ok(resp) => Ok(resp),
            Err(error) => match error {
                AdbError::TcpReadError { .. } => Ok(SyncProtocol::OKAY {
                    length: 0,
                    content: String::from(""),
                }),
                _ => Err(error),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::adb_host::command::host_kill::AdbHostKillCommand;
    use crate::adb_host::command::SyncHostCommand;
    use crate::adb_host::protocol::SyncProtocol;
    use crate::conn::connection::ConnectionInfo;

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let conn = ConnectionInfo::new(&String::from("127.0.0.1"), 5037);
        let mut command = AdbHostKillCommand {
            connection_info: conn,
        };
        let resp = command.execute().unwrap();
        match resp {
            SyncProtocol::OKAY { content, .. } => {
                println!("adb devices {}", content)
            }
            SyncProtocol::FAIL { content, .. } => {
                println!("adb devices {}", content)
            }
        }
    }
}
