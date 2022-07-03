use std::net::TcpStream;
use std::time::Duration;

use log::{debug, trace};

use crate::error::adb::AdbError;

pub struct ConnectionInfo {
    pub host: String,
    pub port: i32,
    pub read_timeout_mills: u64,
    pub write_timeout_mills: u64,
}

impl ConnectionInfo {
    pub fn new(host: &String, port: i32) -> ConnectionInfo {
        ConnectionInfo {
            host: host.clone(),
            port: port.clone(),
            read_timeout_mills: 1000,
            write_timeout_mills: 1000,
        }
    }
}

pub fn connect(connection_info: &ConnectionInfo) -> Result<TcpStream, AdbError> {
    let connection_str = format!(
        "{}:{}",
        connection_info.host.clone(),
        connection_info.port.clone()
    );
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
    match tcp_stream.set_read_timeout(Option::from(Duration::from_millis(
        connection_info.read_timeout_mills.clone(),
    ))) {
        Ok(_) => {}
        Err(error) => {
            debug!("[connect]init connect read timeout failed");
            return Err(AdbError::TcpReadError {
                source: Box::new(error),
            });
        }
    };
    match tcp_stream.set_write_timeout(Option::from(Duration::from_millis(
        connection_info.read_timeout_mills.clone(),
    ))) {
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
