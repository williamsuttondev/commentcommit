mod read;

use std::env;
use read::Config;

fn main() {
    let cfg = match Config::create(env::args()) {
        Ok(configuration) => configuration,
        Err(e) => panic!("{}", e),
    };
    let str_content = match read::get_contents(&cfg.location) {
        Ok(str) => str,
        Err(e) => panic!("e"),
    };
    println!("{}", str_content);
}
