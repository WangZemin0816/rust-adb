pub mod command;

#[cfg(test)]
mod tests {
    use crate::adb_host::command::SyncHostCommand;
    use crate::conn::protocol::SyncProtocol;

    #[test]
    fn read_commands() {
    }
}
