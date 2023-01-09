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
    Invalid
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
    pub fn create(args: Args) -> Result<Config, &'static str> {
        let mut parsed: Vec<String> = Vec::new();
        let expected_length = EXPECTED_LENGTH_OF_ARGS;
        let error_msg = "There are too few arguments in the command line, please try again.";
        for arg in args {
            parsed.push(arg);
            if parsed.len() == expected_length as usize {
                let location = &parsed[1];
                return Ok(Config { location: location.to_string() });
            }
        }
        Err(error_msg)
    }
}
pub fn get_contents(path: &String) -> io::Result<String> {
    fs::read_to_string(path)
}
pub fn split_by_comment(content: String) {
    let mut cleansed: Vec<String> = Vec::new();
    for indv in content.split("###") {
        cleansed.push(indv.trim_start_matches("###").trim().to_string());
    }
    let x = commands(cleansed).unwrap_or_else(|e| {
        println!("The command interpreter failed due to: {}", e);
        process::exit(1);
    });
    println!("{:?}", print_out(&x));
}

fn commands(mut command_arr: Vec<String>) -> Result<Vec<Command>, &'static str> {
    let mut commands_and_contents:Vec<Command> = Vec::new();
    for actionable_line in command_arr.iter_mut() {
        if !actionable_line.starts_with("@") {
            continue;
        }
        actionable_line.remove(0);
        let parts:Vec<&str> = actionable_line.splitn(2, ' ').collect();
        let ccentry: Command = Command {
            command_type: commandtype(parts[0]).unwrap_or_else(|_|{
                return CommandType::Invalid;
            }),
            query_content: parts[1].to_string(),
        };
        commands_and_contents.push(ccentry);
    }
    Ok(commands_and_contents)
}

fn commandtype(cmd_str: &str) -> Result<CommandType, &'static str> {
    let command_term = match LIST_OF_COMMANDS.binary_search(&cmd_str) {
        Ok(cmd) => LIST_OF_COMMANDS[cmd].to_string(),
        Err(_) => "".to_string(),
    };
    if command_term=="commit" {
        Ok(CommandType::C0)
    } else if command_term=="issue" {
        Ok(CommandType::C1)
    } else if command_term == "milestone" {
        Ok(CommandType::C2)
    } else {
        Err("You have used an invalid command!")
    }
}

// temp func

fn print_out(cmds: &Vec<Command>) {
    let iterator = cmds.iter();
    for i in iterator {
        println!("Command action: {:?}", i.command_type);
        println!("Query/Content {}", i.query_content);
    }
}