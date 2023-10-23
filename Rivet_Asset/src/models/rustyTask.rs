use serde::{Deserialize,Serialize};
use std::collections::HashMap;
use clap::command;

#[derive( Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct rustyTask
{
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Command")]
    pub command: String,
    #[serde(rename = "Arguments")]
    pub arguments: HashMap<String, String>,
    #[serde(rename = "File")]
    pub file: String, //is sent as a base64 string but really is a byte array
    #[serde(rename = "IsBlocking")]
    pub is_blocking: bool,
}

impl rustyTask
{
    pub fn new(id:String, command: String, arguments: HashMap<String, String>, file: String, is_blocking: bool) -> Self
    {
        return Self
        {
            id,
            command,
            arguments,
            file,
            is_blocking,
        }
    }
}