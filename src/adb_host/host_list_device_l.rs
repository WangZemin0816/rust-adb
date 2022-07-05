use crate::adb_host::{connect, HostConnectionInfo, exec_command};
use crate::adb_host::SyncHostCommand;
use crate::adb_host::SyncHostProtocol;
use crate::error::adb::AdbError;

pub struct AdbHostListDeviceLCommand {
    pub connection_info: HostConnectionInfo,
}

impl SyncHostCommand for AdbHostListDeviceLCommand {
    fn execute(&mut self) -> Result<SyncHostProtocol, AdbError> {
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

#[cfg(test)]
mod tests {
    use crate::adb_host::host_list_device_l::AdbHostListDeviceLCommand;
    use crate::adb_host::HostConnectionInfo;
    use crate::adb_host::SyncHostCommand;
    use crate::adb_host::SyncHostProtocol;

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("../../log4rs.yml", Default::default());
        let conn = HostConnectionInfo::new(&String::from("127.0.0.1"), &5037);
        let mut command = AdbHostListDeviceLCommand {
            connection_info: conn,
        };
        let resp = command.execute().unwrap();
        match resp {
            SyncHostProtocol::OKAY { content, .. } => {
                println!("client devices {}", content)
            }
            SyncHostProtocol::FAIL { content, .. } => {
                println!("client devices {}", content)
            }
        }
    }
}
