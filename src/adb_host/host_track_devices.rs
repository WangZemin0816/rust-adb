use crate::basic::connection::exec_command_sync;
use crate::basic::connection::{connect, ConnectionInfo};
use crate::basic::AsyncCommand;
use crate::basic::AsyncProtocol;
use crate::error::adb::AdbError;
use std::time::Duration;

pub struct AdbHostTrackDeviceCommand {
    pub connection_info: ConnectionInfo,
}

impl AsyncCommand for AdbHostTrackDeviceCommand {
    fn execute(&mut self) -> Result<AsyncProtocol, AdbError> {
        let tcp_stream = connect(&self.connection_info)?;
        let command = format!("host:track-devices");
        exec_command_sync(tcp_stream, command)
    }
}

impl AdbHostTrackDeviceCommand {
    pub fn new(host: &String, port: &i32) -> AdbHostTrackDeviceCommand {
        let connect_info = ConnectionInfo {
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
    use crate::basic::connection::ConnectionInfo;
    use crate::basic::AsyncCommand;
    use crate::basic::AsyncProtocol;

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("../../log4rs.yml", Default::default());
        let conn = ConnectionInfo::new(&String::from("127.0.0.1"), &5037);
        let mut command = AdbHostTrackDeviceCommand {
            connection_info: conn,
        };
        let resp = command.execute().unwrap();
        match resp {
            AsyncProtocol::OKAY { .. } => {
                println!("client track device ok")
            }
            AsyncProtocol::FAIL { content, length: _ } => {
                println!("client track device failed {}", content)
            }
        }
    }
}
