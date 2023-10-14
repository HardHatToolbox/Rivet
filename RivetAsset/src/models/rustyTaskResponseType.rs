use serde::{Serialize, Deserialize};
//enums need to use the serde_repr crate and then we need to add the repr(u8) directive so it converts them into a int correctly
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Debug,Clone, Serialize_repr,Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum rustyTaskResponseType
{
    None = 0,
    String = 1,
    FileSystemItem = 2,
    ProcessItem = 3,
    HelpMenuItem = 4,
    TokenStoreItem = 5,
    DataChunk = 6

}