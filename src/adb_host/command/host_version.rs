use crate::adb_host::command::basic_sync::BasicSyncHostCommand;
use crate::adb_host::command::SyncHostCommand;
use crate::adb_host::protocol::SyncProtocol;
use crate::conn::connection::{connect, ConnectionInfo};
use crate::error::adb::AdbError;

pub struct AdbHostVersionCommand {
    pub connection_info: ConnectionInfo,
}

impl SyncHostCommand for AdbHostVersionCommand {
    fn execute(&mut self) -> Result<SyncProtocol, AdbError> {
        let mut tcp_stream = connect(&self.connection_info)?;
        BasicSyncHostCommand::exec_command(&mut tcp_stream, String::from("host:version"))
    }
}

#[cfg(test)]
mod tests {
    use crate::adb_host::command::host_version::AdbHostVersionCommand;
    use crate::adb_host::command::SyncHostCommand;
    use crate::adb_host::protocol::SyncProtocol;
    use crate::conn::connection::ConnectionInfo;

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let conn = ConnectionInfo::new(&String::from("127.0.0.1"), 5037);
        let mut command = AdbHostVersionCommand {
            connection_info: conn,
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