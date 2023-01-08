use std::env::Args;


const EXPECTED_LENGTH_OF_ARGS:u8 = 2;
pub const SHORT_ERROR:&str = "0X0";
pub struct Config {
    location: String
}

impl Config {
    pub fn get_location(&self) -> &String {
        &self.location
    }
    pub fn create(args : Args) -> Config {
        let mut parsed:Vec<String> = Vec::new();
        let mut length:u8 = 0;
        for arg in args {
            parsed.push(arg);
            length+=1;
            if length == EXPECTED_LENGTH_OF_ARGS {
                let location:String=parsed[1].clone();
                return Config {
                    location
                };
            }
        }
        let location: String = SHORT_ERROR.to_string();
        return Config {
            location,
        }
    }
}

pub fn read_file() {
    
}