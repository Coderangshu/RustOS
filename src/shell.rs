
use crate::fs::FileSystem;
use crate::println;
use alloc::string::String;
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

    println!("red", "black", "Hello, This is Pratik. This is a red text on a black background!");
    print!("green", "black", "This is blue text on a yellow background!");
    println!("Entering interactive shell. Type `help` for commands, or `exit` to quit.");

    let mut buffer = String::new();

    loop {
        print!("{} >> ", file_system.current_directory);
        buffer.clear();

        read_keyboard(&mut buffer); 

        if buffer.ends_with('\n') && buffer.len() > 1 {
            let cmd = buffer.trim().to_string(); 
            match cmd.as_str() {
                "help" => {
                    println!("Available commands:");
                    println!("  help - Display this message");
                    println!("  exit - Exit the shell");
                    println!("  echo <text> - Print the given text");
                    println!("  touch <filename> - Create a new file");
                    println!("  ls - List all files");
                    println!("  cat <filename> - Read a file");
                    println!("  echo <data> > <filename> <data> - Write to a file");
                    println!("  mkdir <directory_name> - Create a new directory");
                    println!("  cd <directory_name> - Change the current directory");
                    buffer.clear();
                }
                "exit" => {
                    println!("Exiting shell...");
                    break; 
                }
 
                cmd if cmd.starts_with("touch ") => {
                    let filename = &cmd[6..];
                    file_system.create_file(filename.to_string());
                    buffer.clear();
                }
                "shutdown" => {
                    println!("Shutting down...");
                    shutdown(file_system); 
                }
                "ls" => {
                    file_system.list_files();
                    buffer.clear();
                }
                cmd if cmd.starts_with("cat ") => {
                    let filename = &cmd[4..];
                    if let Some(data) = file_system.read_file(filename) {
                        let data_string = String::from_utf8_lossy(data);
                        println!("{}", data_string);
                    }
                    
                    buffer.clear();
                }
                cmd if cmd.starts_with("echo ") => {
                    let parts: Vec<&str> = cmd[5..].split_whitespace().collect();
                    if parts.len() < 2 {
                        let text = &cmd[5..];
                        println!("{}", text);
                    } else if parts.len() >= 3 {
                        let idx = parts.len() - 1; 
                        let filename = parts[idx]; 
                        let data: Vec<&str> = parts[0..idx-1].to_vec();
    
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
