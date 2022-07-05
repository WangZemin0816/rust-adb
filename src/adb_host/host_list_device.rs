use crate::adb_host::SyncHostCommand;
use crate::adb_host::SyncHostResponse;
use crate::adb_host::{connect, exec_command, HostConnectionInfo};
use crate::error::adb::AdbError;

pub struct AdbHostListDevicesCommand {
    pub connection_info: HostConnectionInfo,
}

impl SyncHostCommand for AdbHostListDevicesCommand {
    fn execute(&mut self) -> Result<SyncHostResponse, AdbError> {
        let mut tcp_stream = connect(&self.connection_info)?;
        exec_command(&mut tcp_stream, String::from("host:devices"))
    }
}

impl AdbHostListDevicesCommand {
    pub fn new(host: &String, port: &i32) -> AdbHostListDevicesCommand {
        let connect_info = HostConnectionInfo::new(host, port);
        AdbHostListDevicesCommand {
            connection_info: connect_info,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::adb_host::host_list_device::AdbHostListDevicesCommand;
    use crate::adb_host::HostConnectionInfo;
    use crate::adb_host::SyncHostCommand;
    

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("../../log4rs.yml", Default::default());
        let conn = HostConnectionInfo::new(&String::from("127.0.0.1"), &5037);
        let mut command = AdbHostListDevicesCommand {
            connection_info: conn,
        };
        let resp = command.execute().unwrap();
        println!("{:?}",resp)
    }
}
