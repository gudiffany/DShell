use std::{env, fs};
pub struct Command {
    history_command: Vec<String>,
}

impl Command {
    pub fn new() -> Self {
        Command {
            history_command: Vec::new(),
        }
    }

    pub fn output_command(&mut self, command_name: String, parts: Vec<String>) {
        match command_name.as_str() {
            "ls" => {
                let mut current_dir = env::current_dir().expect("Failed to get current directory.");
                if parts.len() > 1 {
                    current_dir = current_dir.join(&parts[1]);
                }
                let entries = fs::read_dir(current_dir).expect("Failed to read directory.");
                for entry in entries {
                    if let Ok(entry) = entry {
                        let file_name = entry.file_name();
                        let file_name = file_name.to_string_lossy();
                        println!("{}", file_name);
                    }
                }
            }
            "cat" => {
                if parts.len() > 1 {
                    if let Ok(readfile) = fs::read_to_string(&parts[1]) {
                        println!("{}", readfile);
                    } else {
                        println!("Error: File not found or unable to read");
                    }
                } else {
                    println!("No filename provided");
                }
            }
            "env" => {
                println!("hua Q");
            }

            "history" => {
                println!("{:?}", self.history_command);
            }
            "exit" => {
                println!("success exit");
                std::process::exit(1);
            }
            _ => {
                println!("command not found: {:?}", parts);
            }
        }
        self.history_command.push(parts.join(" "));
    }
}
