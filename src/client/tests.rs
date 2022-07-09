
#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;
    use crate::adb_host::host_device_status::HostDeviceStatusCommand;
    use crate::adb_host::HostConnectionInfo;
    use crate::adb_host::SyncHostCommand;
    use crate::client::adb_client::AdbClientImpl;
    use crate::client::device_client::DeviceClientImpl;
    use crate::client::{DeviceService, LogEntry};
    use crate::error::adb::AdbError;

    #[test]
    fn get_packages() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let mut conn = DeviceClientImpl{
            host: String::from("127.0.0.1"),
            port: 5037,
            serial_no: "emulator-5554".to_string()
        };
        let resp =conn.get_packages(&"".to_string());
        println!("{:?}", resp)
    }

    #[test]
    fn get_features() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let mut conn = DeviceClientImpl{
            host: String::from("127.0.0.1"),
            port: 5037,
            serial_no: "emulator-5554".to_string()
        };
        let resp =conn.get_features();
        println!("========= {:#?}", resp)
    }

    #[test]
    fn get_properties() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let mut conn = DeviceClientImpl{
            host: String::from("127.0.0.1"),
            port: 5037,
            serial_no: "emulator-5554".to_string()
        };
        let resp =conn.get_properties(&"".to_string());
        println!("========= {:#?}", resp)
    }

    #[test]
    fn logcat() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let mut conn = DeviceClientImpl{
            host: String::from("127.0.0.1"),
            port: 5037,
            serial_no: "emulator-5554".to_string()
        };
        let log_consumer = |entry:LogEntry|{
            println!("========= {:#?}", String::from_utf8_lossy(&*entry.log))
        };

        let error_handler = |err:AdbError|{
            println!("AdbError {:#?}", err)
        };

        let resp =conn.logcat(&"".to_string(),log_consumer,error_handler);
        println!("========= {:#?}", resp);
        thread::sleep(Duration::from_secs(2000));
    }
}