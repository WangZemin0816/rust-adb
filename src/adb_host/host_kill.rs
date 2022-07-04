use crate::adb_host::{connect, ConnectionInfo, exec_command_sync};
use crate::adb_host::AsyncCommand;
use crate::adb_host::AsyncProtocol;
use crate::error::adb::AdbError;

pub struct AdbHostKillCommand {
    pub connection_info: ConnectionInfo,
}

impl AsyncCommand for AdbHostKillCommand {
    fn execute(&mut self) -> Result<AsyncProtocol, AdbError> {
        let tcp_stream = connect(&self.connection_info)?;
        exec_command_sync(tcp_stream, String::from("host:kill"))
    }
}

impl AdbHostKillCommand {
    pub fn new(host: &String, port: &i32) -> AdbHostKillCommand {
        let connect_info = ConnectionInfo::new(host, port);
        AdbHostKillCommand {
            connection_info: connect_info,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::adb_host::host_kill::AdbHostKillCommand;
    use crate::adb_host::ConnectionInfo;
    use crate::adb_host::AsyncCommand;
    use crate::adb_host::AsyncProtocol;

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("../../log4rs.yml", Default::default());
        let conn = ConnectionInfo::new(&String::from("127.0.0.1"), &5037);
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
