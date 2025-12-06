use std::{
    io::{self, Write},
    process::Command,
};

fn main() {
    loop {
        let mut user_input = String::new();
        print!("> ");

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

        println!("{len}");

        if len == 0 && (command == "exit" || command == "quit") {
            break;
        }

        let mut child = Command::new(command).args(args).spawn().unwrap();

        child.wait().expect("Could not execute");
    }
}
