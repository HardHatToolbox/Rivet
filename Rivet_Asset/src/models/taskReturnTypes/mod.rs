pub mod messageData;
use messageData::stringMessage;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub enum OutputType
{
    StringMessage(stringMessage),
    ByteArray(Vec<u8>),
}