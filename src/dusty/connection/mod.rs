//CONNECTION STRUCT
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::*;
use std::thread;


use std::io::*;
use std::mem;
use dusty::*;

pub struct request
{
    pub filePath : String,
    pub size : usize,
    pub streamChannel : Sender<response>
}

impl request
{
    pub fn new(filePath : String, size: usize, streamChannel : Sender<response>) -> request
    {
        request{filePath : filePath, size: size, streamChannel : streamChannel }
    }
    
    pub fn complete(&self, cache: Arc<Mutex<cacheManager::cache>>)
    {
        //create a new response object that will be fed back to the TcpStream
        
        
        
        //Send the response to the TcpStream
        self.streamChannel.send(httpParser::processFileRequest(self.filePath.as_str(), cache.clone(), self.size)).expect("Thread Channel Error!!");
    }
}

pub struct response
{
    pub body : String,
    pub header : String
}

impl response
{
    pub fn new(header: String, body: String)-> response
    {
        response{ header : header, body : body}
        
    }  
}

pub struct con
{
    pub buffer: String,
    pub stream: TcpStream
}

impl con
{
    pub fn new(buffer : String, stream : TcpStream) -> con
    {
        con{buffer: buffer, stream : stream }
    }
    
}

