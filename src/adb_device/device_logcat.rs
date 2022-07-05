use crate::adb_device::device_shell_async::DeviceAsyncShellCommand;
use crate::adb_device::device_shell_sync::DeviceSyncShellCommand;
use crate::adb_device::{
    device_connection, exec_device_command, AsyncDeviceCommand, AsyncDeviceProtocol,
    DeviceConnectionInfo, SyncDeviceCommand, SyncDeviceProtocol,
};
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

#[cfg(test)]
mod tests {
    use crate::adb_device::device_reboot::DeviceRebootCommand;
    use encoding_rs::SHIFT_JIS;
    use std::io::{BufRead, BufReader, Read};
    use std::thread;
    use std::time::Duration;

    use crate::adb_device::device_logcat::DeviceLogcatCommand;
    use crate::adb_device::device_remount::DeviceRemountCommand;
    use crate::adb_device::device_root::DeviceRootCommand;
    use crate::adb_device::{
        AsyncDeviceCommand, AsyncDeviceProtocol, DeviceConnectionInfo, SyncDeviceCommand,
        SyncDeviceProtocol,
    };

    use crate::adb_host::SyncHostCommand;

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let conn = DeviceConnectionInfo::new(
            &String::from("127.0.0.1"),
            &5037,
            &String::from("emulator-5554"),
        );
        let mut command = DeviceLogcatCommand {
            params: "-c".to_string(),
            connection_info: conn,
        };
        let resp = command.execute().unwrap();
        let decode = SHIFT_JIS.new_decoder();
        match resp {
            AsyncDeviceProtocol::OKAY { mut tcp_stream } => loop {
                let mut buff = vec![0;1024];
                tcp_stream.read(&mut buff).unwrap();
                println!("{}",String::from_utf8_lossy(&*buff))

            },
            AsyncDeviceProtocol::FAIL { content, .. } => {
                println!("logcat failed {}", content)
            }
        }
        thread::sleep(Duration::from_secs(2000));
    }
}
