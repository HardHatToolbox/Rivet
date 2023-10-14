use serde::{Deserialize, Serialize};

//this is mainly done so it mimics whats on the server side
#[derive(Serialize, Deserialize)]
pub struct stringMessage
{
    pub Message: String,
}

impl stringMessage
{
    pub fn new(message: String) -> Self
    {
        return Self
        {
            Message: message,
        }
    }
}