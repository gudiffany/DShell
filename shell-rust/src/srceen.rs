use crate::command::Command;
use colored::*;
use std::io;
use std::io::Write;
pub struct Srceen {}

impl Srceen {
    pub fn start(&mut self) {
        let username = "diffany".red();
        print!("{} @ diffany-virtual-machine:~$", username);
        io::stdout().flush().expect("Failed to flush stdout");
        self.input_command();
    }

    fn input_command(&mut self) {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read input");

        let current_command = command.trim();
        let parts: Vec<String> = current_command
            .split(' ')
            .map(|part| part.to_string())
            .collect();
        let command_name = &parts[0];
        Command::output_command(command_name.to_string(), parts)
    }
}
