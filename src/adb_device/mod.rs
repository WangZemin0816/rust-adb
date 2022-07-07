use crate::adb_host::{
    read_response_content, read_response_length, read_response_status, write_command,
    AsyncHostCommand,
};
use log::trace;
use std::io::Read;
use std::net::TcpStream;
use std::time::Duration;

use crate::adb_host::host_transport::AdbHostTransportCommand;
use crate::adb_host::HostConnectionInfo;
use crate::error::adb::AdbError;

pub mod device_get_features;
pub mod device_get_packages;
pub mod device_get_properties;
pub mod device_logcat;
pub mod device_reboot;
pub mod device_remount;
pub mod device_root;
pub mod device_shell_async;
pub mod device_shell_sync;

pub trait SyncDeviceCommand {
    fn execute(&mut self) -> Result<SyncDeviceProtocol, AdbError>;
}

pub trait AsyncDeviceCommand {
    fn execute(&mut self) -> Result<AsyncDeviceProtocol, AdbError>;
}

pub struct SyncDeviceProtocol {
    pub length: usize,
    pub content: String,
}

pub struct AsyncDeviceProtocol {
    pub tcp_stream: TcpStream,
}

pub struct DeviceConnectionInfo {
    pub host: String,
    pub port: i32,
    pub serial_no: String,
    pub read_timeout: Option<Duration>,
    pub write_timeout: Option<Duration>,
}

impl Clone for DeviceConnectionInfo {
    fn clone(&self) -> Self {
        DeviceConnectionInfo {
            host: self.host.clone(),
            port: self.port.clone(),
            serial_no: self.serial_no.clone(),
            read_timeout: self.read_timeout.clone(),
            write_timeout: self.write_timeout.clone(),
        }
    }
}

impl DeviceConnectionInfo {
    pub fn new(host: &String, port: &i32, serial_no: &String) -> DeviceConnectionInfo {
        DeviceConnectionInfo {
            host: host.clone(),
            port: port.clone(),
            serial_no: serial_no.clone(),
            read_timeout: Option::from(Duration::from_millis(1000)),
            write_timeout: Option::from(Duration::from_millis(1000)),
        }
    }

    pub fn host_connection_info(&self) -> HostConnectionInfo {
        HostConnectionInfo {
            host: self.host.clone(),
            port: self.port.clone(),
            read_timeout: self.read_timeout.clone(),
            write_timeout: self.write_timeout.clone(),
        }
    }
}

fn device_connection(device_connection_info: &DeviceConnectionInfo) -> Result<TcpStream, AdbError> {
    let host_connection_info = device_connection_info.host_connection_info();
    let mut command =
        AdbHostTransportCommand::new(&host_connection_info, &device_connection_info.serial_no);
    let async_protocol = command.execute()?;
    Ok(async_protocol.tcp_stream)
}

pub fn exec_device_command_sync(
    mut tcp_stream: TcpStream, command: String,
) -> Result<AsyncDeviceProtocol, AdbError> {
    trace!("[exec_command_sync]exec command: command={}", command);

    write_command(&mut tcp_stream, &command)?;
    trace!("[exec_command_sync]write command: command={}", command);

    let status = read_response_status(&mut tcp_stream)?;
    trace!("[exec_command_sync]response status: status={}", status);

    if status == "OKAY" {
        return Ok(AsyncDeviceProtocol { tcp_stream });
    }

    if status == "FAIL" {
        let length = read_response_length(&mut tcp_stream)?;
        trace!("[exec_command_sync]response length: length={}", length);

        let content = read_response_content(&mut tcp_stream, length)?;
        trace!("[exec_command_sync]response content: content={}", content);
        return Err(AdbError::ResponseStatusError { content });
    }

    Err(AdbError::ResponseStatusError {
        content: String::from("unknown response status ") + &*status,
    })
}

pub fn exec_device_command(
    tcp_stream: &mut TcpStream, command: String,
) -> Result<SyncDeviceProtocol, AdbError> {
    trace!("[exec_device_command]exec command: command={}", command);

    write_command(tcp_stream, &command)?;
    trace!("[exec_device_command]write command: command={}", command);

    let status = read_response_status(tcp_stream)?;
    trace!("[exec_device_command]response status: status={}", status);

    if status == "OKAY" {
        let content = read_response_all_content(tcp_stream)?;
        trace!("[exec_device_command]response content: content={}", content);

        return Ok(SyncDeviceProtocol {
            length: content.len(),
            content,
        });
    }

    if status == "FAIL" {
        let length = read_response_length(tcp_stream)?;
        trace!("[exec_command_sync]response length: length={}", length);

        let content = read_response_content(tcp_stream, length)?;
        trace!("[exec_command_sync]response content: content={}", content);
        return Err(AdbError::ResponseStatusError { content });
    }

    Err(AdbError::ResponseStatusError {
        content: String::from("unknown response status ") + &*status,
    })
}

pub fn read_response_all_content(tcp_stream: &mut TcpStream) -> Result<String, AdbError> {
    let mut response_content = vec![];
    match tcp_stream.read_to_end(&mut response_content) {
        Ok(_) => {}
        Err(error) => {
            trace!("[read_response_all_content]read content failed: error={}", error);
            return Err(AdbError::TcpReadError {
                source: Box::new(error),
            });
        }
    };

    match String::from_utf8(Vec::from(response_content)) {
        Ok(content_string) => {
            trace!(
                "[read_response_all_content]read command content success: content={}",
                &content_string
            );
            Ok(content_string)
        }
        Err(error) => {
            trace!(
                "[read_response_all_content]parse command content to utf-8 failed: error={}",
                &error
            );
            return Err(AdbError::ParseResponseError {
                source: Box::new(error),
            });
        }
    }
}
