use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts};
use spin::Mutex;
use lazy_static::lazy_static;
use x86_64::instructions::port::Port;
use alloc::string::String;
use alloc::collections::VecDeque;
use crate::print; // Ensure this is imported
use crate::vga_buffer::WRITER;
lazy_static! {
    pub static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    ));

    pub static ref INPUT_BUFFER: Mutex<VecDeque<u8>> = Mutex::new(VecDeque::new());
}

// Adds a character to the buffer
pub fn add_to_buffer(character: u8) {
    let mut buffer = INPUT_BUFFER.lock();
    buffer.push_back(character);
}

// Fetches a character from the buffer
pub fn fetch_from_buffer() -> Option<u8> {
    let mut buffer = INPUT_BUFFER.lock();
    
    buffer.pop_front()
}

pub fn read_keyboard(buffer: &mut String) {
    loop {
        // Fetch the next character from the buffer
        if let Some(character) = fetch_from_buffer() {
            if character!= b'\x08'{
                print!("{}", character as char); // Print the character
            }
            
            if character == b'\n' {
                // Enter pressed, input completed
                if !buffer.is_empty() {
                    buffer.push('\n');
                    return; // Exit the loop
                }
            } else if character == b'\x08' {
                // Backspace pressed, remove last character
                    buffer.pop(); // Remove the last character from the buffer
                    WRITER.lock().handle_backspace(); // Move cursor back and clear the character on screen
            } else {
                // Append character to the buffer
                buffer.push(character as char);
            }
        } else {
            // If the buffer is empty, let the CPU wait briefly (e.g., avoid busy-waiting)
            x86_64::instructions::hlt(); // Halts the CPU until the next interrupt
        }
    }
}

