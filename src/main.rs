#[allow(unused_imports)]
use std::io::{self, Write};
use std::{path::Path, process};

const ECHO_COMMAND: &str = "echo";
const TYPE_COMMAND: &str = "type";
const EXIT_COMMAND: &str = "exit";

struct ShellCommand {
    name: String,
    args: Vec<String>,
}

impl ShellCommand {
    fn new(name: &str, arg: &str) -> Self {
        Self {
            name: name.to_string(),
            args: arg
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
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
    External(ShellCommand),
}

impl Command {
    fn parse(input: &str) -> Self {
        let cmd = input.trim().split_once(' ');

        match cmd {
            Some((ECHO_COMMAND, arg)) => Command::Echo(arg.trim().to_string()),
            Some((TYPE_COMMAND, arg)) => Command::Type(ShellCommand::new(arg.trim(), "")),
            Some((EXIT_COMMAND, code)) => Command::Exit(code.trim().parse().unwrap()),
            Some((cmd, arg)) => Command::External(ShellCommand::new(cmd, arg)),
            None => Command::External(ShellCommand::new(input.trim(), "")),
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
            Command::External(cmd) => {
                if cmd.get_path().is_none() {
                    println!("{}: command not found", cmd.name);
                    return;
                }

                let mut child = process::Command::new(&cmd.name)
                    .args(&cmd.args)
                    .stdout(std::io::stdout())
                    .spawn()
                    .unwrap();

                child.wait().unwrap();
            }
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
