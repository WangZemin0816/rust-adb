// use crate::cmd::connection::AdbSyncConnection;
// use crate::cmd::AdbSyncCommand;
// use crate::error::adb::AdbError;
//
// pub struct AdbVersionCommand {
//     pub connection_str: String,
//     pub read_timeout_mills: u64,
//     pub write_timeout_mills: u64,
// }
//
// impl AdbSyncCommand for AdbVersionCommand {
//     fn execute(&mut self) -> Result<String, AdbError> {
//         let mut adb_command = AdbSyncConnection {
//             connection_str: self.connection_str.clone(),
//             read_timeout_mills: self.read_timeout_mills.clone(),
//             write_timeout_mills: self.write_timeout_mills.clone(),
//         };
//         adb_command.exec_command(String::from("host:version"))
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::cmd::adb_version::AdbVersionCommand;
//     use crate::cmd::AdbSyncCommand;
//
//     #[test]
//     fn read_commands() {
//         let handler = log4rs::init_file("log4rs.yml",Default::default());
//         let mut command = AdbVersionCommand {
//             connection_str: String::from("127.0.0.1:5037"),
//             read_timeout_mills:1000,
//             write_timeout_mills:1000
//         };
//         let resp = command.execute().unwrap();
//         println!("adb version {}", resp);
//     }
// }
