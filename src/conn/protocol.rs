use crate::conn::protocol::SyncProtocol::{FAIL, OKAY};
use crate::error::adb::AdbError;
use std::net::TcpStream;

pub enum SyncProtocol {
    OKAY { length: usize, content: String },
    FAIL { length: usize, content: String },
}

pub enum AsyncProtocol {
    OKAY { tcp_stream: TcpStream },
    FAIL { length: usize, content: String },
}

impl SyncProtocol {
    pub fn from_response(
        status: String,
        length: usize,
        content: String,
    ) -> Result<SyncProtocol, AdbError> {
        if status == "OKAY" {
            return Ok(OKAY { length, content });
        }
        if status == "FAIL" {
            return Ok(FAIL { length, content });
        }
        Err(AdbError::ResponseStatusError {
            message: String::from("unknown response status ") + &*status,
        })
    }
}