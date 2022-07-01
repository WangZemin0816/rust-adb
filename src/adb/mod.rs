pub mod command;

pub trait AdbClient {
    fn host_version() -> String;
}
