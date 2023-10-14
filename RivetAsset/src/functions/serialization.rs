use serde::{Serialize, Deserialize};
use serde_json::{Result, Error};
use crate::models::*;
use crate::models::taskReturnTypes::*;

pub fn serialize<T: Serialize>(data: &T) -> Result<Vec<u8>>
{
    //debug_only_json_String(data);
    return serde_json::to_vec(data);
}


pub fn deserialize<'a, T: Deserialize<'a>>(s: &'a [u8]) -> Result<T>
{
   return serde_json::from_slice(s);
}

// #[cfg(debug_assertions)]
// fn debug_only_json_String<T: Serialize>(data: &T)
// {
//     match serde_json::to_string(data)
//     {
//         Ok(json) => println!("JSON: {}", json),
//         Err(e) => println!("Error: {}", e),
//     }
// }

#[derive(Serialize, Deserialize)]
pub enum SerialTypes
{
    RustyTaskResult(rustyTaskResult::rustyTaskResult),
    C2message(c2Message::c2Message),
    RustyTask(rustyTask::rustyTask),
}

#[macro_export]
macro_rules! try_serialize {
    ($e:expr) => {
        match serialize(&$e) {
            Ok(bytes) => bytes,
            Err(e) => {
                println!("Failed to serialize output: {:?}", e);
                return;
            },
        }
    };
}