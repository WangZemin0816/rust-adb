use log::{info};
use std::net::TcpStream;

use crate::adb::{Device, DeviceWithPath, HostServer};
use crate::adb_device::{new_device_client, DeviceClient};
use crate::adb_host::command::{
    new_host_disconnect_command, new_host_kill_command, new_host_list_device_command,
    new_host_transport_command, new_host_version_command,
};
use crate::adb_host::command::{
    new_host_list_device_l_command, SyncHostCommand,AsyncHostCommand
};
use crate::adb_host::protocol::{AsyncProtocol, SyncProtocol};
use crate::conn::connection::{connect, ConnectionInfo};
use crate::error::adb::AdbError;

pub struct AdbClient {
    pub host: String,
    pub port: i32,
    pub adb_bin_path: String,
}

impl HostServer for AdbClient {
    fn get_connection(&mut self) -> Result<TcpStream, AdbError> {
        connect(&ConnectionInfo {
            host: self.host.clone(),
            port: self.port.clone(),
            read_timeout_mills: 1000,
            write_timeout_mills: 1000,
        })
    }

    fn get_version(&mut self) -> Result<String, AdbError> {
        let mut command = new_host_version_command(self.host.clone(), self.port.clone());
        match command.execute() {
            Ok(response) => match response {
                SyncProtocol::OKAY { content, .. } => Ok(content),
                SyncProtocol::FAIL { content, .. } => {
                    Err(AdbError::ResponseStatusError { message: content })
                }
            },
            Err(error) => Err(error),
        }
    }

    fn disconnect(&mut self, host: String, port: i32) -> Result<(), AdbError> {
        let mut command =
            new_host_disconnect_command(self.host.clone(), self.port.clone(), host, port);
        match command.execute() {
            Ok(response) => match response {
                SyncProtocol::OKAY { content: _, .. } => Ok(()),
                SyncProtocol::FAIL { content, .. } => {
                    Err(AdbError::ResponseStatusError { message: content })
                }
            },
            Err(error) => Err(error),
        }
    }

    fn list_devices(&mut self) -> Result<Vec<Device>, AdbError> {
        let mut command = new_host_list_device_command(self.host.clone(), self.port.clone());
        match command.execute()? {
            SyncProtocol::OKAY { content, .. } => {
                let mut devices = vec![];
                for line in content.lines() {
                    let contents: Vec<&str> = line.trim().split_whitespace().collect();
                    if contents.len() >= 2 {
                        devices.push(Device {
                            serial_no: String::from(contents[0]),
                            status: String::from(contents[1]),
                        })
                    }
                }
                Ok(devices)
            }
            SyncProtocol::FAIL { content, .. } => {
                Err(AdbError::ResponseStatusError { message: content })
            }
        }
    }

    fn list_devices_with_path(&mut self) -> Result<Vec<DeviceWithPath>, AdbError> {
        let mut command = new_host_list_device_l_command(self.host.clone(), self.port.clone());
        match command.execute()? {
            SyncProtocol::OKAY { content, .. } => {
                let mut devices = vec![];
                for line in content.lines() {
                    let contents: Vec<&str> = line.trim().split_whitespace().collect();
                    if contents.len() >= 6 {
                        devices.push(DeviceWithPath {
                            serial_no: String::from(contents[0]),
                            status: String::from(contents[1]),
                            product: String::from(contents[2]),
                            model: String::from(contents[3]),
                            device: String::from(contents[4]),
                            transport_id: String::from(contents[5]),
                        });
                        continue;
                    }
                    info!("find adb line not contains 7 item: content={}", line)
                }
                Ok(devices)
            }
            SyncProtocol::FAIL { content, .. } => {
                Err(AdbError::ResponseStatusError { message: content })
            }
        }
    }

    fn track_devices(
        &mut self,
        _on_change: fn(Vec<Device>),
        _on_error: fn(AdbError),
    ) -> Result<String, AdbError> {
        Ok(String::from(""))
    }

    fn kill(&mut self) -> Result<(), AdbError> {
        let mut command = new_host_kill_command(self.host.clone(), self.port.clone());
        match command.execute() {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    fn get_device(&mut self, serial_no: String) -> Result<Box<dyn DeviceClient>, AdbError> {
        let mut command =
            new_host_transport_command(self.host.clone(), self.port.clone(), serial_no);
        match command.execute() {
            Ok(redirect_stream) => {
                match redirect_stream {
                    AsyncProtocol::OKAY { tcp_stream } => {Ok(new_device_client(tcp_stream))}
                    AsyncProtocol::FAIL { tcp_stream } => {Ok(new_device_client(tcp_stream))}
                }
            }

            Err(error) => Err(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::adb::client::AdbClient;
    use crate::adb::{HostServer};
    

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let mut client = AdbClient {
            host: String::from("127.0.0.1"),
            port: 5037,
            adb_bin_path: String::from(""),
        };
        println!("version: {:?}", client.get_version());
        match client.list_devices() {
            Ok(devices) => {
                for device in devices {
                    println!("devices: {:?}", device)
                }
            }
            Err(error) => {
                println!("{:?}", error)
            }
        }

        match client.list_devices_with_path() {
            Ok(devices) => {
                for device in devices {
                    println!("devices: {:?}", device)
                }
            }
            Err(error) => {
                println!("{:?}", error)
            }
        }

        match client.kill() {
            Ok(_) => {
                println!("kill success")
            }
            Err(error) => {
                println!("{:?}", error)
            }
        }

        match client.list_devices_with_path() {
            Ok(_devices) => {}
            Err(error) => {
                println!("{:?}", error)
            }
        }

        // println!("devices {}",client.list_devices());
    }
}
