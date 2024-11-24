
//This is
use crate::println;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;

pub struct File {
    pub name: String,
    pub data: Vec<u8>,
}

pub struct Directory {
    pub name: String,
    pub files: Vec<File>,
    pub subdirectories: Vec<Directory>,
}

pub struct FileSystem {
    pub root: Directory,          // The root directory of the file system
    pub current_directory: String, // The current working directory
}

impl FileSystem {
    // Create a new file system with a root directory
    pub fn new() -> Self {
        FileSystem {
            root: Directory {
                name: String::from("/"),
                files: Vec::new(),
                subdirectories: Vec::new(),
            },
            current_directory: String::from("/"), // Start at the root
        }
    }

    // Helper to get a mutable reference to the current directory
    fn get_current_directory_mut(&mut self) -> Option<&mut Directory> {
        let path_components: Vec<&str> = self.current_directory.split('/').filter(|&s| !s.is_empty()).collect();
        
        let mut current = &mut self.root;
        
        for component in path_components {
            if let Some(next_dir) = current.subdirectories.iter_mut().find(|dir| dir.name == component) {
                current = next_dir;
            } else {
                return None; // If directory not found, return None
            }
        }
        Some(current)
    }

    // Creates a new file and adds it to the current directory
    pub fn create_file(&mut self, name: String) {
        if let Some(current_dir) = self.get_current_directory_mut() {
            if current_dir.files.iter().any(|file| file.name == name) {
                println!("Error: File '{}' already exists.", name);
                return;
            }
            current_dir.files.push(File {
                name: name.clone(),
                data: Vec::new(),
            });
            println!("File '{}' created successfully.", name);
        } else {
            println!("Error: Current directory not found.");
        }
    }

    // Lists all files and subdirectories in the current directory
    pub fn list_files(&mut self) {
        // Save the current directory path in a variable before borrowing mutably
        let current_dir_path = self.current_directory.clone();
        
        if let Some(current_dir) = self.get_current_directory_mut() {
            if current_dir.files.is_empty() && current_dir.subdirectories.is_empty() {
                println!("The directory is empty.");
            } else {
                println!("Contents of '{}':", current_dir_path);  // Use the saved variable
                for file in &current_dir.files {
                    println!("f:{}", file.name);
                }
                for subdir in &current_dir.subdirectories {
                    println!("d:{}", subdir.name);
                }
            }
        } else {
            println!("Error: Current directory not found.");
        }
    }
    
    
    

    // Creates a new directory and adds it to the current directory
    pub fn create_directory(&mut self, name: String) {
        if let Some(current_dir) = self.get_current_directory_mut() {
            if current_dir.subdirectories.iter().any(|dir| dir.name == name) {
                println!("Error: Directory '{}' already exists.", name);
                return;
            }
            current_dir.subdirectories.push(Directory {
                name: name.clone(),
                files: Vec::new(),
                subdirectories: Vec::new(),
            });
            println!("Directory '{}' created successfully.", name);
        } else {
            println!("Error: Current directory not found.");
        }
    }

    // Change the current working directory
    pub fn change_directory(&mut self, path: String) {
        if path == ".." {
            if self.current_directory != "/" {
                let mut components: Vec<&str> = self.current_directory.split('/').collect();
                components.pop();
                self.current_directory = components.join("/");
                if self.current_directory.is_empty() {
                    self.current_directory = String::from("/");
                }
            }
        } else {
            let new_path = format!("{}/{}", self.current_directory, path);
            if let Some(_) = self.get_current_directory_mut() {
                self.current_directory = new_path;
            } else {
                println!("Error: Directory '{}' not found.", path);
            }
        }
    }

    // Reads a file by its name in the current directory
    pub fn read_file(&mut self, name: &str) -> Option<Vec<u8>> {
        if let Some(current_dir) = self.get_current_directory_mut() {
            // Use the correct field `data` instead of `content`
            for file in &current_dir.files {
                if file.name == name {
                    return Some(file.data.clone());  // Correct field name `data`
                }
            }
        }
        None
    }
    
    

    // Writes data to a file in the current directory
    pub fn write_file(&mut self, name: &str, data: Vec<u8>) {
        if let Some(current_dir) = self.get_current_directory_mut() {
            for file in &mut current_dir.files {
                if file.name == name {
                    file.data = data;
                    println!("Data written to file '{}'.", name);
                    return;
                }
            }
            println!("File '{}' not found.", name);
        }
    }
}
