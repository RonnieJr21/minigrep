use std::{env, error,fs, process};
use minigrep::Config;
fn main() {
   let args: Vec<String> = env::args().collect();

   let config = Config::build(&args).unwrap_or_else(|err| {
    println!("Error with arguments: {}",err);
    process::exit(1);
   });
   println!("Searching for {} at the location {}", config.query, config.file_path);


   if let Err(e) = minigrep::run(config){
    println!("Application error: {e}");
    process::exit(1);
   };




}
