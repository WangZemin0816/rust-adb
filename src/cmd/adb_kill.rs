use crate::cmd::adb_basic::AdbSyncConnection;
use crate::cmd::AdbSyncCommand;
use crate::error::adb::AdbError;

pub struct AdbKillCommand {
    pub connection_str: String,
    pub read_timeout_mills: u64,
    pub write_timeout_mills: u64,
}

impl AdbSyncCommand for AdbKillCommand {
    fn execute(&mut self) -> Result<String, AdbError> {
        let mut adb_command = AdbSyncConnection {
            connection_str: self.connection_str.clone(),
            read_timeout_mills: self.read_timeout_mills.clone(),
            write_timeout_mills: self.write_timeout_mills.clone(),
        };
        match adb_command.exec_command(String::from("host:kill")) {
            Ok(resp) => { Ok(resp) }
            Err(error) => {
                match error {
                    AdbError::TcpReadError { .. } => {Ok(String::from(""))}
                    _ => {Err(error)}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::adb_kill::AdbKillCommand;
    use crate::cmd::AdbSyncCommand;

    #[test]
    fn read_commands() {
        let handler = log4rs::init_file("log4rs.yml",Default::default());
        let mut command = AdbKillCommand {
            connection_str: String::from("127.0.0.1:5037"),
            read_timeout_mills: 1,
            write_timeout_mills: 1000,
        };
        let resp = command.execute().unwrap();
        println!("adb kill {}", resp);
    }
}
