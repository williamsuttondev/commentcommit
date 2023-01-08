mod read;

use std::env;
use std::fs;
use read::Config;

fn main() {
    let cfg: Config = Config::create(env::args());
    match cfg.get_location().as_str() {
        read::SHORT_ERROR => println!("There were too few arguments, please enter more arguments. (Error code {})", read::SHORT_ERROR),
        _ => read::read_file(),
    }
}
