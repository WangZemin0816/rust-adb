use crate::adb_host::command::basic_command::{exec_command, exec_command_sync};
use crate::adb_host::command::{AsyncHostCommand, SyncHostCommand};
use crate::adb_host::protocol::{AsyncProtocol, SyncProtocol};
use crate::conn::connection::{connect, ConnectionInfo};
use crate::error::adb::AdbError;

pub struct AdbHostKillCommand {
    pub connection_info: ConnectionInfo,
}

impl AsyncHostCommand for AdbHostKillCommand {
    fn execute(&mut self) -> Result<AsyncProtocol, AdbError> {
        let mut tcp_stream = connect(&self.connection_info)?;
        exec_command_sync(tcp_stream, String::from("host:kill"))
    }
}

#[cfg(test)]
mod tests {
    use crate::adb_host::command::host_kill::AdbHostKillCommand;
    use crate::adb_host::command::{AsyncHostCommand, SyncHostCommand};
    use crate::adb_host::protocol::{AsyncProtocol, SyncProtocol};
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
            AsyncProtocol::OKAY { .. } => {
                println!("host kill ok")
            }
            AsyncProtocol::FAIL { .. } => {
                println!("host kill fail")
            }
        }
    }
}
