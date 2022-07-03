use std::net::TcpStream;

mod client;

pub trait DeviceClient {}

pub struct DeviceClientImpl {}

impl DeviceClient for DeviceClientImpl {}

pub fn new_device_client(tcp_stream: TcpStream) -> Box<dyn DeviceClient> {
    Box::new(DeviceClientImpl{})
}
