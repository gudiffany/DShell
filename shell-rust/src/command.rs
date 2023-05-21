use std::{env, fs};
pub struct Command {}

impl Command {
    pub fn output_command(command_name: String, parts: Vec<String>) {
        match command_name.as_str() {
            
            "ls" => {
                if parts.len() > 1 {
                    let current_dir = env::current_dir().expect("Failed to get current directory.");
                    let directory_path = current_dir.join(&parts[1]);
                    let entries = fs::read_dir(directory_path).expect("Failed to read directory.");
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let file_name = entry.file_name();
                            let file_name = file_name.to_string_lossy();
                            println!("{}", file_name);
                        }
                    }
                } else {
                    let current_dir = env::current_dir().expect("Failed to get current directory.");
                    let entries = fs::read_dir(current_dir).expect("Failed to read directory.");
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let file_name = entry.file_name();
                            let file_name = file_name.to_string_lossy();
                            println!("{}", file_name);
                        }
                    }
                }
            }


            "exit" => {
                println!("success exit");
                std::process::exit(1);
            }
            _ => {
                println!("command not found: {:?}", parts);
            }
        }
    }
}
