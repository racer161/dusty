mod dusty
{
    pub mod httpParser;
    pub mod pathBuilder;
    pub mod cacheManager;
    pub mod connection;
    pub mod httpParserThread;
}

use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::*;
use std::thread;

use std::io::*;
use std::mem;

use dusty::*;

fn main() 
{
    let parserThreadCount : usize = 16;
    let gathererThreadCount : usize = 16;
    let binding = "10.253.62.21:3000";
    
    let sharedCache : Arc<Mutex<cacheManager::cache>> = Arc::new(Mutex::new(cacheManager::cache::new()));
    let listener = TcpListener::bind(binding).unwrap();
    
    let (parserChannels, gathererManager) = httpParserThread::createIndependentThreadPools(parserThreadCount,gathererThreadCount,sharedCache);
    let mut parserIndex = 0;
    
    println!("Thread pools spawned server listening on {} ", binding);
    // accept connections and process them, spawning a new thread for each one
    for mut stream in listener.incoming() 
    {
        match stream 
        {
            Ok(mut stream) => 
            {
                        parserChannels[parserIndex].send(stream);
                        parserIndex+=1;
                        if parserIndex >= parserChannels.len() { parserIndex = 0; }
                
            }
            Err(e) => println!("TCP Stream Error: {} " , e)
        }
    }
}

/*
GET /index.html
/file1.html
/file2.html
/file3.html
/file4.html
 HTTP/1.1
User-Agent: httperf/0.9.1
Host: localhost
*/