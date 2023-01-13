use std::env::Args;
use std::fs;
use std::io;
use std::io::Write;
use std::process;

const EXPECTED_LENGTH_OF_ARGS:u8 = 2;
const LIST_OF_COMMANDS:&'static [&str] = &["commit", "issue", "milestone"];

#[derive(Debug)]
pub enum CommandType {
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
pub struct CardCommand {
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
pub fn split_by_comment(content: String, filename: &String) -> std::io::Result<()> {
    let mut cleansed:Vec<String> = Vec::new();
    let mut remove_card = fs::OpenOptions::new().write(true).truncate(true).open(filename)?;
    for indv in content.lines() {
        let handle = indv.trim();
        if handle.starts_with("###") {
            let mut linespl = handle.split("###");
            linespl.next();
            cleansed.push(match linespl.next() {
                Some(t) => t,
                None => "",
            }.trim().to_string());
        } else {
            writeln!(remove_card, "{}", indv)?;
        }
    }
    let x = commands(cleansed).unwrap_or_else(|e| {
        println!("The command interpreter failed due to: {}", e);
        process::exit(1);
    });
    println!("{:?}", print_out(&x)); // TODO implement here
    Ok(())
}

fn commands(mut command_arr: Vec<String>) -> Result<Vec<CardCommand>, &'static str> {
    let mut commands_and_contents:Vec<CardCommand> = Vec::new();
    for actionable_line in command_arr.iter_mut() {
        if actionable_line.starts_with("@") {
            actionable_line.remove(0);
            let mut parts = actionable_line.splitn(2, ' ');
            let ccentry: CardCommand = CardCommand {
                command_type: commandtype(match parts.next() {
                    Some(t)=> t,
                    None => ""
                }).unwrap_or_else(|_|{
                    return CommandType::Invalid;
                }),
                query_content: match parts.next() {
                    Some(t) => t,
                    None => "",
                }.to_string(),
            };
            commands_and_contents.push(ccentry);
        } else {
            let latest_command = match commands_and_contents.last_mut() {
                Some(vec) => vec,
                None => continue,
            };
            actionable_line.insert_str(0, " ");
            latest_command.query_content.push_str(actionable_line);
        }
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

fn print_out(cmds: &Vec<CardCommand>) {
    for i in cmds.iter() {
        println!("CardCommand action: {:?}", i.command_type);
        println!("Query/Content {}", i.query_content);
    }
}