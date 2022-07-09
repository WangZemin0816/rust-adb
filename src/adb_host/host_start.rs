use crate::adb_host::HostConnectionInfo;
use crate::adb_host::{SyncHostCommand, SyncHostResponse};
use crate::error::adb::AdbError;

use std::process::Command;

pub struct AdbHostStartCommand {
    pub bin_path: String,
    pub connection_info: HostConnectionInfo,
}

impl SyncHostCommand for AdbHostStartCommand {
    fn execute(&mut self) -> Result<SyncHostResponse, AdbError> {
        match Command::new(self.bin_path.clone())
            .arg("-P")
            .arg(format!("{}", self.connection_info.port))
            .arg("start-server")
            .output()
        {
            Ok(response) => {
                if response.status.success() {
                    let content = String::from_utf8_lossy(&response.stdout);
                    return Ok(SyncHostResponse {
                        content: String::from(content.clone()),
                        length: content.len(),
                    });
                }

                let error = String::from_utf8_lossy(&response.stderr);
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
