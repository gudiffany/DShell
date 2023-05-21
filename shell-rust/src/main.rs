use std::{env, fs};
mod srceen;
fn main() {
    loop {
        let parts = srceen::Srceen::start(); // 刷新标准输出缓冲区

        let command_name = &parts[0];
        // println!("{}",command_name);
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
                return;
            }
            _ => {
                println!("command not found: {:?}", parts);
            }
        }
    }
}

