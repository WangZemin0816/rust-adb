use std::io::Read;

use std::net::TcpStream;

use crate::adb_device::device_shell_async::DeviceAsyncShellCommand;

use crate::adb_device::{AsyncDeviceCommand, AsyncDeviceProtocol, DeviceConnectionInfo};
use crate::client::LogEntry;
use crate::error::adb::AdbError;

pub struct DeviceLogcatCommand {
    pub params: String,
    pub connection_info: DeviceConnectionInfo,
}

impl AsyncDeviceCommand for DeviceLogcatCommand {
    fn execute(&mut self) -> Result<AsyncDeviceProtocol, AdbError> {
        let command = format!("shell:echo && logcat {} 2>/dev/null", self.params);
        DeviceAsyncShellCommand::new(&self.connection_info, &command).execute()
    }
}

pub fn read_next_entry(tcp_stream: &mut TcpStream) -> Result<LogEntry, AdbError> {
    skip_un_use_bytes(tcp_stream)?;
    let length = read_next_uint16le(tcp_stream)?;
    let mut header_size = read_next_uint16le(tcp_stream)?;
    if header_size < 20 || header_size > 100 {
        header_size = 20;
    };
    let pid = read_next_int32le(tcp_stream)?;
    let tid = read_next_int32le(tcp_stream)?;
    let sec = read_next_int32le(tcp_stream)?;
    let nsec = read_next_int32le(tcp_stream)?;
    let mut header = vec![0; header_size as usize];
    match tcp_stream.read_exact(&mut header) {
        | Ok(_) => {}
        | Err(error) => {
            return Err(AdbError::TcpReadError {
                source: Box::new(error),
            });
        }
    };
    let mut body = vec![0; length as usize];
    match tcp_stream.read_exact(&mut body) {
        | Ok(_) => {}
        | Err(error) => {
            return Err(AdbError::TcpReadError {
                source: Box::new(error),
            });
        }
    };
    if body.len() < 2 {
        return Err(AdbError::ResponseStatusError {
            content: "read logcat content failed".to_string(),
        });
    }
    Ok(LogEntry {
        pid,
        tid,
        sec,
        nsec,
        header,
        tag: body[1] as u32,
        log: body[2..].to_owned(),
        priority: body[0] as u32,
    })
}

fn skip_un_use_bytes(tcp_stream: &mut TcpStream) -> Result<(), AdbError> {
    let mut buf = vec![0; 1];
    while buf[0] != 0x0a {
        match tcp_stream.read_exact(&mut buf) {
            | Ok(_) => {}
            | Err(error) => {
                return Err(AdbError::TcpReadError {
                    source: Box::new(error),
                });
            }
        };
    }
    Ok(())
}

fn read_next_int32le(tcp_stream: &mut TcpStream) -> Result<u32, AdbError> {
    let mut buf = vec![0; 4];
    match tcp_stream.read_exact(&mut buf) {
        | Ok(_) => {}
        | Err(error) => {
            return Err(AdbError::TcpReadError {
                source: Box::new(error),
            });
        }
    };
    let bit1 = format!("{:02x}", buf[0]);
    let bit2 = format!("{:02x}", buf[1]);
    let bit3 = format!("{:02x}", buf[2]);
    let bit4 = format!("{:02x}", buf[3]);
    let combined = bit4 + &bit3 + &bit2 + &bit1;
    match u32::from_str_radix(&combined, 16) {
        | Ok(size) => Ok(size),
        | Err(error) => Err(AdbError::ParseResponseError {
            source: Box::new(error),
        }),
    }
}

fn read_next_uint16le(tcp_stream: &mut TcpStream) -> Result<u16, AdbError> {
    let mut buf = vec![0; 2];
    match tcp_stream.read_exact(&mut buf) {
        | Ok(_) => {}
        | Err(error) => {
            return Err(AdbError::TcpReadError {
                source: Box::new(error),
            });
        }
    };
    let high = format!("{:02x}", buf[0]);
    let low = format!("{:02x}", buf[1]);
    let combined = low + &high;
    match u16::from_str_radix(&combined, 16) {
        | Ok(size) => Ok(size),
        | Err(error) => Err(AdbError::ParseResponseError {
            source: Box::new(error),
        }),
    }
}

#[cfg(test)]
mod tests {
    use log::trace;
    use std::thread;
    use std::time::Duration;

    use crate::adb_device::device_logcat::{read_next_entry, DeviceLogcatCommand};

    use crate::adb_device::{AsyncDeviceCommand, AsyncDeviceProtocol, DeviceConnectionInfo};

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let conn = DeviceConnectionInfo {
            host: String::from("127.0.0.1"),
            port: 5037,
            serial_no: "emulator-5554".to_string(),
            read_timeout: None,
            write_timeout: Option::from(Duration::from_millis(1000)),
        };
        let mut command = DeviceLogcatCommand {
            params: " -B *:I ".to_string(),
            connection_info: conn,
        };
        let resp = command.execute().unwrap();
        match resp {
            | AsyncDeviceProtocol::OKAY { mut tcp_stream } => loop {
                let entry = read_next_entry(&mut tcp_stream).unwrap();
                trace!("read entry {:?}", String::from_utf8_lossy(&entry.log));
            },
            | _ => {}
        }
        thread::sleep(Duration::from_secs(2000));
    }
}
