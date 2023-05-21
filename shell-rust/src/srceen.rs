use colored::*;
use std::io;
use std::io::Write;
pub struct Srceen {}

impl Srceen {
    pub fn start() -> Vec<String> {
        let username = "diffany".red();
        print!("{} @ diffany-virtual-machine:~$", username);
        io::stdout().flush().expect("Failed to flush stdout");

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read input");

        let current_command = command.trim();
        let parts: Vec<String> = current_command
            .split(' ')
            .map(|part| part.to_string())
            .collect();

        parts
    }
}

