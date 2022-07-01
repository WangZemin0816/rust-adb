mod adb;
mod conn;

#[cfg(test)]
mod tests {
    use std::io::Error;
    use std::thread;
    use std::time::Duration;
    use crate::conn::connection::Connection;
    use crate::conn::connection_tcp::new_connection;

    #[test]
    fn it_works() {
        let print_err = |error| -> (Error){
            print!("eeeerrrrooorrr");
            error
        };

        let mut  connection =
            new_connection(String::from("127.0.0.1"), 5037, print_err).unwrap();
        connection.write("000Chost:version".as_ref()).unwrap();
        thread::sleep(Duration::from_millis(1000));
        let mut buffer = vec![];
        connection.read(&mut buffer).unwrap();
        let str_item = String::from_utf8(buffer).unwrap();


        println!("sssssssss{}",str_item)

    }
}
