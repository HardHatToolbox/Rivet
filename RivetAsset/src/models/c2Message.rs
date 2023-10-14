use serde::{Deserialize,Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct c2Message
{
    #[serde(rename = "PathMessage")]
    pub path_message: Vec<String>,
    #[serde(rename = "TaskData")]
    pub task_data: String,
}