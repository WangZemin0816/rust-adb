
use crate::adb_host::{
    read_response_content, read_response_length, read_response_status, write_command,
    AsyncHostCommand, AsyncHostProtocol,
};
use log::trace;
use std::io::Read;
use std::net::TcpStream;
use std::time::Duration;

use crate::adb_host::host_transport::AdbHostTransportCommand;
use crate::adb_host::HostConnectionInfo;
use crate::error::adb::AdbError;

mod device_get_features;
mod device_get_packages;
mod device_get_properties;
mod device_reboot;
mod device_remount;
mod device_root;
mod device_shell_async;
mod device_shell_sync;
mod device_logcat;

pub trait SyncDeviceCommand {
    fn execute(&mut self) -> Result<SyncDeviceProtocol, AdbError>;
}

pub trait AsyncDeviceCommand {
    fn execute(&mut self) -> Result<AsyncDeviceProtocol, AdbError>;
}

pub enum SyncDeviceProtocol {
    OKAY { length: usize, content: String },
    FAIL { length: usize, content: String },
}

pub enum AsyncDeviceProtocol {
    OKAY { tcp_stream: TcpStream },
    FAIL { length: usize, content: String },
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
        AdbHostTransportCommand::new0(&host_connection_info, &device_connection_info.serial_no);
    let async_protocol = command.execute()?;
    match async_protocol {
        AsyncHostProtocol::OKAY { tcp_stream } => Ok(tcp_stream),
        AsyncHostProtocol::FAIL { content, .. } => {
            Err(AdbError::ResponseStatusError { message: content })
        }
    }
}

pub fn exec_device_command_sync(
    mut tcp_stream: TcpStream,
    command: String,
) -> Result<AsyncDeviceProtocol, AdbError> {
    trace!("[exec_command_sync]exec command: command={}", command);

    write_command(&mut tcp_stream, &command)?;
    trace!("[exec_command_sync]write command: command={}", command);

    let status = read_response_status(&mut tcp_stream)?;
    trace!("[exec_command_sync]response status: status={}", status);

    if status == "OKAY" {
        return Ok(AsyncDeviceProtocol::OKAY { tcp_stream });
    }

    if status == "FAIL" {
        let length = read_response_length(&mut tcp_stream)?;
        trace!("[exec_command_sync]response length: length={}", length);

        let content = read_response_content(&mut tcp_stream, length)?;
        trace!("[exec_command_sync]response content: content={}", content);
        return Ok(AsyncDeviceProtocol::FAIL { length, content });
    }

    Err(AdbError::ResponseStatusError {
        message: String::from("unknown response status ") + &*status,
    })
}

pub fn exec_device_command(
    tcp_stream: &mut TcpStream,
    command: String,
) -> Result<SyncDeviceProtocol, AdbError> {
    trace!("[exec_device_command]exec command: command={}", command);

    write_command(tcp_stream, &command)?;
    trace!("[exec_device_command]write command: command={}", command);

    let status = read_response_status(tcp_stream)?;
    trace!("[exec_device_command]response status: status={}", status);

    if status == "OKAY" {
        let content = read_response_all_content(tcp_stream)?;
        trace!("[exec_device_command]response content: content={}", content);

        return Ok(SyncDeviceProtocol::OKAY {
            length: content.len(),
            content,
        });
    }

    if status == "FAIL" {
        let length = read_response_length(tcp_stream)?;
        trace!("[exec_command_sync]response length: length={}", length);

        let content = read_response_content(tcp_stream, length)?;
        trace!("[exec_command_sync]response content: content={}", content);
        return Ok(SyncDeviceProtocol::FAIL { length, content });
    }

    Err(AdbError::ResponseStatusError {
        message: String::from("unknown response status ") + &*status,
    })
}

pub fn read_response_all_content(tcp_stream: &mut TcpStream) -> Result<String, AdbError> {
    let mut response_content = vec![];
    match tcp_stream.read_to_end(&mut response_content) {
        Ok(_) => {}
        Err(error) => {
            trace!(
                "[read_response_all_content]read content failed: error={}",
                error
            );
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
