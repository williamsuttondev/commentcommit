// TODO Construct git commands dynamically based upon git cards
// Implement task runners via threads to construct based upon x command in the git card

use std::process::Command;

use crate::read::CommandType;

// Putting multiple in here temporarily, will be more later down the line
// Implement the return option type for later error handling
pub fn execute_git_comm() /* -> Option<bool> */ {
    // imp unwrap_or_else for error handling
    // don't use multiple vars for our case but its fine...
    // args takes an array of str slices or literals
    Command::new("git").args(["add", "test-track/updateme.txt"]).status()
                                                    .expect("There was an error running git add!");
    let result_commit = Command::new("git").args(["commit", "-m Disregard - Testing commit functionality of software"]).output()
                                                    .expect("There was an error running git commit!");
    let result_push = Command::new("git").arg("push").output()
                                                    .expect("There was an error running git push!");
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