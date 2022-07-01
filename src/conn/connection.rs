
use std::io;
use std::io::{Error, Read, Write};
use std::net::TcpStream;

pub trait Connection {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn read(&mut self,buf: &mut [u8]) -> Result<usize, Error>;
}


