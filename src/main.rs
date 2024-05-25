#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

struct ShellBuiltin {
    name: String,
}

impl ShellBuiltin {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    fn is_shell_builtin(&self) -> bool {
        matches!(self.name.as_str(), "echo" | "type" | "exit")
    }
}

enum Command {
    Echo(String),
    Type(ShellBuiltin),
    Exit(i32),
    Unknown(String),
}

impl Command {
    fn parse(input: &str) -> Self {
        let cmd = input.trim().split_once(' ');

        match cmd {
            Some(("echo", arg)) => Command::Echo(arg.trim().to_string()),
            Some(("type", arg)) => Command::Type(ShellBuiltin::new(arg.trim())),
            Some(("exit", code)) => Command::Exit(code.trim().parse().unwrap()),
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
            Command::Type(cmd) => println!("{} not found", cmd.name),
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
