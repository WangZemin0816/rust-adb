pub mod adb_basic;
pub mod adb_version;
pub mod adb_kill;
pub mod adb_devices;
pub mod adb_track_device;

use std::thread::JoinHandle;
use crate::error::adb::AdbError;


pub trait AdbSyncCommand {
    fn execute(&mut self) -> Result<String, AdbError>;
}

pub trait AdbAsyncCommand {
    fn execute(&mut self, consumer: fn(&String) -> Result<(), AdbError>, error_handler: fn(&AdbError)) -> JoinHandle<()>;
}


// pub fn new_adb_version_cmd(connection_str: String) -> impl AdbSyncCommand {
//     AdbVersionCommand { connection_str }
// }

