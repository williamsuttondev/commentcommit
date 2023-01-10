// TODO Construct git commands dynamically based upon git cards
// Implement task runners via threads to construct based upon x command in the git card

use std::process::Command;

pub fn test() {
    let test = Command::new("echo").arg("test").spawn().expect("There was an error running the command");
    println!("{:?}", test.stdout);
}