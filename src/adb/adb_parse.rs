// use std::error::Error;
// use std::fmt::Error;
//
// const CMD_PREFIX: &str = "000C";
// const OK_RESP_PREFIX: &str = "000C";
// const ERR_RESP_PREFIX: &str = "000C";
//
//
// fn parse_command(command_body: String) -> Vec<u8> {
//     CMD_PREFIX + command_body
// }
//
// fn parse_response(response: Vec<u8>) -> Result<String, dyn Error> {
//     if OK_RESP_PREFIX.len()>response.len() || ERR_RESP_PREFIX.len()>response.len(){
//         Err(Error{})
//     }
//     let response_str = String::from_utf8(response);
//      match response_str {
//          Ok(response)=>{
//
//          }
//      }
//
// }