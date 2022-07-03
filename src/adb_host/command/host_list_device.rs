use crate::adb_host::command::basic_sync::BasicSyncHostCommand;
use crate::adb_host::command::SyncHostCommand;
use crate::adb_host::protocol::SyncProtocol;
use crate::conn::connection::{connect, ConnectionInfo};
use crate::error::adb::AdbError;

pub struct AdbHostListDevicesCommand {
    pub connection_info: ConnectionInfo,
}

impl SyncHostCommand for AdbHostListDevicesCommand {
    fn execute(&mut self) -> Result<SyncProtocol, AdbError> {
        let mut tcp_stream = connect(&self.connection_info)?;
        BasicSyncHostCommand::exec_command(&mut tcp_stream,String::from("host:devices"))
    }
}

#[cfg(test)]
mod tests {
    use crate::adb_host::command::host_list_device::AdbHostListDevicesCommand;
    use crate::adb_host::command::SyncHostCommand;
    use crate::adb_host::protocol::SyncProtocol;
    use crate::conn::connection::ConnectionInfo;

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let conn = ConnectionInfo::new(&String::from("127.0.0.1"), 5037);
        let mut command = AdbHostListDevicesCommand {
            connection_info: conn,
        };
        let resp = command.execute().unwrap();
        match resp {
            SyncProtocol::OKAY { content, .. } => {
                println!("adb devices {}", content)
            }
            SyncProtocol::FAIL { content, .. } => {
                println!("adb devices {}", content)
            }
        }
    }
}