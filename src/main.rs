mod read;

use std::env;
use read::Config;

fn main() {
    let cfg = match Config::create(env::args()) {
        Ok(configuration) => configuration,
        Err(e) => panic!("{}", e),
    };
    if read::check_valid(&cfg.location) {
        println!("The file exists!");
    } else {
        println!("The file does not exist!");
    }
}
