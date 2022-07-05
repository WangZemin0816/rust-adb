use std::net::TcpStream;
use std::time::Duration;
use std::io::{Read, Write};
use log::{debug, trace};
use crate::error::adb::AdbError;

pub mod host_disconnect;
pub mod host_kill;
pub mod host_list_device;
pub mod host_list_device_l;
pub mod host_track_devices;
pub mod host_transport;
pub mod host_version;
pub mod host_device_status;
pub mod host_device_path;

pub trait SyncHostCommand {
    fn execute(&mut self) -> Result<SyncHostProtocol, AdbError>;
}

pub trait AsyncHostCommand {
    fn execute(&mut self) -> Result<AsyncHostProtocol, AdbError>;
}

pub enum SyncHostProtocol {
    OKAY { length: usize, content: String },
    FAIL { length: usize, content: String },
}

pub enum AsyncHostProtocol {
    OKAY { tcp_stream: TcpStream },
    FAIL { length: usize, content: String },
}

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
    trace!(
        "[connect]begin to create a new tcp connection: connection_str={}",
        connection_str.clone()
    );
    let tcp_stream = match TcpStream::connect(connection_str.clone()) {
        Ok(tcp_stream) => tcp_stream,
        Err(error) => {
            debug!(
                "[connect]create tcp connection failed: connection_str={}",
                connection_str.clone()
            );
            return Err(AdbError::TcpConnectError {
                source: Box::new(error),
            });
        }
    };
    match tcp_stream.set_read_timeout(connection_info.read_timeout) {
        Ok(_) => {}
        Err(error) => {
            debug!("[connect]init connect read timeout failed");
            return Err(AdbError::TcpReadError {
                source: Box::new(error),
            });
        }
    };
    match tcp_stream.set_write_timeout(connection_info.write_timeout) {
        Ok(_) => {}
        Err(error) => {
            debug!("[connect]init connect write timeout failed");
            return Err(AdbError::TcpReadError {
                source: Box::new(error),
            });
        }
    };
    Ok(tcp_stream)
}

pub fn exec_command_sync(
    mut tcp_stream: TcpStream,
    command: String,
) -> Result<AsyncHostProtocol, AdbError> {
    trace!("[exec_command_sync]exec command: command={}", command);

    write_command(&mut tcp_stream, &command)?;
    trace!("[exec_command_sync]write command: command={}", command);

    let status = read_response_status(&mut tcp_stream)?;
    trace!("[exec_command_sync]response status: status={}", status);

    if status == "OKAY" {
        return Ok(AsyncHostProtocol::OKAY { tcp_stream });
    }

    if status == "FAIL" {
        let length = read_response_length(&mut tcp_stream)?;
        trace!("[exec_command_sync]response length: length={}", length);

        let content = read_response_content(&mut tcp_stream, length)?;
        trace!("[exec_command_sync]response content: content={}", content);
        return Ok(AsyncHostProtocol::FAIL { length, content });
    }

    Err(AdbError::ResponseStatusError {
        message: String::from("unknown response status ") + &*status,
    })
}

pub fn exec_command(tcp_stream: &mut TcpStream, command: String) -> Result<SyncHostProtocol, AdbError> {
    trace!("[exec_command]exec command: command={}", command);

    write_command(tcp_stream, &command)?;
    trace!("[exec_command]write command: command={}", command);

    let status = read_response_status(tcp_stream)?;
    trace!("[exec_command]response status: status={}", status);

    let length = read_response_length(tcp_stream)?;
    trace!("[exec_command]response length: length={}", length);

    let content = read_response_content(tcp_stream, length)?;
    trace!("[exec_command]response content: content={}", content);

    if status == "OKAY" {
        return Ok(SyncHostProtocol::OKAY { length, content });
    }
    if status == "FAIL" {
        return Ok(SyncHostProtocol::FAIL { length, content });
    }
    Err(AdbError::ResponseStatusError {
        message: String::from("unknown response status ") + &*status,
    })
}

pub fn write_command(tcp_stream: &mut TcpStream, command: &String) -> Result<(), AdbError> {
    let full_command = add_command_length_prefix(command.clone());
    trace!("[write_command]full command: command={}", full_command);
    match tcp_stream.write_all(full_command.as_ref()) {
        Ok(_) => Ok(()),
        Err(error) => {
            trace!("[write_command]write command failed: err={:?}", error);
            Err(AdbError::TcpWriteError {
                source: Box::new(error),
            })
        }
    }
}

pub fn read_response_content(
    tcp_stream: &mut TcpStream,
    length: usize,
) -> Result<String, AdbError> {
    let mut response_content = vec![0; length];
    match tcp_stream.read_exact(&mut response_content) {
        Ok(_) => {}
        Err(error) => {
            trace!(
                "[read_response_content]read content failed: error={}",
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
                "[read_response_content]read command content success: content={}",
                &content_string
            );
            Ok(content_string)
        }
        Err(error) => {
            trace!(
                "[read_response_content]parse command content to utf-8 failed: error={}",
                &error
            );
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
            trace!(
                "[read_response_length]read command content length from stream failed: error={:?}",
                &error
            );
            return Err(AdbError::TcpReadError {
                source: Box::new(error),
            });
        }
    }
    match String::from_utf8(Vec::from(content_length)) {
        Ok(response) => {
            trace!(
                "[read_response_length]read command content length success: length={}",
                &response
            );
            match usize::from_str_radix(&*response, 16) {
                Ok(size) => {
                    trace!(
                        "[read_response_length]parse command content length success: length={}",
                        &size
                    );
                    Ok(size)
                }
                Err(error) => {
                    trace!("[read_response_length]parse command content length from hex to usize failed: length={}",&error);
                    Err(AdbError::ParseResponseError {
                        source: Box::new(error),
                    })
                }
            }
        }
        Err(error) => {
            trace!("[read_response_length]parse command content length to utf-8 string failed: error={}",&error);
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
            trace!(
                "[read_response_status]read command status from stream failed: error={:?}",
                &error
            );
            return Err(AdbError::TcpReadError {
                source: Box::new(error),
            });
        }
    }
    match String::from_utf8(Vec::from(is_ok_buffer)) {
        Ok(response_status) => Ok(response_status),
        Err(error) => {
            trace!(
                "[read_response_status]parse response status to utf-8 failed: err={}",
                error
            );
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
