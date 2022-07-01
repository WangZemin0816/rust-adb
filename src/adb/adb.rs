use std::{io, thread};
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::time::Duration;



pub fn list_devices() -> u64 {
    let mut stream = TcpStream::connect("127.0.0.1:5037")
        .expect("Couldn't connect to the server...");
    stream.set_nonblocking(true).expect("set_nonblocking call failed");
    stream.write_all("000Chost:version".as_ref()).unwrap();
    stream.flush();
    thread::sleep(Duration::from_millis(1000));
    let mut buf = vec![];
    loop {
        match stream.read_to_end(&mut buf) {
            Ok(_) => break,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP

            }
            Err(e) => panic!("encountered IO error: {e}"),
        };
    };
    println!("bytes: {buf:?}");
    123
}