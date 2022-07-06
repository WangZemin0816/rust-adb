use crate::adb_host::HostConnectionInfo;
use crate::adb_host::{SyncHostCommand, SyncHostResponse};
use crate::error::adb::AdbError;
use log::trace;

use std::process::Command;

pub struct AdbHostStartCommand {
    pub bin_path: String,
    pub connection_info: HostConnectionInfo,
}

impl SyncHostCommand for AdbHostStartCommand {
    fn execute(&mut self) -> Result<SyncHostResponse, AdbError> {
        trace!(
            "[start_adb_server]start adb: connect={:?} bin={}",
            self.connection_info,
            self.bin_path
        );
        match Command::new(self.bin_path.clone())
            .arg("-P")
            .arg(format!("{}", self.connection_info.port))
            .arg("start-server")
            .output()
        {
            Ok(response) => {
                if response.status.success() {
                    let content = String::from_utf8_lossy(&response.stdout);
                    trace!("[start_adb_server]start adb success: stdout={}", content);
                    return Ok(SyncHostResponse {
                        content: String::from(content.clone()),
                        length: content.len(),
                    });
                }

                let error = String::from_utf8_lossy(&response.stderr);
                trace!("[start_adb_server]start adb failed: stderr={}", error);
                Err(AdbError::ResponseStatusError {
                    content: String::from(error.clone()),
                })
            }
            Err(error) => Err(AdbError::StartAdbFailed {
                bin_path: self.bin_path.clone(),
                source: Box::new(error),
            }),
        }
    }
}

impl AdbHostStartCommand {
    pub fn new(host: &String, port: &i32, bin_path: &String) -> AdbHostStartCommand {
        let connect_info = HostConnectionInfo::new(host, port);
        AdbHostStartCommand {
            bin_path: bin_path.clone(),
            connection_info: connect_info,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::adb_host::host_start::AdbHostStartCommand;

    use crate::adb_host::HostConnectionInfo;
    use crate::adb_host::SyncHostCommand;

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let conn = HostConnectionInfo::new(&String::from("127.0.0.1"), &5037);
        let mut command = AdbHostStartCommand {
            bin_path: "adb".to_string(),
            connection_info: conn,
        };
        let resp = command.execute().unwrap();
        println!("{:?}", resp)
    }
}
