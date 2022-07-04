use crate::adb_device::DeviceService;

pub struct DeviceClient {
    pub host: String,
    pub port: i32,
    pub serial_no: String,
}

impl DeviceService for DeviceClient {

}