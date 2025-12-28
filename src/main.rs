use std::{
    collections::VecDeque,
    env::{self},
    fs::{File, read_to_string},
    io::{self, Write},
    path::Path,
    process::Command,
};

// TODO: This program has way to many unwraps, these need to be removed and
// handled appropriately
// TODO: Handle unwrappings better.
// TODO: We need to keep track of history

fn main() {
    let history_file_name = ".shell_history";
    //Que for storing history commands
    let mut history_queue: VecDeque<String> = VecDeque::with_capacity(1000);
    // Start by setting current path to home directory
    match home::home_dir() {
        Some(path) => match env::set_current_dir(path) {
            //TODO: should we cover okay case here?
            Ok(_) => {}
            Err(_) => {
                println!("Error: could not change current directory to home path.");
            }
        },
        _ => {
            println!("error: something went wrong when fetching home directory.");
            std::process::exit(1);
        }
    }

    // We need to check if the file exists
    // TODO: maybe move this to try_exists instead, alternatively rework with
    // match statement, just try to open file, if that fails then create the
    // file
    let file_exists = Path::new(history_file_name).exists();

    if file_exists == false {
        // Create a history file to store commands
        match File::create(history_file_name) {
            Ok(_) => println!("Succssfully created file"),
            Err(_) => println!("Failed to create file"),
        }
    }

    //TODO: we need to populate que from file by reading line by line
    // and adding the commands into the que, should not exceed 1k commands
    for line in read_to_string(history_file_name).unwrap().lines() {
        history_queue.push_back(line.to_string());
    }

    loop {
        print!("$ ");
        let mut user_input = String::new();

        // Ensures it appears immediately.
        io::stdout().flush().unwrap();
        let bytes_read = io::stdin().read_line(&mut user_input).unwrap();

        // Check for EOF, which given null bytes should hit this.
        if bytes_read == 0 {
            break;
        }
        // retrieve the parts of user input splitted by whitespace
        let mut parts = user_input.trim().split_whitespace();
        // if the command is just empty then we continue the loop
        let command: String = match parts.next() {
            Some(cmd_value) => cmd_value.to_string().to_lowercase(),
            None => continue,
        };

        let args = parts.clone();
        let len = parts.count();

        match command.as_str() {
            "cd" => {
                let path_str = args.collect::<Vec<_>>().join(" ");
                let new_dir = Path::new(&path_str);
                // this how you change current dir just need to construt
                // it from a string
                match env::set_current_dir(new_dir) {
                    Ok(_) => println!("Successfully changed current dir"),
                    Err(_) => println!("cd: No such file or directory"),
                };
            }
            "exit" | "quit" if len == 0 => {
                break;
            }
            _ => {
                // This should be when we don't recognise first word as cd etc.
                match Command::new(command).args(args).spawn() {
                    Ok(mut child) => {
                        child.wait().expect("Could not execute");
                    }
                    Err(_) => println!("tshell: command not found."),
                }
            }
        }
    }
}
