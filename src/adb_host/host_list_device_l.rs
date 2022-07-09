use crate::adb_host::SyncHostCommand;
use crate::adb_host::SyncHostResponse;
use crate::adb_host::{connect, exec_command, HostConnectionInfo};
use crate::error::adb::AdbError;

pub struct AdbHostListDeviceLCommand {
    pub connection_info: HostConnectionInfo,
}

impl SyncHostCommand for AdbHostListDeviceLCommand {
    fn execute(&mut self) -> Result<SyncHostResponse, AdbError> {
        let mut tcp_stream = connect(&self.connection_info)?;
        exec_command(&mut tcp_stream, String::from("host:devices-l"))
    }
}

impl AdbHostListDeviceLCommand {
    pub fn new(host: &String, port: &i32) -> AdbHostListDeviceLCommand {
        let connect_info = HostConnectionInfo::new(host, port);
        AdbHostListDeviceLCommand {
            connection_info: connect_info,
        }
    }
}
