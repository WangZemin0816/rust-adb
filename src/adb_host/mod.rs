pub mod command;
pub mod protocol;

#[cfg(test)]
mod tests {
    use crate::adb_host::command::new_host_version_command;
    use crate::adb_host::command::SyncHostCommand;
    use crate::adb_host::protocol::SyncProtocol;

    #[test]
    fn read_commands() {
        let _ = log4rs::init_file("log4rs.yml", Default::default());
        let mut command = new_host_version_command(String::from("127.0.0.1"), 5037);
        let resp = command.execute().unwrap();
        match resp {
            SyncProtocol::OKAY { content, .. } => {
                println!("adb version {}", content)
            }
            SyncProtocol::FAIL { content, .. } => {
                println!("adb version {}", content)
            }
        }
    }
}
