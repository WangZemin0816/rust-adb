extern crate core;

mod adb_device;
mod adb_host;

pub mod client;
pub mod error;

pub fn new_adb_client(host: &String, port: i32, bin_path: String) -> Box<dyn AdbClient> {
    Box::new(AdbClientImpl {
        host: host.clone(),
        port: port.clone(),
        bin_path: bin_path.clone(),
    })
}
