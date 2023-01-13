mod read;
mod command;

use std::env;
use read::Config;
use std::process;

fn main() {
    // let cfg = Config::create(env::args()).unwrap_or_else(|e| {
    //     eprintln!("Error creating configuration: {}", e);
    //     process::exit(1);
    // });
    // let str_content = read::get_contents(&cfg.location).unwrap_or_else(|e| {
    //     eprintln!("Error reading file: {}", e);
    //     process::exit(1);
    // });
    // read::split_by_comment(str_content);
    command::execute_git_comm();
}
