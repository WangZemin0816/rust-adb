use std::net::TcpStream;
use crate::adb_host::AsyncHostCommand;
use crate::adb_host::host_transport::AdbHostTransportCommand;
use crate::basic::protocol::AsyncProtocol;
use crate::client::DeviceService;
use crate::error::adb::AdbError;

pub struct DeviceClient {
    pub host: String,
    pub port: i32,
    pub serial_no: String,
}

impl DeviceClient {
    pub fn new(host: String, port: i32, serial_no: String) -> DeviceClient {
        DeviceClient {
            host,
            port,
            serial_no,
        }
    }
}

impl DeviceService for DeviceClient {
    fn get_connection(&mut self) -> Result<TcpStream, AdbError> {
        let mut command = AdbHostTransportCommand::new(&self.host, &self.port, &self.serial_no);
        let async_protocol = command.execute()?;
        match async_protocol {
            AsyncProtocol::OKAY { tcp_stream } => Ok(tcp_stream),
            AsyncProtocol::FAIL { content, .. } => {
                Err(AdbError::ResponseStatusError { message: content })
            }
        }
    }
}
