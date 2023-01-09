use std::env::Args;
use std::fs;
use std::io;

const EXPECTED_LENGTH_OF_ARGS:u8 = 2;

enum CommandType {
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10
}

pub struct Config {
    pub location: String
}
pub struct Command {
    command_type: CommandType,
    queury_content: String
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
pub fn split_by_comment(content: String){
    let line = content.lines();
    let mut cleansed:Vec<String> = Vec::new();
    for indv in line {
        let handle = indv.trim();
        if handle.starts_with("###") {
            let linespl:Vec<&str> = handle.split("###").collect();
            cleansed.push(linespl[1].trim().to_string());
        }
    }
    println!("{:?}", cleansed);
    if !commands(cleansed) {
        panic!("There was an error executing one of the commands, please make sure you used valid syntax.");
    }
}

fn commands(mut command_arr: Vec<String>) -> bool {
    let contents_action = command_arr.iter_mut();
    for actionable_line in contents_action {
        if actionable_line.starts_with("@") {
            actionable_line.remove(0);
            let delimited:(&str, &str) = match actionable_line.split_once(" ") {
                Some(x) => x,
                None => panic!("You need to enter a command after the @!"),
            };
            println!("Command: {}, Query: {}",delimited.0, delimited.1);
        }
    }
    true
}