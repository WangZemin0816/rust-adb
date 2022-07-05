use crate::adb_host::AsyncHostCommand;
use crate::adb_host::AsyncHostResponse;
use crate::adb_host::{connect, exec_command_sync, HostConnectionInfo};
use crate::error::adb::AdbError;

pub struct AdbHostKillCommand {
    pub connection_info: HostConnectionInfo,
}

impl AsyncHostCommand for AdbHostKillCommand {
    fn execute(&mut self) -> Result<AsyncHostResponse, AdbError> {
        let tcp_stream = connect(&self.connection_info)?;
        exec_command_sync(tcp_stream, String::from("host:kill"))
    }
}

impl AdbHostKillCommand {
    pub fn new(host: &String, port: &i32) -> AdbHostKillCommand {
        let connect_info = HostConnectionInfo::new(host, port);
        AdbHostKillCommand {
            connection_info: connect_info,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::adb_host::host_kill::AdbHostKillCommand;
    use crate::adb_host::AsyncHostCommand;

    use crate::adb_host::HostConnectionInfo;

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("../../log4rs.yml", Default::default());
        let conn = HostConnectionInfo::new(&String::from("127.0.0.1"), &5037);
        let mut command = AdbHostKillCommand {
            connection_info: conn,
        };
        let _resp = command.execute().unwrap();
    }
}
