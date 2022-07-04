use crate::basic::AsyncCommand;
use crate::basic::connection::exec_command_sync;
use crate::basic::connection::{connect, ConnectionInfo};
use crate::basic::AsyncProtocol;
use crate::error::adb::AdbError;

pub struct AdbHostTransportCommand {
    pub serial_no: String,
    pub connection_info: ConnectionInfo,
}

impl AsyncCommand for AdbHostTransportCommand {
    fn execute(&mut self) -> Result<AsyncProtocol, AdbError> {
        let tcp_stream = connect(&self.connection_info)?;
        let command = format!("host:transport:{}", self.serial_no.clone());
        exec_command_sync(tcp_stream, command)
    }
}

impl AdbHostTransportCommand {
    pub fn new(host: &String, port: &i32, serial_no: &String) -> AdbHostTransportCommand {
        let connection_info = ConnectionInfo::new(host, port);
        AdbHostTransportCommand {
            serial_no: serial_no.clone(),
            connection_info,
        }
    }
    pub fn new0(connection_info:&ConnectionInfo, serial_no: &String) -> AdbHostTransportCommand {
        AdbHostTransportCommand {
            connection_info:connection_info.clone(),
            serial_no: serial_no.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::basic::AsyncCommand;
    use crate::adb_host::host_transport::AdbHostTransportCommand;
    use crate::basic::connection::ConnectionInfo;
    use crate::basic::AsyncProtocol;

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("../../log4rs.yml", Default::default());
        let conn = ConnectionInfo::new(&String::from("127.0.0.1"), &5037);
        let mut command = AdbHostTransportCommand {
            serial_no: "emulator-5554".to_string(),
            connection_info: conn,
        };
        let resp = command.execute().unwrap();
        match resp {
            AsyncProtocol::OKAY { .. } => {
                println!("ok")
            }
            AsyncProtocol::FAIL { content, length: _ } => {
                println!("client transport FAIL {}", content)
            }
        }
    }
}
