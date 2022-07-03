
use std::io::{Error};

pub trait Connection {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn read(&mut self,buf: &mut Vec<u8>) -> Result<usize, Error>;
}


