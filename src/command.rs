use std::process::Command;

pub fn test() {
    let test = Command::new("echo").arg("test").spawn().expect("There was an error running the command");
    println!("{:?}", test.stdout);
}