use std::{
    env::{self},
    io::{self, Write},
    path::Path,
    process::Command,
};

// TODO: This program has way to many unwraps, these need to be removed and
// handled appropriately
// TODO: Handle unwrappings better.

fn main() {
    // Start by setting current path to home directory
    let path = home::home_dir().unwrap();
    env::set_current_dir(path).unwrap();

    loop {
        let mut user_input = String::new();
        print!("$ ");

        io::stdout().flush().unwrap(); // Ensures it appears immediately.
        let bytes_read = io::stdin().read_line(&mut user_input).unwrap();

        // Check for EOF, which given null bytes should hit this.
        if bytes_read == 0 {
            break;
        }

        // Unsure if this is supposed to be unwrap or we should check for err
        let mut parts = user_input.trim().split_whitespace();
        let command: String = parts.next().unwrap().to_string().to_lowercase();
        let args = parts.clone();
        let len = parts.count();

        if len == 0 && (command == "exit" || command == "quit") {
            break;
        }

        if command == "cd" {
            let path_str = args.collect::<Vec<_>>().join(" ");
            let new_dir = Path::new(&path_str);
            // this how you change current dir just need to construt
            // it from a string
            match env::set_current_dir(new_dir) {
                Ok(_) => println!("Successfully changed current dir"),
                Err(_) => println!("cd: No such file or directory"),
            };
        } else {
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
