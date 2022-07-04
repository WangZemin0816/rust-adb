use crate::basic::AsyncCommand;
use crate::basic::connection::exec_command_sync;
use crate::basic::connection::{connect, ConnectionInfo};
use crate::basic::AsyncProtocol;
use crate::error::adb::AdbError;

pub struct AdbHostDisconnectCommand {
    pub host: String,
    pub port: i32,
    pub connection_info: ConnectionInfo,
}

impl AsyncCommand for AdbHostDisconnectCommand {
    fn execute(&mut self) -> Result<AsyncProtocol, AdbError> {
        let tcp_stream = connect(&self.connection_info)?;
        let command = format!("host:disconnect:{}:{}", self.host, self.port);
        exec_command_sync(tcp_stream, command)
    }
}

impl AdbHostDisconnectCommand {
    pub fn new(
        host: &String,
        port: &i32,
        dis_host: &String,
        dis_port: &i32,
    ) -> AdbHostDisconnectCommand {
        let connect_info = ConnectionInfo::new(host, port);
        AdbHostDisconnectCommand {
            connection_info: connect_info,
            host: dis_host.clone(),
            port: dis_port.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::basic::AsyncCommand;
    use crate::adb_host::host_disconnect::AdbHostDisconnectCommand;
    use crate::basic::connection::ConnectionInfo;
    use crate::basic::AsyncProtocol;

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("../../log4rs.yml", Default::default());
        let conn = ConnectionInfo::new(&String::from("127.0.0.1"), &5037);
        let mut command = AdbHostDisconnectCommand {
            connection_info: conn,
            host: String::from("127.0.0.1"),
            port: 5037,
        };
        let resp = command.execute().unwrap();
        match resp {
            AsyncProtocol::OKAY { .. } => {
                println!("client disconnect OKAY")
            }
            AsyncProtocol::FAIL { content, length: _ } => {
                println!("client disconnect FAIL {}", content)
            }
        }
    }
}
