use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts};
use spin::Mutex;
use lazy_static::lazy_static;
use x86_64::instructions::port::Port;
use alloc::string::String;
use crate::print; // Ensure this is imported

lazy_static! {
    pub static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    ));
}

/// Reads the keyboard input and appends it to the buffer
pub fn read_keyboard(buffer: &mut String) {
    let mut keyboard = KEYBOARD.lock(); // Acquire lock on the keyboard
    let mut port = Port::new(0x60); // Keyboard I/O port

    // State to track pressed keys
    let mut last_scancode = None;

    loop {
        // Read scancode from the port
        let scancode: u8 = unsafe { port.read() };

        // If the scancode is the same as the last one, skip processing (debouncing)
        if Some(scancode) == last_scancode {
            continue;
        }
        
        last_scancode = Some(scancode); // Store the current scancode for the next iteration

        // Process the scancode
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            // Check if the key event is new (not already processed)
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        if character == '\n' {
                            // Enter key, complete input
                            if buffer.len()==0 {
                                continue;
                            }
                            buffer.push(character); // Add Enter to the buffer
                            // print!("Enter pressed, input completed.\n");
                            return; // Exit the loop and return to main.rs
                        } else if character == '\x08' {
                            // Backspace key, remove last character
                            buffer.pop();
                        } else {
                            // Append character to the buffer
                            buffer.push(character);
                        }
                    }
                    DecodedKey::RawKey(key) => {
                        // Handle non-printable keys (e.g., Shift, Ctrl, etc.)
                        print!("{:?}", key);
                    }
                }
            }
        }
    }
}
