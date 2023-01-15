mod read;
mod command;

use std::env;
use read::Config;
use std::process;

fn main() {
    let cfg = Config::create(env::args()).unwrap_or_else(|e| {
        eprintln!("Error creating configuration: {}", e);
        process::exit(1);
    });
    let str_content = read::get_contents(&cfg.location).unwrap_or_else(|e| {
        eprintln!("Error reading file: {}", e);
        process::exit(1);
    });
    let commands = read::split_by_comment(str_content, &cfg.location).unwrap();
    let _ = match command::layout_and_execute_commands(commands, cfg.location.as_str()){
        Ok(_) => process::exit(0),
        Err(e) => {println!("processfailedwitherror: {}", e); process::exit(1)}
    };
}
