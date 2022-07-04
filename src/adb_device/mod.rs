use std::net::TcpStream;
use crate::adb_host::{AsyncCommand, AsyncProtocol};

use crate::adb_host::host_transport::AdbHostTransportCommand;
use crate::adb_host::ConnectionInfo;
use crate::error::adb::AdbError;

pub mod device_get_path;
mod device_get_status;
mod device_get_properties;
mod device_get_features;

fn device_connection(
    connect_info: &ConnectionInfo,
    serial_no: &String,
) -> Result<TcpStream, AdbError> {
    let mut command = AdbHostTransportCommand::new0(connect_info, serial_no);
    let async_protocol = command.execute()?;
    match async_protocol {
        AsyncProtocol::OKAY { tcp_stream } => Ok(tcp_stream),
        AsyncProtocol::FAIL { content, .. } => {
            Err(AdbError::ResponseStatusError { message: content })
        }
    }
}
