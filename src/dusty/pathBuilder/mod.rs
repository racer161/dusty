//PATH BUILDER

 use std::io;
 use std::io::prelude::*;
 use std::fs::File;
 use std::path::{Path,PathBuf};
 use std::env::*;

pub fn returnFullPath<'b>(localPath: &'b str) -> String
{
    match current_dir().unwrap().to_str()
    {
        Some(s) => 
        {
            
            let mut tempPath = PathBuf::new();
            tempPath.push(s);
            tempPath.push("public_html");
            let newPath : Vec<&str> = localPath.split("/").collect();
            for pt in newPath
            {
                tempPath.push(pt)
            }
            //tempPath.canonicalize();
            match tempPath.to_str()
            {
                Some(finalPath) => return finalPath.to_string(),
                _ => 
                    {
                        println!("Path Error");
                        return String::from(s);
                    }
            }
            
        }
        _ => panic!("Failed to access working directory! Maybe you should check the directory permissions.")
    }
    
}