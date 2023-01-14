// TODO Construct git commands dynamically based upon git cards
// Implement task runners via threads to construct based upon x command in the git card

use std::process::Command;

/// This is our list of approved commands, at the minute, only has commit issue and milestone and will
/// be used as a way of checking the sanity for what the user has input.
const LIST_OF_COMMANDS:&'static [&str] = &["commit", "issue", "milestone"];


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

fn construct_arg_comm(args : &Vec<String>) -> Vec<&str> {
    let mut git_commands: Vec<&str> = Vec::new();
    for arg in args {
        git_commands.push(arg);
    }
    git_commands
}