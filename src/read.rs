use std::env::Args;
use std::fs;
use std::io;

const EXPECTED_LENGTH_OF_ARGS:u8 = 2;
pub struct Config {
    pub location: String
}

impl Config {
    pub fn create(args : Args) -> Result<Config, &'static str> {
        let mut parsed:Vec<String> = Vec::new();
        let mut length:u8 = 0;
        for arg in args {
            parsed.push(arg);
            length+=1;
            if length == EXPECTED_LENGTH_OF_ARGS {
                let location:String=parsed[1].clone();
                return Ok(Config {
                    location,
                })
            }
        }
        return Err("There are too few arguments in the command line, please try again.");
    }
}
pub fn get_contents(path: &String) -> io::Result<String> {
    fs::read_to_string(path)
}