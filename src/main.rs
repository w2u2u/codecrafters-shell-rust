#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        match input.split_once(' ') {
            Some(("echo", msg)) => println!("{}", msg.trim()),
            Some(("exit", code)) => process::exit(code.trim().parse::<i32>().unwrap()),
            _ => println!("{}: command not found", input.trim()),
        }
    }
}
