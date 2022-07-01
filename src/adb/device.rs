
pub trait Device {
    fn get_serial_no(&self) -> String;
    fn get_packages(&self) -> Vec<String>;
}

fn new_device() {}

struct DeviceImpl {}

impl Device for DeviceImpl {
    fn get_serial_no(&self) -> String {}
    fn get_packages(&self) -> Vec<String> {}
}