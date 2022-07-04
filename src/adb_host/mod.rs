use crate::basic::protocol::{AsyncProtocol, SyncProtocol};
use crate::error::adb::AdbError;

pub mod host_disconnect;
pub mod host_kill;
pub mod host_list_device;
pub mod host_list_device_l;
pub mod host_track_devices;
pub mod host_transport;
pub mod host_version;

pub trait SyncHostCommand {
    fn execute(&mut self) -> Result<SyncProtocol, AdbError>;
}

pub trait AsyncHostCommand {
    fn execute(&mut self) -> Result<AsyncProtocol, AdbError>;
}
