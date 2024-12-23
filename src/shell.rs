use crate::fs::FileSystem;
use crate::println;
use alloc::string::String;
// use crate::println;
use crate::print;
use alloc::vec::Vec;
use crate::alloc::string::ToString;
use x86_64::instructions::{interrupts, hlt};


pub fn shutdown() -> ! {
    println!("Shutting down system...");
    interrupts::disable();
    loop {
        hlt();
    }
}

pub fn start_shell(file_system: &mut FileSystem) {
    use crate::keyboard::read_keyboard;

    println!("Entering interactive shell. Type `help` for commands, or `exit` to quit.");

    let mut buffer = String::new();

    loop {
        // Display prompt
        print!("{} >> ", file_system.current_directory);
        buffer.clear(); // Clear the buffer before reading new input

        // Read keyboard input and append to the buffer
        read_keyboard(&mut buffer); // Function to read keyboard input
        // Process the input when 'Enter' is pressed
        if buffer.ends_with('\n') && buffer.len() > 1 {
            let cmd = buffer.trim().to_string(); // Get command by trimming the buffer
            match cmd.as_str() {
                "help" => {
                    println!("Magenta", "black", "Available commands:");
                    println!("yellow", "black", " help - Display this message");
                    println!("yellow", "black", "  exit - Exit the shell");
                    println!("yellow", "black", "  echo <text> - Print the given text");
                    println!("yellow", "black", "  touch <filename> - Create a new file");
                    println!("yellow", "black", "  ls - List all files");
                    println!("yellow", "black", "  cat <filename> - Read a file");
                    println!("yellow", "black", "  write <filename> <data> - Write to a file");
                    println!("yellow", "black", "  mkdir <directory_name> - Create a new directory");
                    println!("yellow", "black", "  cd <directory_name> - Change the current directory");
                    println!("yellow", "black", "  shutdown - Poweroff");
                    println!("yellow", "black", "  pwd - Get current working directory");
                    buffer.clear();
                }
                "exit" => {
                    println!("Exiting shell...");
                    break; // Exit the loop when `exit` is typed
                }
                
                cmd if cmd.starts_with("touch ") => {
                    let filename = &cmd[6..];
                    file_system.create_file(filename.to_string());
                    buffer.clear();
                }
                "shutdown" => {
                    println!("Shutting down...");
                    shutdown(); // Call the shutdown function
                }
                "ls" => {
                    file_system.list_files();
                    buffer.clear();
                }
                cmd if cmd.starts_with("cat ") => {
                    let filename = &cmd[4..];
                    if let Some(data) = file_system.read_file(filename) {
                        let data_string = String::from_utf8_lossy(data);
                        println!("{}", data_string); // Assuming data is a Vec<u8>
                    }
                    
                    buffer.clear();
                }
                cmd if cmd.starts_with("echo ") => {
                    let parts: Vec<&str> = cmd[5..].split_whitespace().collect();
                    // print!("{}",parts.len());
                    if parts.len() < 2 {
                        let text = &cmd[5..];
                        println!("{}", text);
                    } else if parts.len() >= 3 {
                        let idx = parts.len() - 1; // Index of the last element
                        let filename = parts[idx]; // Take the last element
                        let data: Vec<&str> = parts[0..idx-1].to_vec();
    
                        // Convert data into bytes
                        let data_bytes = data.join(" ").into_bytes();
                        file_system.write_file(filename, data_bytes);
                    }else {
                        println!("Usage: echo text or echo text > <filename>");
                    }
                        buffer.clear();
                }
                cmd if cmd.starts_with("mkdir ") => {
                    let dirname = &cmd[6..];
                    file_system.create_directory(dirname.to_string());
                    buffer.clear();
                }
                cmd if cmd.starts_with("cd ") => {
                    let dirname = &cmd[3..];
                    file_system.change_directory(dirname.to_string());
                    buffer.clear();
                }
                "pwd" => {
                    println!("{}", file_system.current_directory);
                    buffer.clear();
                }
                _ => {
                    println!("Unknown command. Type `help` for a list of commands.");
                }
            }
        }
    }
}
