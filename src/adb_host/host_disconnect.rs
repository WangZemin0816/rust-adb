use crate::adb_host::AsyncHostCommand;
use crate::adb_host::AsyncHostResponse;
use crate::adb_host::{connect, exec_command_sync, HostConnectionInfo};
use crate::error::adb::AdbError;

pub struct AdbHostDisconnectCommand {
    pub host: String,
    pub port: i32,
    pub connection_info: HostConnectionInfo,
}

impl AsyncHostCommand for AdbHostDisconnectCommand {
    fn execute(&mut self) -> Result<AsyncHostResponse, AdbError> {
        let tcp_stream = connect(&self.connection_info)?;
        let command = format!("host:disconnect:{}:{}", self.host, self.port);
        exec_command_sync(tcp_stream, command)
    }
}

impl AdbHostDisconnectCommand {
    pub fn new(
        host: &String, port: &i32, dis_host: &String, dis_port: &i32,
    ) -> AdbHostDisconnectCommand {
        let connect_info = HostConnectionInfo::new(host, port);
        AdbHostDisconnectCommand {
            connection_info: connect_info,
            host: dis_host.clone(),
            port: dis_port.clone(),
        }
    }
}
