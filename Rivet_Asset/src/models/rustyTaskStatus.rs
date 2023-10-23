use serde::{Serialize, Deserialize};
//enums need to use the serde_repr crate and then we need to add the repr(u8) directive so it converts them into a int correctly
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Debug,Clone, Serialize_repr,Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum rustyTaskStatus
{
    Pending = 0,
    Tasked = 1,
    Running = 2,
    Complete = 3,
    FailedWithWarnings = 4,
    CompleteWithErrors = 5,
    Failed = 6,
    Cancelled = 7,
    NONE
}