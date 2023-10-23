pub mod whoami;
use std::collections::HashMap;
use crate::models::{rustyTask::rustyTask,rustyTaskResult::rustyTaskResult};
use crate::tasking::commands::whoami::Whoami;
use crate::tasking::processing::taskProcessor;


#[enum_delegate::register]
pub trait command_base
{
    fn execute(&self, task: rustyTask, task_processor: &taskProcessor);
}

#[enum_delegate::implement(command_base)]
pub enum Commands
{
    whoami(Whoami),
}

pub struct CommandList
{
    pub command_list: HashMap<String,Commands>,
}

impl CommandList
{
    pub fn Init() -> Self
    {
        let mut temp_command_list = HashMap::new();
        temp_command_list.insert("whoami".to_string(), Commands::whoami(Whoami::new()));
        return Self
        {
            command_list: temp_command_list,
        }

    }
}