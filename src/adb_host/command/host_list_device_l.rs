use crate::adb_host::command::SyncHostCommand;
use crate::conn::connection::exec_command;
use crate::conn::connection::{connect, ConnectionInfo};
use crate::conn::protocol::SyncProtocol;
use crate::error::adb::AdbError;

pub struct AdbHostListDeviceLCommand {
    pub connection_info: ConnectionInfo,
}

impl SyncHostCommand for AdbHostListDeviceLCommand {
    fn execute(&mut self) -> Result<SyncProtocol, AdbError> {
        let mut tcp_stream = connect(&self.connection_info)?;
        exec_command(&mut tcp_stream, String::from("host:devices-l"))
    }
}

impl AdbHostListDeviceLCommand {
    pub fn new(host: &String, port: &i32) -> AdbHostListDeviceLCommand {
        let connect_info = ConnectionInfo::new(host, port);
        AdbHostListDeviceLCommand {
            connection_info: connect_info,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::adb_host::command::host_list_device_l::AdbHostListDeviceLCommand;
    use crate::adb_host::command::SyncHostCommand;
    use crate::conn::connection::ConnectionInfo;
    use crate::conn::protocol::SyncProtocol;

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let conn = ConnectionInfo::new(&String::from("127.0.0.1"), &5037);
        let mut command = AdbHostListDeviceLCommand {
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
