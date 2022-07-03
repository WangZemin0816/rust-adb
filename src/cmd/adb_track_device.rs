use std::thread::JoinHandle;
use crate::cmd::adb_basic::{AdbAsyncConnection};
use crate::cmd::AdbAsyncCommand;
use crate::error::adb::AdbError;

pub struct AdbTrackDeviceCommand {
    pub connection_str: String,
    pub read_timeout_mills: u64,
    pub write_timeout_mills: u64,
}

impl AdbAsyncCommand for AdbTrackDeviceCommand {
    fn execute(&mut self, consumer: fn(&String) -> Result<(), AdbError>, error_handler: fn(&AdbError)) -> JoinHandle<()> {
            let mut adb_command = AdbAsyncConnection {
                connection_str: self.connection_str.clone(),
                read_timeout_mills: self.read_timeout_mills.clone(),
                write_timeout_mills: self.write_timeout_mills.clone(),
            };
            adb_command.exec_command_async(String::from("host:track-devices"),consumer,error_handler)
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::adb_track_device::AdbTrackDeviceCommand;
    use crate::cmd::AdbAsyncCommand;
    use crate::error::adb::AdbError;

    #[test]
    fn read_commands() {
        let handler = log4rs::init_file("log4rs.yml",Default::default());
        let mut command = AdbTrackDeviceCommand {
            connection_str: String::from("127.0.0.1:5037"),
            read_timeout_mills:1000,
            write_timeout_mills:1000
        };
        let print_consumer = |log:&String|{
            println!("{}",log);
            Ok(())
        };
        let print_error = |err:&AdbError|{
            println!("errrrrrrrr {}",err);
            match err {
                AdbError::TcpConnectError { .. } => {
                    println!("aaaaaaaaaaaaaaaaaaaa")
                }
                AdbError::TcpWriteError { .. } => {
                    println!("bbbbbbbbbbbb")
                }
                AdbError::TcpReadError { .. } => {
                    println!("cccccccccccccccccccccccc")
                }
                AdbError::ParseResponseError { .. } => {
                    println!("ddddddddddddddddddddddddddddd")
                }
                AdbError::ResponseStatusError { .. } => {
                    println!("eeeeeeeeeeeeeeeeeeeeeeeeeeee")
                }
            };
        };
        let resp = command.execute(print_consumer,print_error);
        resp.join();
        // println!("adb version {}", resp);
    }
}
