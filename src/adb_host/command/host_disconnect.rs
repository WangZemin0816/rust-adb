use crate::adb_host::command::AsyncHostCommand;
use crate::conn::connection::exec_command_sync;
use crate::conn::connection::{connect, ConnectionInfo};
use crate::conn::protocol::AsyncProtocol;
use crate::error::adb::AdbError;

pub struct AdbHostDisconnectCommand {
    pub host: String,
    pub port: i32,
    pub connection_info: ConnectionInfo,
}

impl AsyncHostCommand for AdbHostDisconnectCommand {
    fn execute(&mut self) -> Result<AsyncProtocol, AdbError> {
        let tcp_stream = connect(&self.connection_info)?;
        let command = format!(
            "host:disconnect:{}:{}",
            self.host.clone(),
            self.port.clone()
        );
        exec_command_sync(tcp_stream, command)
    }
}

impl AdbHostDisconnectCommand {
    fn new(host: String, port: i32, dis_host: String, dis_port: i32) -> AdbHostDisconnectCommand {
        let connect_info = ConnectionInfo::new(&host, port);
        AdbHostDisconnectCommand {
            connection_info: connect_info,
            host: dis_host,
            port: dis_port,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::adb_host::command::host_disconnect::AdbHostDisconnectCommand;
    use crate::adb_host::command::AsyncHostCommand;
    use crate::conn::connection::ConnectionInfo;
    use crate::conn::protocol::AsyncProtocol;

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let conn = ConnectionInfo::new(&String::from("127.0.0.1"), 5037);
        let mut command = AdbHostDisconnectCommand {
            connection_info: conn,
            host: String::from("127.0.0.1"),
            port: 5037,
        };
        let resp = command.execute().unwrap();
        match resp {
            AsyncProtocol::OKAY { .. } => {
                println!("adb disconnect OKAY")
            }
            AsyncProtocol::FAIL { content, length: _ } => {
                println!("adb disconnect FAIL {}", content)
            }
        }
    }
}
