use crate::adb_host::protocol::SyncProtocol::{FAIL, OKAY};
use crate::error::adb::AdbError;
use std::net::TcpStream;

pub enum SyncProtocol {
    OKAY { length: usize, content: String },
    FAIL { length: usize, content: String },
}

pub enum AsyncProtocol {
    OKAY { tcp_stream: TcpStream },
    FAIL { tcp_stream: TcpStream },
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
impl AsyncProtocol {
    pub fn from_response(status: String, tcp_stream: TcpStream) -> Result<AsyncProtocol, AdbError> {
        if status == "OKAY" {
            return Ok(AsyncProtocol::OKAY { tcp_stream });
        }
        if status == "FAIL" {
            return Ok(AsyncProtocol::FAIL { tcp_stream });
        }
        Err(AdbError::ResponseStatusError {
            message: String::from("unknown response status ") + &*status,
        })
    }
}
