use std::time::Duration;
use crate::conn::connection::exec_command_sync;
use crate::adb_host::command::AsyncHostCommand;
use crate::conn::protocol::AsyncProtocol;
use crate::conn::connection::{connect, ConnectionInfo};
use crate::error::adb::AdbError;

pub struct AdbHostTrackDeviceCommand {
    pub connection_info: ConnectionInfo,
}

impl AsyncHostCommand for AdbHostTrackDeviceCommand {
    fn execute(&mut self) -> Result<AsyncProtocol, AdbError> {
        let tcp_stream = connect(&self.connection_info)?;
        let command = format!("host:track-devices");
        exec_command_sync(tcp_stream, command)
    }
}

impl AdbHostTrackDeviceCommand {
    fn new(host: String, port: i32) -> AdbHostTrackDeviceCommand {
        let connect_info = ConnectionInfo{
            host,
            port,
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

    use crate::adb_host::command::host_track_devices::AdbHostTrackDeviceCommand;
    use crate::adb_host::command::AsyncHostCommand;
    use crate::conn::protocol::AsyncProtocol;
    use crate::conn::connection::ConnectionInfo;

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let conn = ConnectionInfo::new(&String::from("127.0.0.1"), 5037);
        let mut command = AdbHostTrackDeviceCommand {
            connection_info: conn,
        };
        let resp = command.execute().unwrap();
        match resp {
            AsyncProtocol::OKAY { .. } => {
                println!("adb track device ok")
            }
            AsyncProtocol::FAIL { content, length: _ } => {
                println!("adb track device failed {}", content)
            }
        }
    }
}
