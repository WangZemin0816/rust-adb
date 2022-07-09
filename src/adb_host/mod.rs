use crate::error::adb::AdbError;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

pub mod host_device_path;
pub mod host_device_status;
pub mod host_disconnect;
pub mod host_kill;
pub mod host_list_device;
pub mod host_list_device_l;
pub mod host_start;
pub mod host_track_devices;
pub mod host_transport;
pub mod host_version;

pub trait SyncHostCommand {
    fn execute(&mut self) -> Result<SyncHostResponse, AdbError>;
}

pub trait AsyncHostCommand {
    fn execute(&mut self) -> Result<AsyncHostResponse, AdbError>;
}

#[derive(Debug)]
pub struct SyncHostResponse {
    pub length: usize,
    pub content: String,
}

pub struct AsyncHostResponse {
    pub tcp_stream: TcpStream,
}

#[derive(Debug)]
pub struct HostConnectionInfo {
    pub host: String,
    pub port: i32,
    pub read_timeout: Option<Duration>,
    pub write_timeout: Option<Duration>,
}

impl HostConnectionInfo {
    pub fn new(host: &String, port: &i32) -> HostConnectionInfo {
        HostConnectionInfo {
            host: host.clone(),
            port: port.clone(),
            read_timeout: Option::from(Duration::from_millis(1000)),
            write_timeout: Option::from(Duration::from_millis(1000)),
        }
    }
}

impl Clone for HostConnectionInfo {
    fn clone(&self) -> Self {
        HostConnectionInfo {
            host: self.host.clone(),
            port: self.port.clone(),
            read_timeout: self.read_timeout.clone(),
            write_timeout: self.write_timeout.clone(),
        }
    }
}

pub fn connect(connection_info: &HostConnectionInfo) -> Result<TcpStream, AdbError> {
    let connection_str = format!("{}:{}", connection_info.host, connection_info.port);
    let tcp_stream = match TcpStream::connect(connection_str.clone()) {
        Ok(tcp_stream) => tcp_stream,
        Err(error) => {
            return Err(AdbError::TcpConnectError {
                source: Box::new(error),
            });
        }
    };
    match tcp_stream.set_read_timeout(connection_info.read_timeout) {
        Ok(_) => {}
        Err(error) => {
            return Err(AdbError::TcpReadError {
                source: Box::new(error),
            });
        }
    };
    match tcp_stream.set_write_timeout(connection_info.write_timeout) {
        Ok(_) => {}
        Err(error) => {
            return Err(AdbError::TcpReadError {
                source: Box::new(error),
            });
        }
    };
    Ok(tcp_stream)
}

pub fn exec_command_sync(
    mut tcp_stream: TcpStream, command: String,
) -> Result<AsyncHostResponse, AdbError> {

    write_command(&mut tcp_stream, &command)?;

    let status = read_response_status(&mut tcp_stream)?;

    if status == "OKAY" {
        return Ok(AsyncHostResponse { tcp_stream });
    }

    if status == "FAIL" {
        let length = read_response_length(&mut tcp_stream)?;

        let content = read_response_content(&mut tcp_stream, length)?;
        return Err(AdbError::ResponseStatusError { content });
    }

    Err(AdbError::ResponseStatusError {
        content: String::from("unknown response status ") + &*status,
    })
}

pub fn exec_command(
    tcp_stream: &mut TcpStream, command: String,
) -> Result<SyncHostResponse, AdbError> {

    write_command(tcp_stream, &command)?;

    let status = read_response_status(tcp_stream)?;

    let length = read_response_length(tcp_stream)?;

    let content = read_response_content(tcp_stream, length)?;

    if status == "OKAY" {
        return Ok(SyncHostResponse { length, content });
    }
    if status == "FAIL" {
        return Err(AdbError::ResponseStatusError { content });
    }
    Err(AdbError::ResponseStatusError {
        content: String::from("unknown response status ") + &*status,
    })
}

pub fn write_command(tcp_stream: &mut TcpStream, command: &String) -> Result<(), AdbError> {
    let full_command = add_command_length_prefix(command.clone());
    match tcp_stream.write_all(full_command.as_ref()) {
        Ok(_) => Ok(()),
        Err(error) => {
            Err(AdbError::TcpWriteError {
                source: Box::new(error),
            })
        }
    }
}

pub fn read_response_content(
    tcp_stream: &mut TcpStream, length: usize,
) -> Result<String, AdbError> {
    let mut response_content = vec![0; length];
    match tcp_stream.read_exact(&mut response_content) {
        Ok(_) => {}
        Err(error) => {
            return Err(AdbError::TcpReadError {
                source: Box::new(error),
            });
        }
    };

    match String::from_utf8(Vec::from(response_content)) {
        Ok(content_string) => {

            Ok(content_string)
        }
        Err(error) => {
            return Err(AdbError::ParseResponseError {
                source: Box::new(error),
            });
        }
    }
}

pub fn read_response_length(tcp_stream: &mut TcpStream) -> Result<usize, AdbError> {
    let mut content_length = [0; 4];
    match tcp_stream.read_exact(&mut content_length) {
        Ok(_) => {}
        Err(error) => {
            return Err(AdbError::TcpReadError {
                source: Box::new(error),
            });
        }
    }
    match String::from_utf8(Vec::from(content_length)) {
        Ok(response) => {
            match usize::from_str_radix(&*response, 16) {
                Ok(size) => {
                    Ok(size)
                }
                Err(error) => {
                    Err(AdbError::ParseResponseError {
                        source: Box::new(error),
                    })
                }
            }
        }
        Err(error) => {
            return Err(AdbError::ParseResponseError {
                source: Box::new(error),
            });
        }
    }
}

pub fn read_response_status(tcp_stream: &mut TcpStream) -> Result<String, AdbError> {
    let mut is_ok_buffer = [0; 4];
    match tcp_stream.read_exact(&mut is_ok_buffer) {
        Ok(_) => {}
        Err(error) => {
            return Err(AdbError::TcpReadError {
                source: Box::new(error),
            });
        }
    }
    match String::from_utf8(Vec::from(is_ok_buffer)) {
        Ok(response_status) => Ok(response_status),
        Err(error) => {
            Err(AdbError::ParseResponseError {
                source: Box::new(error),
            })
        }
    }
}

pub fn add_command_length_prefix(command_body: String) -> String {
    let trim_command = command_body.trim();
    let trim_command_length = format!("{:04X}", trim_command.len());
    trim_command_length + trim_command
}
