use crate::adb_host::AsyncHostCommand;
use crate::adb_host::AsyncHostResponse;
use crate::adb_host::{connect, exec_command_sync, HostConnectionInfo};
use crate::error::adb::AdbError;
use std::time::Duration;

pub struct AdbHostTrackDeviceCommand {
    pub connection_info: HostConnectionInfo,
}

impl AsyncHostCommand for AdbHostTrackDeviceCommand {
    fn execute(&mut self) -> Result<AsyncHostResponse, AdbError> {
        let tcp_stream = connect(&self.connection_info)?;
        let command = format!("host:track-devices");
        exec_command_sync(tcp_stream, command)
    }
}

impl AdbHostTrackDeviceCommand {
    pub fn new(host: &String, port: &i32) -> AdbHostTrackDeviceCommand {
        let connect_info = HostConnectionInfo {
            host: host.clone(),
            port: port.clone(),
            read_timeout: None,
            write_timeout: Option::from(Duration::from_millis(1000)),
        };
        AdbHostTrackDeviceCommand {
            connection_info: connect_info,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::adb_host::host_track_devices::AdbHostTrackDeviceCommand;
    use crate::adb_host::AsyncHostCommand;
    
    use crate::adb_host::HostConnectionInfo;

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("../../log4rs.yml", Default::default());
        let conn = HostConnectionInfo::new(&String::from("127.0.0.1"), &5037);
        let mut command = AdbHostTrackDeviceCommand {
            connection_info: conn,
        };
        let _resp = command.execute().unwrap();
    }
}
