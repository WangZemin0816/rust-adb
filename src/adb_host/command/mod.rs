use crate::adb_host::command::host_disconnect::AdbHostDisconnectCommand;
use crate::adb_host::command::host_kill::AdbHostKillCommand;
use crate::adb_host::command::host_list_device::AdbHostListDevicesCommand;
use crate::adb_host::command::host_list_device_l::AdbHostListDeviceLCommand;
use crate::adb_host::command::host_track_devices::AdbHostTrackDeviceCommand;
use crate::adb_host::command::host_transport::AdbHostTransportCommand;
use crate::adb_host::command::host_version::AdbHostVersionCommand;
use crate::conn::protocol::{AsyncProtocol, SyncProtocol};
use crate::conn::connection::ConnectionInfo;
use crate::error::adb::AdbError;
use std::time::Duration;

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
