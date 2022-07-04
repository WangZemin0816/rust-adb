use crate::basic::connection::exec_command;
use crate::basic::connection::{connect, ConnectionInfo};
use crate::basic::SyncCommand;
use crate::basic::SyncProtocol;
use crate::error::adb::AdbError;

pub struct AdbHostListDevicesCommand {
    pub connection_info: ConnectionInfo,
}

impl SyncCommand for AdbHostListDevicesCommand {
    fn execute(&mut self) -> Result<SyncProtocol, AdbError> {
        let mut tcp_stream = connect(&self.connection_info)?;
        exec_command(&mut tcp_stream, String::from("host:devices"))
    }
}

impl AdbHostListDevicesCommand {
    pub fn new(host: &String, port: &i32) -> AdbHostListDevicesCommand {
        let connect_info = ConnectionInfo::new(host, port);
        AdbHostListDevicesCommand {
            connection_info: connect_info,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::adb_host::host_list_device::AdbHostListDevicesCommand;
    use crate::basic::connection::ConnectionInfo;
    use crate::basic::SyncCommand;
    use crate::basic::SyncProtocol;

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("../../log4rs.yml", Default::default());
        let conn = ConnectionInfo::new(&String::from("127.0.0.1"), &5037);
        let mut command = AdbHostListDevicesCommand {
            connection_info: conn,
        };
        let resp = command.execute().unwrap();
        match resp {
            SyncProtocol::OKAY { content, .. } => {
                println!("client devices {}", content)
            }
            SyncProtocol::FAIL { content, .. } => {
                println!("client devices {}", content)
            }
        }
    }
}
