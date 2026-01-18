use std::{
    collections::VecDeque,
    env::{self},
    fs::{File, OpenOptions, read_to_string},
    io::{self, BufWriter, Write},
    path::Path,
    process::Command,
};

// TODO: This program has way to many unwraps, these need to be removed and
// handled appropriately
// TODO: Handle unwrappings better.
// TODO: we need to ensure that commands are added after they are actually used.

fn main() {
    let history_file_name = ".shell_history";

    //Que for storing history commands
    let mut history_queue: VecDeque<String> = VecDeque::with_capacity(1000);

    // Start by setting current path to home directory
    set_home_directory();

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

    for line in read_to_string(history_file_name).unwrap().lines() {
        // remove this ugly addition of newline, figure out a better way to
        // read the contents of file.
        history_queue.push_back(line.to_string() + "\n");
    }

    loop {
        print!("$ ");
        let mut user_input = String::new();

        // Ensures it appears immediately, do we need this?
        io::stdout().flush().unwrap();
        let bytes_read = io::stdin().read_line(&mut user_input).unwrap();

        // Check for EOF, which given null bytes should hit this.
        if bytes_read == 0 {
            break;
        }

        // Clone string value to history queue, can we do this better?
        // TODO: this needs to be moved
        history_queue.push_back(user_input.clone());

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
            "history" if len == 0 => {
                for (i, v) in history_queue.iter().enumerate() {
                    print!("{}.\t{}", i + 1, v);
                }
            }
            "!!" => {
                println!("previous command");
                continue;
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
    // set home directory first, so we always save in correct dir.
    set_home_directory();

    match OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(history_file_name)
    {
        Ok(history_file) => {
            let mut writer = BufWriter::new(history_file);
            for v in history_queue.iter() {
                // figure out why we need to instansiate this???
                let _ = write!(writer, "{}", v);
            }
        }
        Err(_) => println!("error: something went wrong when opening history file."),
    };
}

// used to set the current directory to the home directory
fn set_home_directory() {
    match home::home_dir() {
        Some(path) => match env::set_current_dir(path) {
            //TODO: should we cover okay case here?
            Ok(_) => {}
            Err(_) => {
                println!("Error: could not change current directory to home path.");
            }
        },
        _ => {
            println!("error: when fetching home directory.");
            std::process::exit(1);
        }
    }
}
