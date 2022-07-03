// use std::fmt::format;
// use std::io::{Read, Write};
// use std::net::{Shutdown, TcpStream};
// use std::num::ParseIntError;
// use std::thread;
// use std::thread::JoinHandle;
// use std::time::Duration;
// use log::{info, trace};
// use crate::error::adb::AdbError;
//
// pub struct AdbAsyncConnection {
//     pub connection_str: String,
//     pub read_timeout_mills: u64,
//     pub write_timeout_mills: u64,
// }
//
// impl AdbAsyncConnection {
//     pub fn exec_command_async(&mut self, command: String, consumer: fn(&String) -> Result<(), AdbError>, error_handler: fn(&AdbError)) -> JoinHandle<()> {
//         let mut adb_command = AdbSyncConnection {
//             connection_str: self.connection_str.clone(),
//             read_timeout_mills: self.read_timeout_mills.clone(),
//             write_timeout_mills: self.write_timeout_mills.clone(),
//         };
//         trace!("begin to exec command async command {}",command.clone());
//         thread::spawn(move || {
//             trace!("async thread start for command {}",command.clone());
//             let mut tcp_stream = match adb_command.connect() {
//                 Ok(stream) => {
//                     stream
//                 }
//                 Err(error) => {
//                     trace!("async thread stop for command {} during connect due to {}",command.clone(),error);
//                     error_handler(&error);
//                     return ();
//                 }
//             };
//             match adb_command.write_command(&mut tcp_stream, &command) {
//                 Ok(_) => {
//                     ()
//                 }
//                 Err(error) => {
//                     trace!("async thread stop for command {} during write due to {}",command.clone(),&error);
//                     error_handler(&error);
//                     return ();
//                 }
//             };
//             loop {
//                 match adb_command.read_response_content(&mut tcp_stream,10) {
//                     Ok(data) => {
//                         match consumer(&data) {
//                             Ok(_) => {}
//                             Err(error) => {
//                                 trace!("async thread stop for command {} during consume due to {}",command.clone(),error);
//                                 return ();
//                             }
//                         }
//                     }
//                     Err(error) => {
//                         trace!("async thread stop for command {} during read response due to {}",command.clone(),error);
//                         error_handler(&error);
//                         return ();
//                     }
//                 }
//             }
//         })
//     }
// }
//
//
// pub struct AdbSyncConnection {
//     pub connection_str: String,
//     pub read_timeout_mills: u64,
//     pub write_timeout_mills: u64,
// }
//
// impl AdbSyncConnection {
//     pub fn exec_command(&mut self, command: String) -> Result<String, AdbError> {
//         let mut tcp_stream = self.connect()?;
//         self.init_connect_timeout(&mut tcp_stream)?;
//         self.write_command(&mut tcp_stream, &command)?;
//
//         self.read_response_status(&mut tcp_stream)?;
//         let content_length = self.read_response_length(&mut tcp_stream)?;
//
//         let response = self.read_response_content(&mut tcp_stream, content_length);
//         let _ = tcp_stream.shutdown(Shutdown::Both);
//         return response
//     }
//
//     fn connect(&mut self) -> Result<TcpStream, AdbError> {
//         match TcpStream::connect(&self.connection_str) {
//             Ok(tcp_stream) => {
//                 Ok(tcp_stream)
//             }
//             Err(error) => {
//                 Err(AdbError::TcpConnectError { source: Box::new(error) })
//             }
//         }
//     }
//
//     fn init_connect_timeout(&mut self, tcp_stream: &mut TcpStream) -> Result<(), AdbError> {
//         match tcp_stream.set_read_timeout(Option::from(Duration::from_millis(self.read_timeout_mills))) {
//             Ok(_) => {}
//             Err(error) => {
//                 info!("init connect read timeout failed");
//                 return Err(AdbError::TcpReadError {
//                     source: Box::new(error)
//                 });
//             }
//         };
//         match tcp_stream.set_write_timeout(Option::from(Duration::from_millis(self.read_timeout_mills))) {
//             Ok(_) => {}
//             Err(error) => {
//                 info!("init connect write timeout failed");
//                 return Err(AdbError::TcpReadError {
//                     source: Box::new(error)
//                 });
//             }
//         };
//         Ok(())
//     }
//
//
//     fn write_command(&mut self, tcp_stream: &mut TcpStream, command: &String) -> Result<(), AdbError> {
//         let full_command = self.add_command_length_prefix(command.clone());
//         trace!("format final command info is {}",full_command.clone());
//         match tcp_stream.write_all(full_command.as_ref()) {
//             Ok(_) => {
//                 Ok(())
//             }
//             Err(error) => {
//                 Err(AdbError::TcpWriteError { source: Box::new(error) })
//             }
//         }
//     }
//
//     fn read_response_content(&mut self, tcp_stream: &mut TcpStream, length: usize) -> Result<String, AdbError> {
//         let mut response_content = vec![0; length];
//         trace!("begin read command content from stream: length={}",&length);
//         match tcp_stream.read_exact(&mut response_content) {
//             Ok(_) => {}
//             Err(error) => {
//                 trace!("read command content from stream failed: error={}",&error);
//                 return Err(AdbError::TcpReadError { source: Box::new(error) });
//             }
//         };
//
//         match String::from_utf8(Vec::from(response_content)) {
//             Ok(content_string) => {
//                 trace!("read command content from stream success: content={}",&content_string);
//                 Ok(content_string)
//             }
//             Err(error) => {
//                 trace!("parse command content to utf-8 failed: error={}",&error);
//                 return Err(AdbError::ParseResponseError { source: Box::new(error) });
//             }
//         }
//     }
//
//     fn read_response_length(&mut self, tcp_stream: &mut TcpStream) -> Result<usize, AdbError> {
//         let mut content_length = [0; 4];
//         match tcp_stream.read_exact(&mut content_length) {
//             Ok(_) => {}
//             Err(error) => {
//                 trace!("read command content length from stream failed: error={:?}",&error);
//                 return Err(AdbError::TcpReadError { source: Box::new(error) });
//             }
//         }
//         match String::from_utf8(Vec::from(content_length)) {
//             Ok(response) => {
//                 trace!("read command content length success: length={}",&response);
//                 match usize::from_str_radix(&*response, 16) {
//                     Ok(size) => {
//                         trace!("parse command content length success: length={}",&size);
//                         Ok(size)
//                     }
//                     Err(error) => {
//                         trace!("parse command content length from hex to usize failed: length={}",&error);
//                         Err(AdbError::ParseResponseError { source: Box::new(error) })
//                     }
//                 }
//             }
//             Err(error) => {
//                 trace!("parse command content length to utf-8 string failed: error={}",&error);
//                 return Err(AdbError::ParseResponseError { source: Box::new(error) });
//             }
//         }
//     }
//
//     fn read_response_status(&mut self, tcp_stream: &mut TcpStream) -> Result<(), AdbError> {
//         let mut is_ok_buffer = [0; 4];
//         match tcp_stream.read_exact(&mut is_ok_buffer) {
//             Ok(_) => {}
//             Err(error) => {
//                 trace!("read command status from stream failed: error={:?}",&error);
//                 return Err(AdbError::TcpReadError { source: Box::new(error) });
//             }
//         }
//         match String::from_utf8(Vec::from(is_ok_buffer)) {
//             Ok(response_status) => {
//                 if response_status != "OKAY" {
//                     trace!("command response status is not ok: status={}",response_status.clone());
//                     return Err(AdbError::ResponseStatusError { message: response_status });
//                 }
//                 trace!("read okay command status: status={}",response_status.clone());
//                 Ok(())
//             }
//             Err(error) => {
//                 trace!("parse response length to utf-8 failed: err={}",error);
//                 Err(AdbError::ParseResponseError { source: Box::new(error) })
//             }
//         }
//     }
//
//     fn add_command_length_prefix(&mut self, mut command_body: String) -> String {
//         let trim_command = command_body.trim();
//         let trim_command_length = format!("{:04X}", trim_command.len());
//         trim_command_length + trim_command
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use log::info;
//     use crate::cmd::connection::AdbSyncConnection;
//
//     #[test]
//     fn read_commands() {
//         let handler = log4rs::init_file("log4rs.yml", Default::default());
//         let mut command = AdbSyncConnection { connection_str: String::from("127.0.0.1:5037"), read_timeout_mills: 1, write_timeout_mills: 1 };
//         let resp = command.exec_command(String::from("host:version")).unwrap();
//         println!("adb command {}", resp);
//     }
// }
