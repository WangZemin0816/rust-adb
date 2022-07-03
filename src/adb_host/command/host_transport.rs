use crate::adb_host::command::basic_sync::BasicSyncHostCommand;
use crate::adb_host::command::SyncHostCommand;
use crate::adb_host::protocol::SyncProtocol;
use crate::conn::connection::{connect, ConnectionInfo};
use crate::error::adb::AdbError;

pub struct AdbHostTransportCommand {
    pub serial_no: String,
    pub connection_info: ConnectionInfo,
}

impl SyncHostCommand for AdbHostTransportCommand {
    fn execute(&mut self) -> Result<SyncProtocol, AdbError> {
        let tcp_stream = connect(&self.connection_info)?;
        let adb_command = BasicSyncHostCommand { tcp_stream };
        let command = format!("host:transport:{}", self.serial_no.clone());
        adb_command.exec_command(command)
    }
}
