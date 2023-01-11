// TODO Construct git commands dynamically based upon git cards
// Implement task runners via threads to construct based upon x command in the git card

use std::process::Command;

use crate::read::CommandType;


pub fn test() {
    let test = Command::new("echo").arg("test").spawn().expect("There was an error running the command");
    println!("{:?}", test.stdout);

}

fn construct_arg_comm(args : &Vec<CommandType>) -> Vec<&str> {
    let mut git_commands: Vec<&str> = Vec::new();
    for arg in args {
        git_commands.push(match convert_to_git_comm(arg) {
            Some(x) => x,
            None => panic!("One of the git commands could not be parsed, please check your git card history!")
        })
    }
    git_commands
}

fn convert_to_git_comm(type_of_comm : &CommandType) -> Option<&str> {
    let commands_compiled: Vec<&str>;
    match type_of_comm {
        CommandType::C0 => return Some("commit"),
        CommandType::C1 => return Some("issue"),
        CommandType::C2 => return Some("milestone"),
        _ => return None
    }
}