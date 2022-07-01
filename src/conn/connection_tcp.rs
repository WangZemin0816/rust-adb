use std::io::{Error, Read, Write};
use std::net::TcpStream;
use crate::conn::connection::Connection;

pub fn new_connection(ipv4: String, port: u64, error_handler: fn(Error) -> (Error)) -> Result<TcpConnection, Error> {
    let connect_str = ipv4 + ":" + &*port.to_string();
    let stream = TcpStream::connect(connect_str)?;
    Ok(TcpConnection { stream, error_handler })
}


pub struct TcpConnection {
    stream: TcpStream,
    error_handler: fn(Error) -> (Error),
}

impl Connection for TcpConnection {
     fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        match self.stream.write(buf) {
            Ok(size) => {
                self.stream.flush()?;
                Ok(size)
            }
            Err(error) => {
                let handler = &self.error_handler;
                let handler_err = handler(error);
                Err(handler_err)
            }
        }
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        match self.stream.read(buf) {
            Ok(size) => { Ok(size) }
            Err(error) => {
                let handler = &self.error_handler;
                let handler_err = handler(error);
                Err(handler_err)
            }
        }
    }
}