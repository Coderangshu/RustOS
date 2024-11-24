
use crate::fs::FileSystem;
use crate::println;
use alloc::string::String;
// use crate::println;
use crate::print;
use alloc::vec::Vec;
use crate::alloc::string::ToString;
use x86_64::instructions::{interrupts, hlt};


pub fn shutdown(file_system: &mut FileSystem) -> ! {
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
        // print!(">> ");
        print!("{} >> ", file_system.current_directory);
        buffer.clear(); // Clear the buffer before reading new input

        // Read keyboard input and append to the buffer
        read_keyboard(&mut buffer); // Function to read keyboard input

        // Process the input when 'Enter' is pressed
        if buffer.ends_with('\n') && buffer.len() > 1 {
            let cmd = buffer.trim().to_string(); // Get command by trimming the buffer
            match cmd.as_str() {
                "help" => {
                    println!("Available commands:");
                    println!("  help - Display this message");
                    println!("  exit - Exit the shell");
                    println!("  echo <text> - Print the given text");
                    println!("  touch <filename> - Create a new file");
                    println!("  ls - List all files");
                    println!("  cat <filename> - Read a file");
                    println!("  write <filename> <data> - Write to a file");
                    println!("  mkdir <directory_name> - Create a new directory");
                    println!("  cd <directory_name> - Change the current directory");
                    buffer.clear();
                }
                "exit" => {
                    println!("Exiting shell...");
                    break; // Exit the loop when `exit` is typed
                }
                cmd if cmd.starts_with("echo ") => {
                    let text = &cmd[5..];
                    println!("{}", text);
                    buffer.clear();
                }
                cmd if cmd.starts_with("touch ") => {
                    let filename = &cmd[6..];
                    file_system.create_file(filename.to_string());
                    buffer.clear();
                }
                "shutdown" => {
                    println!("Shutting down...");
                    shutdown(file_system); // Call the shutdown function
                }
                "ls" => {
                    file_system.list_files();
                    buffer.clear();
                }
                cmd if cmd.starts_with("cat ") => {
                    let filename = &cmd[4..];
                    if let Some(data) = file_system.read_file(filename) {
                        println!("File content: {:?}", data); // Assuming data is a Vec<u8>
                    }
                    buffer.clear();
                }
                cmd if cmd.starts_with("write ") => {
                    let parts: Vec<&str> = cmd[6..].split_whitespace().collect();
                    if parts.len() < 2 {
                        println!("Usage: write <filename> <data>");
                    } else {
                        let filename = parts[0];
                        let data = parts[1..].join(" ").into_bytes();
                        file_system.write_file(filename, data);
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
