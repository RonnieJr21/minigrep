use std::{error,fs};



pub fn run(config:Config)-> Result<(), Box<dyn error::Error>>{
   
    let query_results = fs::read_to_string(config.file_path)?;
 
    println!("Results: \n{query_results}");

    Ok(())
}

pub struct Config{
    pub query: String,
    pub file_path:String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3{
           return Err("Please provide two arguments following a double dash \nEx: \"cargo run -- arg1 arg2\"")
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {query, file_path})
    }
}