// use std::io::Error;
// use std::net::TcpStream;
//
// pub trait AdbClient {
//     fn host_version(&mut self) -> String;
// }
//
//
// pub struct AdbClientImpl {
//     connection_addr: String,
// }
//
// impl AdbClient for AdbClientImpl {
//     fn host_version(&mut self) -> String {
//         self.
//     }
//
//     fn ping_pong(ping: String) -> Result<String, Error> {
//         let stream = TcpStream::connect(connect_str)?;
//     }
// }