use crate::adb_host::command::basic_sync::BasicSyncHostCommand;
use crate::adb_host::command::{SyncHostCommand, SyncTransportCommand};
use crate::adb_host::protocol::SyncProtocol;
use crate::conn::connection::{connect, ConnectionInfo};
use crate::error::adb::AdbError;
use std::net::TcpStream;

pub struct AdbHostTransportCommand {
    pub serial_no: String,
    pub connection_info: ConnectionInfo,
}

impl SyncTransportCommand for AdbHostTransportCommand {
    fn execute(&mut self) -> Result<TcpStream, AdbError> {
        let mut tcp_stream = connect(&self.connection_info)?;
        let command = format!("host:transport:{}", self.serial_no.clone());
        BasicSyncHostCommand::exec_command(&mut tcp_stream,command)?;
        Ok(tcp_stream)
    }
}
