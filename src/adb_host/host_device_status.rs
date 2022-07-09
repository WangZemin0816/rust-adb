use crate::adb_host::{
    connect, exec_command, HostConnectionInfo, SyncHostCommand, SyncHostResponse,
};
use crate::error::adb::AdbError;

pub struct HostDeviceStatusCommand {
    pub serial_no: String,
    pub connection_info: HostConnectionInfo,
}

impl SyncHostCommand for HostDeviceStatusCommand {
    fn execute(&mut self) -> Result<SyncHostResponse, AdbError> {
        let mut tcp_stream = connect(&self.connection_info)?;
        let command = format!("host-serial:{}:get-state", self.serial_no);
        exec_command(&mut tcp_stream, command)
    }
}
