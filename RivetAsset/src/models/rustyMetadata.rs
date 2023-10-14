use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
//this converts in the incoming json to camel case so its the rust standard
#[serde(rename_all = "camelCase")]
pub struct rustyMetadata
{
    //these rename attributes are used to convert the json to snake case and back again
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Hostname")]
    pub hostname: String,
    #[serde(rename = "Address")]
    pub address: String,
    #[serde(rename = "Username")]
    pub username: String,
    #[serde(rename = "ProcessName")]
    pub process_name: String,
    #[serde(rename = "ProcessId")]
    pub process_id: i32,
    #[serde(rename = "Integrity")]
    pub integrity: String,
    #[serde(rename = "Arch")]
    pub arch: String,
    #[serde(rename = "ManagerName")]
    pub manager_name: String,
    #[serde(rename = "Sleep")]
    pub sleep: i32,
}

impl rustyMetadata
{
    pub fn new() -> Self
    {
        println!("Creating new metadata");
        let sleep =
            {
                let sleep_lock = crate::SLEEP_DURATION.lock().unwrap();
                *sleep_lock
            };
        return Self
        {
            id:  Uuid::new_v4().to_string(),
            hostname: "hostname-DEMO".to_string(),
            address: "127.0.0.1".to_string(),
            username: "User-DEMO".to_string(),
            process_name: "Rustineer-DEMO".to_string(),
            process_id: 0,
            integrity: "High-DEMO".to_string(),
            arch: "x64".to_string(),
            manager_name: "{{REPLACE_MANAGER_NAME}}".to_string(),
            sleep
        }
    }
}