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
// TODO: introduce a better way to handle match statements, USE ENUMS!

enum ParsedCommand {
    Cd { path: String },
    Exit,
    History,
    RepeatLast,       // !!
    RepeatNth(usize), // !10
    External { program: String, args: Vec<String> },
}

impl ParsedCommand {
    fn parse(input: &str) -> Option<Self> {
        let input = input.trim();

        if input == "!!" {
            return Some(Self::RepeatLast);
        }

        if let Some(n) = parse_bang_number(input) {
            return Some(Self::RepeatNth(n as usize));
        }

        let mut parts = input.split_whitespace();
        let cmd = parts.next()?;

        match cmd {
            "cd" => {
                let path = parts.collect::<Vec<_>>().join(" ");
                Some(Self::Cd { path })
            }
            "exit" | "quit" => Some(Self::Exit),
            "history" => Some(Self::History),
            _ => Some(Self::External {
                program: cmd.to_string(),
                args: parts.map(String::from).collect(),
            }),
        }
    }
}

impl ParsedCommand {
    fn execute(self, history: &VecDeque<String>) -> Result<Option<String>, String> {
        match self {
            ParsedCommand::RepeatLast => history
                .back()
                .cloned()
                .ok_or("No previous command".into())
                .map(Some),

            ParsedCommand::RepeatNth(n) => history
                .get(n - 1)
                .cloned()
                .ok_or("History index out of range".into())
                .map(Some),

            ParsedCommand::Cd { path } => {
                match env::set_current_dir(path) {
                    Ok(_) => println!("Successfully changed current dir"),
                    Err(_) => println!("cd: No such file or directory"),
                };
                Ok(None)
            }

            ParsedCommand::Exit => {
                std::process::exit(0);
            }

            ParsedCommand::History => {
                for (i, cmd) in history.iter().enumerate() {
                    print!("{}.\t{}", i + 1, cmd);
                }
                Ok(None)
            }

            ParsedCommand::External { program, args } => {
                match Command::new(program).args(args).spawn() {
                    Ok(mut child) => {
                        child.wait().expect("Could not execute");
                    }
                    Err(_) => println!("tshell: command not found."),
                }
                Ok(None)
            }
        }
    }
}

fn main() {
    let history_file_name = ".shell_history";

    //Que for storing history commands
    let mut history_queue: VecDeque<String> = VecDeque::with_capacity(1000);

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
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }

        let cmd = match ParsedCommand::parse(&input) {
            Some(c) => c,
            None => continue,
        };

        match cmd.execute(&history_queue) {
            Ok(Some(expanded)) => {
                // execute expanded history command
                if let Some(c) = ParsedCommand::parse(&expanded) {
                    let _ = c.execute(&history_queue);
                }
            }
            Ok(None) => {}
            Err(e) => println!("tshell: {}", e),
        }

        history_queue.push_back(input);
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

fn parse_bang_number(s: &str) -> Option<u64> {
    s.strip_prefix('!')?.parse().ok()
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
