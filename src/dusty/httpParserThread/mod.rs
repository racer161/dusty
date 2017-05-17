use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

use std::thread;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::mpsc::*;
use dusty::*;



pub fn spawnNewParserThread(streamReceiver : Receiver<TcpStream>, gathererManagerChannel : Sender<connection::request>, cache: Arc<Mutex<cacheManager::cache>>)
{    
    thread::spawn(move || 
    {
        
        let (localSend, localRecv) = channel::<connection::response>();
        loop
        {
            //recieve the latest stream
            match streamReceiver.recv()
            {
                Ok(mut stream) => 
                {
                    //read the stream until the request end line by line
                    let (schedule, headerTemplate) = httpParser::createRequestSchedule(&stream,localSend.clone(), cache.clone());
                    
                    //count the number of requests on this http request
                    let mut requestCount = schedule.len();
                    
                    //loop through the requests
                    for req in schedule
                    {
                        //Feed to thread pool LOL
                        gathererManagerChannel.send(req);
                    }
                    
                    //wait for all the requests to come in 
                    loop
                    {
                        //receive the next processed response from the gatherer threads
                        match localRecv.recv()
                        {
                            Ok(response) =>
                            {
                                //print!("{}", response.header.as_str());
                                //print!("{}", response.body.as_str());
                                
                                //write the header of the response to the TcpStream
                                stream.write(response.header.as_bytes()).expect("Error Writing to Socket!");
                                
                                //write the response to the TcpStream stream
                                stream.write(response.body.as_bytes()).expect("Error Writing to Socket!");
                                requestCount-=1;
                                if requestCount == 0 {break;}
                            },
                            Err(e) => println!("Error recieving gatherer responses! : {} ", e)
                        }
                    }
                    
                }
                Err(e) => println!("Error in Gatherer Thread: {} ", e)
            }    
        }
    });
    
}

//Gatherer threads access the cache manager and send the responses back to the http socket holding thread
pub fn spawnNewGathererThread(requestReceiver : Receiver<connection::request>, cache: Arc<Mutex<cacheManager::cache>>) //Creates a new thread with the given request Receiver
{
    thread::spawn(move || 
    {
        loop
        {
            // wait for a request to come in
            match requestReceiver.recv()
            {
                Ok(request) => 
                {
                    //complete is a convenience method that calls the appropriate http file parsing and cache manager methods
                    //it also sends the request back to the requests httpParser parent thread via a channel contained in the request
                    request.complete(cache.clone())
                }
                Err(e) => println!("Error in Gatherer Thread: {} ", e)
            }    
        }
    });
}

//Thread model one
//Two independent worker pools controlled by two independent managers
pub fn createIndependentThreadPools(parserThreadPoolSize: usize, gathererThreadPoolSize: usize , cache: Arc<Mutex<cacheManager::cache>>) -> (Vec<Sender<TcpStream>>,Sender<connection::request>)
{
    //These are the meat and potatos of dusty
    //They are simply a vec pointing to channels 
    //These in turn point to dusty's custom thread pool
    let mut gathererChannels : Vec<Sender<connection::request>> = Vec::new();
    let mut parserChannels : Vec<Sender<TcpStream>> = Vec::new();
    
    //create Gatherer Threads
    for i in 0..gathererThreadPoolSize //Spawn gatherer threads
    {
        let (send, recv) = channel::<connection::request>();
        gathererChannels.push(send);
        spawnNewGathererThread(recv, cache.clone());
    }
    
    //Initialize the gatherer manager which holds the pointer to the vec of gatherer threads
    let gathererManager : Sender<connection::request> = spawnGathererManagerThread(gathererChannels);
    
    //intialize the parser threads which are given a channel to the gathererManager 
    for i in 0..parserThreadPoolSize
    {
        let (send, rec) = channel::<TcpStream>();
        parserChannels.push(send);
        spawnNewParserThread(rec,gathererManager.clone(), cache.clone());
    }
    
    //return the gatherer manager channel to ensure it lives to the end of the program
    return (parserChannels, gathererManager)
    
}

//This function is only used by thread model one
pub fn spawnGathererManagerThread(channels : Vec<Sender<connection::request>>) -> Sender<connection::request>
{
    let (send, recv) = channel::<connection::request>();
    thread::spawn(move || 
    {
        let mut channelIndex : usize = 0;
        loop
        {
            match recv.recv()
            {
                Ok(req) => 
                {
                    //TO DO 
                    //Implement thread load balancing
                    //flip to the next thread and send it to its queue
                    channels[channelIndex].send(req);
                    channelIndex+=1;
                    
                    if channelIndex >= channels.len() { channelIndex = 0; }
                }
                Err(e) => println!("Error in Gatherer Thread: {} ", e)
            }    
        }
    });
    return send
}