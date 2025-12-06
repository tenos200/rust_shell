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
        let input: String = user_input.to_lowercase().trim().parse().unwrap();

        println!("{input}");

        if input == "exit" || input == "quit" {
            break;
        }

        // This can execute a command lol, but not with arguments.
        Command::new(input)
            .spawn()
            .expect("Failed to execute command");
    }
}
