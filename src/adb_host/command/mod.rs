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

mod host_disconnect;
mod host_kill;
mod host_list_device;
mod host_list_device_l;
mod host_track_devices;
mod host_transport;
mod host_version;

pub trait SyncHostCommand {
    fn execute(&mut self) -> Result<SyncProtocol, AdbError>;
}

pub trait AsyncHostCommand {
    fn execute(&mut self) -> Result<AsyncProtocol, AdbError>;
}


pub fn new_host_transport_command(
    host: String,
    port: i32,
    serial_no: String,
) -> impl AsyncHostCommand {
    let conn = ConnectionInfo::new(&host, port);
    AdbHostTransportCommand {
        serial_no: serial_no.clone(),
        connection_info: conn,
    }
}

pub fn new_host_track_device_command(host: String, port: i32) -> impl AsyncHostCommand {
    let conn = ConnectionInfo {
        host,
        port,
        read_timeout: None,
        write_timeout: Option::from(Duration::from_millis(1000)),
    };
    AdbHostTrackDeviceCommand {
        connection_info: conn,
    }
}
