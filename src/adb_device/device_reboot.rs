use crate::adb_device::{
    device_connection, exec_device_command, DeviceConnectionInfo, SyncDeviceCommand, SyncDeviceProtocol,
};
use crate::error::adb::AdbError;

pub struct DeviceRebootCommand {
    pub connection_info: DeviceConnectionInfo,
}

impl SyncDeviceCommand for DeviceRebootCommand {
    fn execute(&mut self) -> Result<SyncDeviceProtocol, AdbError> {
        let mut tcp_stream = device_connection(&self.connection_info)?;
        match exec_device_command(&mut tcp_stream, "reboot:".to_string()) {
            | Ok(response) => Ok(response),
            | Err(error) => match error {
                | AdbError::TcpReadError { .. } => Ok(SyncDeviceProtocol {
                    content: "".to_string(),
                    length: 0,
                }),
                | _ => Err(error),
            },
        }
    }
}
