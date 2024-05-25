#[allow(unused_imports)]
use std::io::{self, Write};
use std::{path::Path, process};

const ECHO_COMMAND: &str = "echo";
const TYPE_COMMAND: &str = "type";
const EXIT_COMMAND: &str = "exit";

struct ShellCommand {
    name: String,
}

impl ShellCommand {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    fn is_shell_builtin(&self) -> bool {
        matches!(
            self.name.as_str(),
            ECHO_COMMAND | TYPE_COMMAND | EXIT_COMMAND
        )
    }

    fn get_path(&self) -> Option<String> {
        let path_env = std::env::var("PATH").unwrap_or_default();
        let paths = path_env.split(':').collect::<Vec<&str>>();

        paths.iter().find_map(|&path| {
            let mut full_path = Path::new(path).join(&self.name);
            full_path.set_extension("");

            if full_path.exists() {
                Some(full_path.to_string_lossy().to_string())
            } else {
                None
            }
        })
    }
}

enum Command {
    Echo(String),
    Type(ShellCommand),
    Exit(i32),
    Unknown(String),
}

impl Command {
    fn parse(input: &str) -> Self {
        let cmd = input.trim().split_once(' ');

        match cmd {
            Some((ECHO_COMMAND, arg)) => Command::Echo(arg.trim().to_string()),
            Some((TYPE_COMMAND, arg)) => Command::Type(ShellCommand::new(arg.trim())),
            Some((EXIT_COMMAND, code)) => Command::Exit(code.trim().parse().unwrap()),
            Some((cmd, _)) => Command::Unknown(cmd.to_string()),
            None => Command::Unknown(input.trim().to_string()),
        }
    }

    fn execute(&self) {
        match self {
            Command::Echo(cmd) => println!("{}", cmd),
            Command::Type(cmd) if cmd.is_shell_builtin() => {
                println!("{} is a shell builtin", cmd.name)
            }
            Command::Type(cmd) => {
                if let Some(path) = cmd.get_path() {
                    println!("{} is {}", cmd.name, path);
                } else {
                    println!("{} not found", cmd.name);
                }
            }
            Command::Exit(code) => process::exit(*code),
            Command::Unknown(cmd) => println!("{}: command not found", cmd),
        }
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let command = Command::parse(&input);
        command.execute();
    }
}
