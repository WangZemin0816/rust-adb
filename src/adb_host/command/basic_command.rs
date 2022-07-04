use std::io::{Read, Write};
use std::net::TcpStream;

use log::{info, trace};

use crate::adb_host::protocol::{AsyncProtocol, SyncProtocol};
use crate::error::adb::AdbError;

pub fn exec_command_sync(
    mut tcp_stream: TcpStream,
    command: String,
) -> Result<AsyncProtocol, AdbError> {
    trace!("[exec_command]exec command: command={}", command);

    write_command(&mut tcp_stream, &command)?;
    trace!("[exec_command]write command: command={}", command);

    let status = read_response_status(&mut tcp_stream)?;
    trace!("[exec_command]response status: status={}", status);

    return AsyncProtocol::from_response(status,tcp_stream)
}

pub fn exec_command(tcp_stream: &mut TcpStream, command: String) -> Result<SyncProtocol, AdbError> {
    trace!("[exec_command]exec command: command={}", command);

    write_command(tcp_stream, &command)?;
    trace!("[exec_command]write command: command={}", command);

    let status = read_response_status(tcp_stream)?;
    trace!("[exec_command]response status: status={}", status);

    let length = read_response_length(tcp_stream)?;
    trace!("[exec_command]response length: length={}", length);

    let content = read_response_content(tcp_stream, length)?;
    trace!("[exec_command]response content: content={}", content);

    return SyncProtocol::from_response(status, length, content);
}

fn write_command(tcp_stream: &mut TcpStream, command: &String) -> Result<(), AdbError> {
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

fn read_response_content(tcp_stream: &mut TcpStream, length: usize) -> Result<String, AdbError> {
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

fn read_response_length(tcp_stream: &mut TcpStream) -> Result<usize, AdbError> {
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

fn read_response_status(tcp_stream: &mut TcpStream) -> Result<String, AdbError> {
    let mut is_ok_buffer = [0; 4];
    match tcp_stream.read_exact(&mut is_ok_buffer) {
        Ok(_) => {}
        Err(error) => {
            info!(
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

fn add_command_length_prefix(command_body: String) -> String {
    let trim_command = command_body.trim();
    let trim_command_length = format!("{:04X}", trim_command.len());
    trim_command_length + trim_command
}
