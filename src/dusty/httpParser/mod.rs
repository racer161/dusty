//HTTP PARSER
use std::sync::Arc;
use std::sync::Mutex;
use std::string::String; 
use dusty::*;

use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::*;
use std::thread;

use std::io::*;
use std::mem;

//the method called by the Parser thread
//I hope to speed this up with AVX2 parsing soon
//I've written a similar parser using AVX2 in C++ 
//Should be some basic copy and paste if intrinsics in Rust are decently implemented
pub fn createRequestSchedule<'a>(req: &TcpStream, streamChannel : Sender<connection::response>, cache: Arc<Mutex<cacheManager::cache>> ) -> (Vec<connection::request>, String) 
{
    let mut schedule : Vec<connection::request> = Vec::new();
    
    let mut buffer = BufReader::new(req);
    //println!("Started reading!");
    
    let mut lineBuffer = String::with_capacity(512);
    
    loop  //loop through the request backwards to get the header data first
    {
        lineBuffer.clear();
        match buffer.read_line(&mut lineBuffer)
        {
            Ok(size) =>
            {              
                match size
                {
                    //I wanted to break if it read nothing but that doesn't really work 
                    //due to the nature of bufferd reading
                    0 =>{},
                    _ => 
                    {
                        //println!("{}",lineBuffer.as_str());
                        match processRequestLine(lineBuffer.as_str(),streamChannel.clone(),cache.clone())
                        {
                            Some(newRequest) =>
                            {
                                //push the new request onto the schedule
                                schedule.push(newRequest);
                            }
                            None => 
                            {
                                //Read until we read a double new line which indicates the end of the header
                                if lineBuffer == "\r\n"
                                {
                                    //println!("Ok We broke!");
                                    break;
                                }
                            }
                        }
                        
                    }
                }              
            },
            Err(e) => println!("Error Reading to buffer in request!")
        }
    }
    return (schedule,String::from(" "));
}

//callBack to process each header
fn processHeader(header: &str, contentStart: usize)
{
    println!("Header : {}", &header[0..contentStart]);
    println!("Content : {}",&header[contentStart+1..header.len()]);
    println!();
}


//callback to process each file requested
pub fn processFileRequest<'a>(filePath: &'a str, cache: Arc<Mutex<cacheManager::cache>>, size : usize) -> connection::response //The method called by the gatherer thread
{ 
    match cache.lock()
    {
        Ok(mut guard) =>
        { 
            match size
            {
                0 => //the file doesn't exist in the cache so we need to read it in
                {
                    let (fileString, size) = cacheManager::readNewUncachedFile(&filePath);
                    match size//Try to read in the file to cache
                    {
                        0 => 
                        {
                            println!("HTTP 404 NOT FOUND");
                            return connection::response::new(String::from("HTTP 404 NOT FOUND\r\n\r\n"),String::from(" "))//the file couldn't be read or couldn't be found
                                                          //regardless of the reason we return File not found
                        }
                        _ => 
                        {
                            guard.insertNewCacheItem(pathBuilder::returnFullPath(&filePath).as_str(), fileString);
                            return connection::response::new(String::from("HTTP/1.1 200 OK\r\nServer: Dusty (Win64)\r\nConnection: Closed\r\n\r\n"),guard.read(pathBuilder::returnFullPath(&filePath).as_str()))
                        }
                    }

                },
                     //Its already in the cache so we'll just read it
                _ => return connection::response::new(String::from("HTTP/1.1 200 OK\r\nServer: Dusty (Win64)\r\nConnection: Closed\r\n\r\n"),guard.read(pathBuilder::returnFullPath(&filePath).as_str()))

            }
        },
        Err(e) => 
        {
            println!("Mutex poisoned! : {}", e );
            println!("HTTP 404 NOT FOUND");
            return connection::response::new(String::from("HTTP 404 NOT FOUND"),String::from(" "))//the file couldn't be read or couldn't be found
        }
    }
}

fn processRequestLine(line : &str, streamChannel : Sender<connection::response>, cache: Arc<Mutex<cacheManager::cache>> ) -> Option<connection::request>
{
    if line == "\r\n"
    {
        return None
    }
    match cache.lock()
    {
        Ok(mut guard) =>
        { 
            match line.find(':')//If this is a header line
            {
                Some(index) => 
                {
                    //processHeader(line,index);
                    return None;
                }
                None => 
                {
                    if line.contains("HTTP")// Finds the line with HTTP which is the last line of file requests
                    {

                        let mut oneLineRequest : Vec<&str> = line.split_whitespace().collect();
                        if oneLineRequest.len() > 1
                        {
                            let filePath = pathBuilder::returnFullPath(oneLineRequest[1]);
                            //println!("FilePath : {} ",filePath);
                            return Some(connection::request::new(filePath.clone(),guard.checkSize(&filePath.as_str()),streamChannel));
                        }
                        else
                        {
                            return None;
                        } 
                    } else //create a slice with the request
                    {
                        if line.contains("GET")
                        {
                            let mut oneLineRequest : Vec<&str> = line.split_whitespace().collect();
                            
                                let mut filePath = pathBuilder::returnFullPath(oneLineRequest[1]);
                                //filePath.remove(0);
                                //filePath.remove(0);
                                //println!("FilePath : {} ",filePath);
                                //println!("File: {}", &*filePath);
                                //create a new request and add it to the schedule
                                return Some(connection::request::new(filePath.clone(),guard.checkSize(&filePath.as_str()),streamChannel));
                        }else
                        {
                            line.trim();
                            let mut filePath = pathBuilder::returnFullPath(&line);
                            
                            //filePath.remove(0);
                            //filePath.remove(0);
                            //println!("FilePath : {} ",filePath);
                            //println!("File: {}", &*filePath);
                            //create a new request and add it to the schedule
                            return Some(connection::request::new(filePath.clone(),guard.checkSize(&filePath.as_str()),streamChannel));
                        }

                    }
                }
            }
        },
        Err(e) => 
        {
            panic!("Mutex poisoned! : {}", e );
            return None
        }
    }
}






fn convertVecStrToU8(responseBuffer : &Vec<&str>) -> Vec<u8>
{
    let mut str_as_u8 : Vec<u8> = Vec::new();
    for res in responseBuffer
    {
        str_as_u8.extend(res.as_bytes());
    }
    str_as_u8
}