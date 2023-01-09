use core::panic;
use std::env::Args;
use std::fs;
use std::io;
use std::process;

const EXPECTED_LENGTH_OF_ARGS:u8 = 2;
const LIST_OF_COMMANDS:&'static [&str] = &["commit", "issue", "milestone"];

#[derive(Debug)]
enum CommandType {
    C0,
    C1,
    C2,
    // C3,
    // C4,
    // C5,
    // C6,
    // C7,
    // C8,
    // C9,
}

#[derive(Debug)]
pub struct Command {
    command_type: CommandType,
    query_content: String
}

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
    match commands(cleansed) {
        Ok(x) => println!("{:?}", x),
        Err(e) => {println!("The command interpreter failed due to: {}", e); process::exit(1);},
    }
}

fn commands(mut command_arr: Vec<String>) -> Result<Vec<Command>, &'static str> {
    let mut commands_and_contents:Vec<Command> = Vec::new();
    let mut accepted:bool = true;
    let contents_action = command_arr.iter_mut();
    for actionable_line in contents_action {
        if actionable_line.starts_with("@") {
            actionable_line.remove(0);
            let delimited:(&str, &str) = match actionable_line.split_once(" ") {
                Some(x) => x,
                None => {accepted = !accepted; break},
            };
            let ccentry: Command = Command {
                command_type: match commandtype(delimited.0) {
                    Ok(ct) => ct,
                    Err(e) => panic!("Something went wrong with the command type!: {}", e),
                },
                query_content: delimited.1.to_string(),
            };
            commands_and_contents.push(ccentry);
        }
    }
    if accepted {
        Ok(commands_and_contents)
    }
    else {
        Err("There was an error parsing the commands!")
    }
}

fn commandtype(cmd_str: &str) -> Result<CommandType, &'static str> {
    let command_term:String;
    match LIST_OF_COMMANDS.binary_search(&cmd_str) {
        Ok(cmd) => command_term = LIST_OF_COMMANDS[cmd].to_string(),
        Err(_) => command_term = "".to_string(),
    }
    match command_term.as_str() {
        "commit" => return Ok(CommandType::C0),
        "issue" => return Ok(CommandType::C1),
        "milestone" => return Ok(CommandType::C2),
        _ => return Err("You have used an invalid command!"),

    }
}