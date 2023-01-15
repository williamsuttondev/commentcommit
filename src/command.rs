// TODO Construct git commands dynamically based upon git cards
// Implement task runners via threads to construct based upon x command in the git card

use std::process::Command;

use crate::read;

pub fn layout_and_execute_commands(commands : Vec<read::CardCommand>, req_file : &str) -> Result<&str, &str> {
    // Multiple implementation for different git command types will be here
    let _ = Command::new("git").args(["add", req_file]).status();
    let commit_command:Vec<read::CardCommand> = commands.into_iter().filter(|x| x.command_type == "commit").collect();
    let commit_command_len = commit_command.len();
    if commit_command_len == 1 {
        execute_git_comm(commit_command.first().unwrap());
    } else {
        return Err("You have entered too many or no commit messages");
    }
    match Command::new("git").arg("push").status() {
        Ok(status) => {
            if status.success() {
                return Ok("Git commands successfully ran!");
            } else {
                return Err("There was an error running git push command!");
            }
        },
        Err(_) => return Err("There wan error constructing the command!"),
    }
}

fn execute_git_comm(curr_command : &read::CardCommand) {
    if curr_command.command_type == "commit" {
        let _ = Command::new("git").args(["commit", format!("{}{}", "-m", curr_command.command_type.as_str()).as_str()]).status();
    }
}