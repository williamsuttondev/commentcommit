use std::env::Args;
use std::fs;
use std::io;
use std::process;

/// This will be the expected amount of arguments of the cl interpreter,
/// it will essentially know when to exit the iterator/loop when commands are being interpreted.
const EXPECTED_LENGTH_OF_ARGS:u8 = 2;

#[derive(Debug)]
/// This is our custom struct/type that contains two strings, currently the command_type and query_content
/// we can use this struct for extensibility purposes if there was ever more information that needed to be known.
pub struct CardCommand {
    command_type: String,
    query_content: String
}

/// This config struct will be a simple way (currently) to store information about our read configuration,
/// this currently only contains the locatation + name of our file.
pub struct Config {
    pub location: String
}
impl Config {
    /// This function will be called on a Config objects, to construct the location and other key
    /// information for later use so the filestream can be closed a lot quicker making the file more
    /// available. (First port of call function)
    /// # Arguments
    /// `args` - The fs::Args type that will be everything the user puts into the command line when 
    /// running the application.
    pub fn create(args: Args) -> Result<Config, &'static str> {
        let mut parsed: Vec<String> = Vec::new();
        let expected_length = EXPECTED_LENGTH_OF_ARGS;
        let error_msg = "There are too few arguments in the command line, please try again.";
        for arg in args {
            parsed.push(arg);
            if parsed.len() == expected_length as usize {
                let location = &parsed[1];
                return Ok(Config { location: location.to_string() });
            }
        }
        Err(error_msg)
    }
}
/// Simply opens the file and reads its contents to a string.
/// # Arguments
/// `path` - The string path that points to the the file to be opened.
pub fn get_contents(path: &String) -> io::Result<String> {
    fs::read_to_string(path)
}
/// This function will split the read file contents depending on two specific conditions,
/// 1. The line begins with "###" when special chars have been trimmed from the start
/// 2. The line doesn't begin with "###".
/// These two conditions will be important, as the string line will be further split
/// if it begins with ###, once the contents of that string are split and have been added,
/// the lines beginning with ### will be removed.
/// # Arguments
/// `content` - The String containing all of the contents of a file.
/// `filename` - The perpetuated for use to identify which file the program needs to open to modify.
pub fn split_by_comment(content: String, filename: &String) -> std::io::Result<()> {
    let cleansed = content.lines()
        .filter(|line| line.trim().starts_with("###"))
        .map(|line| line.split("###").nth(1).unwrap_or("").trim().to_string())
        .collect::<Vec<String>>();
    let _ = std::fs::write(filename, content.lines()
        .filter(|line| !line.trim().starts_with("###"))
        .collect::<Vec<&str>>()
        .join("\n"));
    let x = commands_conversion(cleansed).unwrap_or_else(|_| {
        println!("The command interpreter failed");
        process::exit(1);
    });
    println!("{:?}", print_out(&x));
    Ok(())
}
/// This function is responsible for taking the String vector containing different actions and comments
/// and splitting them up into their organised contents for use in the git automation.
/// It will take lines beginning with @, remove the at and take the word next to it as a command arg for git.
/// If the line doesn't begin with an @ and there is no existing card commands inside vector, 
/// the line will be discarded (for now).
/// If the line doesn't begin with an @ and there is existing command in the vector, then the contents will be added
/// to the most recent vector. (How most docs work.)
/// # Arguments
/// `command_arr` - The array containing each line from the split_by_comment function.
fn commands_conversion(command_arr: Vec<String>) -> Result<Vec<CardCommand>, &'static str> {
    let commands_and_contents = command_arr
        .into_iter()
        .fold(
            Vec::new(),
            |mut commands, line| {
                if line.starts_with("@") {
                    let parts: Vec<&str> = line.splitn(2, ' ').collect();
                    let command_type = parts[0].trim_start_matches("@").to_string();
                    let query_content = match parts.get(1) {
                        Some(t) => t.to_string(),
                        None => "".to_string(),
                    };
                    commands.push(CardCommand { command_type, query_content });
                } else {
                    if let Some(latest_command) = commands.last_mut() {
                        latest_command.query_content += &(" ".to_string() + &line);
                    }
                }
                commands
            },
        );
    Ok(commands_and_contents)
}
/// Temp test func.
fn print_out(cmds: &Vec<CardCommand>) {
    for i in cmds.iter() {
        println!("CardCommand action: {}", i.command_type);
        println!("Query/Content {}", i.query_content);
    }
}