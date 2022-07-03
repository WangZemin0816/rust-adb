use crate::adb_host::command::host_disconnect::AdbHostDisconnectCommand;
use crate::adb_host::command::host_kill::AdbHostKillCommand;
use crate::adb_host::command::host_list_device::AdbHostListDevicesCommand;
use crate::adb_host::command::host_list_device_l::AdbHostListDeviceLCommand;
use crate::adb_host::command::host_transport::AdbHostTransportCommand;
use crate::adb_host::command::host_version::AdbHostVersionCommand;
use crate::adb_host::protocol::SyncProtocol;
use crate::conn::connection::ConnectionInfo;
use crate::error::adb::AdbError;

mod basic_async;
mod basic_sync;
mod host_disconnect;
mod host_kill;
mod host_list_device;
mod host_list_device_l;
mod host_transport;
mod host_version;

pub trait SyncHostCommand {
    fn execute(&mut self) -> Result<SyncProtocol, AdbError>;
}

pub fn new_host_version_command(host: String, port: i32) -> impl SyncHostCommand {
    let conn = ConnectionInfo::new(&host, port);
    AdbHostVersionCommand {
        connection_info: conn,
    }
}

pub fn new_host_disconnect_command(
    host: String,
    port: i32,
    dis_host: String,
    dis_port: i32,
) -> impl SyncHostCommand {
    let conn = ConnectionInfo::new(&host, port);
    AdbHostDisconnectCommand {
        host: dis_host,
        port: dis_port,
        connection_info: conn,
    }
}

pub fn new_host_list_device_command(host: String, port: i32) -> impl SyncHostCommand {
    let conn = ConnectionInfo::new(&host, port);
    AdbHostListDevicesCommand {
        connection_info: conn,
    }
}

pub fn new_host_list_device_l_command(host: String, port: i32) -> impl SyncHostCommand {
    let conn = ConnectionInfo::new(&host, port);
    AdbHostListDeviceLCommand {
        connection_info: conn,
    }
}

pub fn new_host_kill_command(host: String, port: i32) -> impl SyncHostCommand {
    let conn = ConnectionInfo::new(&host, port);
    AdbHostKillCommand {
        connection_info: conn,
    }
}

pub fn new_host_transport_command(
    host: String,
    port: i32,
    serial_no: String,
) -> impl SyncHostCommand {
    let conn = ConnectionInfo::new(&host, port);
    AdbHostTransportCommand {
        serial_no: serial_no.clone(),
        connection_info: conn,
    }
}
