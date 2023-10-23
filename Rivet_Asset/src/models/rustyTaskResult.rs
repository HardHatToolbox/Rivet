use serde::{Deserialize,Serialize};
use crate::models::rustyTaskResponseType::rustyTaskResponseType;
use crate::models::rustyTaskStatus::rustyTaskStatus;

#[derive(Debug,Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct rustyTaskResult
{
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Command")]
    pub command: String,
    #[serde(rename = "Result")]
    pub result: String, // is actually a byte array
    #[serde(rename = "IsHidden")]
    pub is_hidden: bool,
    #[serde(rename = "ImplantId")]
    pub implant_id: String ,
    #[serde(rename = "Status")]
    pub status: rustyTaskStatus,
    #[serde(rename = "ResponseType")]
    pub response_type: rustyTaskResponseType,
}

impl rustyTaskResult
{
    pub fn new(id: String, command: String, result: String, is_hidden: bool, implant_id: String, status: rustyTaskStatus, response_type: rustyTaskResponseType) -> Self
    {
        return Self
        {
            id,
            command,
            result,
            is_hidden,
            implant_id,
            status,
            response_type,
        }
    }
}
