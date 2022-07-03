pub mod adb_devices;
pub mod adb_kill;
pub mod adb_track_device;
pub mod adb_version;

use crate::error::adb::AdbError;
use std::thread::JoinHandle;

pub trait AdbSyncCommand {
    fn execute(&mut self) -> Result<String, AdbError>;
}

pub trait AdbAsyncCommand {
    fn execute(
        &mut self,
        consumer: fn(&String) -> Result<(), AdbError>,
        error_handler: fn(&AdbError),
    ) -> JoinHandle<()>;
}

// pub fn new_adb_version_cmd(connection_str: String) -> impl AdbSyncCommand {
//     AdbVersionCommand { connection_str }
// }
