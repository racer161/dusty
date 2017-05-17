
//CACHE MANAGER
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::fs::File;

pub struct cache
{
    c : HashMap<String, String>,
    fileSize : HashMap<String , usize>
}

impl cache
{

    pub fn new() -> cache
    {
        return cache
        {
            c : HashMap::new(),
            fileSize : HashMap::new()
        }

    }

    pub fn read(&self, path : &str) -> String
    {
        return self.c[path].clone();
    }


    //Checks to see if there is an entry for size in the file size HashMap
    //If it exists it returns the cached string size
    //If it doesn't exst it reads the file and adds the entry to the cache then returns the size
    //If it returns 0 the file is not found 
    pub fn checkSize(&mut self, path : &str) -> usize
    {
        match self.fileSize.contains_key(path) 
        {
            true => return self.fileSize[path],
            false => return 0//openFile(path)
        }
    }

    //1.
    pub fn insertNewCacheItem(&mut self , path : &str, FileStr: String) -> usize
    {
        self.c.insert(path.to_string(),FileStr.clone());
        self.fileSize.insert(path.to_string(),FileStr.len());
        FileStr.len()
    }

}

    //2.
    pub fn readNewUncachedFile(filename: &str) -> (String, usize)
    {
        let mut f = File::open(filename);
        match f
        {
                Ok(mut fl) => 
                {
                    //println!("FileName not in Error : {}", filename);
                    return readText(&mut fl, &filename)
                }
                Err(e) => 
                {
                    //println!("File Open Error: {} " , e);
                    //println!("FileName in Error : {}", filename);
                    (String::from(" "),70)
                }
        }
    }

    //3.
    fn readText(readable: &mut Read, filename: &str ) -> (String, usize)
    {
        let mut s = String::new();
        let lenRead = readable.read_to_string(&mut s);
            match lenRead 
            {
                    Ok(lR) => 
                    {
                        //println!("FileSize: {} " , lR);
                        //return self.insertNewCacheItem(filename, s);
                        (s,lR)
                    }
                    Err(e) => 
                    {
                        println!("File Read Error: {} " , e);
                        (String::from(" "),0)
                    }
            }
    }