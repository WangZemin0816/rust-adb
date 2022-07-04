use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use log::{debug, trace};
use crate::adb_host::{AsyncProtocol, SyncProtocol};

use crate::error::adb::AdbError;
