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
