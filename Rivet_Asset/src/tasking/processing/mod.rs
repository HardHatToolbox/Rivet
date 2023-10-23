use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use crate::models::{rustyTask::rustyTask, rustyTaskResult::rustyTaskResult, rustyTaskResponseType::rustyTaskResponseType, rustyTaskStatus::rustyTaskStatus};
use crate::tasking::commands::CommandList;
use crate::tasking::commands::command_base;
use crate::models::taskReturnTypes::OutputType;
use dashmap::DashMap;
use crate::{METADATA, try_serialize};
use crate::functions::serialization::serialize;
use crate::functions::lockHelper::with_lock;
use base64::{Engine as _, engine::general_purpose};


pub struct taskProcessor
{
    // key is the task id and value is the whole task
    pub rustyTaskDic: DashMap<String, rustyTask>,
    // key is the task id and value is the whole task
    pub rustyTaskResultDic: DashMap<String, rustyTaskResult>,
    pub inbound_tasks: mpsc::Receiver<rustyTask>,
    pub  outbound_task_results: mpsc::Sender<rustyTaskResult>,
}

impl taskProcessor
{
    pub fn new(inbound_tasks: Receiver<rustyTask>, outbound_task_results: Sender<rustyTaskResult>) -> Self
    {
        return Self
        {
            rustyTaskDic: DashMap::new(),
            rustyTaskResultDic: DashMap::new(),
            inbound_tasks,
            outbound_task_results,
        }

    }

    pub fn handle_task(&self, rust_task: rustyTask, command_list: &CommandList)
    {
        println!("Handling task with id: {}", rust_task.id);
        //add the task to the task dictionary, use its id as the key
        self.rustyTaskDic.insert(rust_task.id.clone(), rust_task.clone()); // clone rust_task
        //need to make a rustyTaskResult item, check if we can find our command to run based on the task name
        let mut task_res = rustyTaskResult::new(rust_task.id.to_string(), rust_task.command.to_string(), String::new(), false, with_lock(&*METADATA, | metadata | metadata.id.to_string()) , rustyTaskStatus::Running, rustyTaskResponseType::None);
        println!("Looking for command: {}", &rust_task.command);
        if let Some(command) = command_list.command_list.get(&rust_task.command)
        {
            //add the task result to the task result dictionary, use its id as the key
            println!("Inserting task with id {} into task result dictionary", rust_task.id);
            self.rustyTaskResultDic.insert(rust_task.id.clone(), task_res.clone()); // clone task_res
            //run the command
            command.execute(rust_task, self);

        }
        else
        {
            println!("Unknown command: {}", &rust_task.command);
            task_res.response_type = rustyTaskResponseType::String;
            task_res.result = general_purpose::STANDARD.encode("Unknown command".as_bytes().to_vec());
            task_res.status = rustyTaskStatus::Failed;
            //todo add send task here
        }
    }

    //function so a running command can send a task result back to the server
    pub fn fill_result(&self, output: OutputType, task: rustyTask, task_status: rustyTaskStatus, task_response_type: rustyTaskResponseType)
    {
        println!("Filling and sending task result");
        //find the task result in the dictionary
        if let Some(mut task_res) = self.rustyTaskResultDic.get_mut(&task.id)
        {
            let mut serialized:  Vec<u8>;
            match output
            {
                OutputType::StringMessage(message) =>
                {
                    serialized = try_serialize!(message)
                },
                OutputType::ByteArray(byte_array) =>
                {
                    serialized = try_serialize!(byte_array)
                }
            }

            println!("Serialized: {:?}", serialized);
            task_res.result = general_purpose::STANDARD.encode(serialized);
            task_res.status = task_status;
            task_res.response_type = task_response_type;
            //place in the outbound queue of the HTTP_COMM
            match self.outbound_task_results.send(task_res.clone())
            {
                Ok(_) => println!("Sent task result"),
                Err(e) => println!("Failed to send task result: {:?}", e),
            }
        }
        else
        {
            println ! ("Failed to get task result from dictionary");
            //print the entries of the dictionary
            for entry in self.rustyTaskResultDic.iter()
            {
                println ! ("Key: {} Value: {:?}", entry.key(), entry.value());
            }
        }
    }
}