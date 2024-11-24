use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new (Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe {&mut *(0xb8000 as *mut Buffer)},
    });
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]

pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

// To represent a full color code to specify both foreground and background , we create a newtype on top of u8
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// To ensure that the ColorCode has the exact same data layout as a u8, we use the repr(transparent) attribute
#[repr(transparent)]
pub struct ColorCode(u8); // FIXME

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8)<<4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// using C representation for this struct as field ordering is undefines in rust
// thus we follow the C style ordering
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// to write to screen we use the Writer type
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {

    pub fn default_color_code() -> ColorCode {
        ColorCode::new(Color::White, Color::Black)
    }
    
    pub fn move_cursor_back(&mut self) {
        if self.column_position > 0 {
            self.column_position -= 1;
        } else if BUFFER_HEIGHT > 1 {
            // If at the start of a line, move up one row
            self.column_position = BUFFER_WIDTH - 1;
        }
    }

    /// Clears the character at the current cursor position
    pub fn clear_char_at(&mut self) {
        let row = BUFFER_HEIGHT - 1;
        let col = self.column_position;

        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        self.buffer.chars[row][col].write(blank);
    }

    /// Handles backspace: moves the cursor back and clears the character
    pub fn handle_backspace(&mut self) {
        self.move_cursor_back();
        self.clear_char_at();
    }

    // to print new line
    fn new_line(&mut self) {
        // for row we start from row 1 as row 0 is moved out of view when moved up
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row-1][col].write(character);
            }
        }
        // clear the last row
        self.clear_row(BUFFER_HEIGHT-1);
        // set the position of cursor in the row at beginning
        self.column_position = 0;
    }

    // to clear the last row after moving all the above lines up
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        // switch on byte
        match byte {
            // case 1: if byte is a newline
            b'\n' => self.new_line(),
            // case 2: not a newline character
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                // now instead of normal assigning with = we are using write function
                // provided by Volatile crate, making these writes to buffer important
                // and not getting deleted by the compiler optimization
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    // to print strings, we can convert string to individual bytes and print one by one
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // if printable ASCII (any character from <space> to <tilda>) byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // else not part of printable ASCII range
                // we print ■ which is denoted by oxfe in hex code
                _ => self.write_byte(0xfe),
            }
        }
    }
}

// a wrapper function for write_string so that we can use the Rust's built in write! / writeln! formatting macros
// this is similar to write_string only it has a return type fmt:Result, which can be achieved by implementing the
// core::fmt::Write trait
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

impl Color {
    pub fn from_str(color: &str) -> Option<Color> {
        match color.to_lowercase().as_str() {
            "black" => Some(Color::Black),
            "blue" => Some(Color::Blue),
            "green" => Some(Color::Green),
            "cyan" => Some(Color::Cyan),
            "red" => Some(Color::Red),
            "magenta" => Some(Color::Magenta),
            "brown" => Some(Color::Brown),
            "lightgray" => Some(Color::LightGray),
            "darkgray" => Some(Color::DarkGray),
            "lightblue" => Some(Color::LightBlue),
            "lightgreen" => Some(Color::LightGreen),
            "lightcyan" => Some(Color::LightCyan),
            "lightred" => Some(Color::LightRed),
            "pink" => Some(Color::Pink),
            "yellow" => Some(Color::Yellow),
            "white" => Some(Color::White),
            _ => None,
        }
    }
}




#[doc(hidden)]
// this func is a wrapper func which locks the WRITER and calls write_fmt method, which is imported
// from the Write trait, the additional unwrap() at end panics if no printing occurs
// but as we return Ok() at the end of the write_str panic doesn't happen
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    // the writer lock is taken and print is printed inside a no interrupt block
    // in this way when a print function is executing no interrupt can occur as it is disabled
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

//Default working code
// #[macro_export]
// macro_rules! print {
//     ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
// }

// #[macro_export]
// macro_rules! println {
//     () => ($crate::print!("\n"));
//     ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
// }

pub fn _print_with_color(args: fmt::Arguments, color_code: ColorCode) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writer.color_code = color_code;
        writer.write_fmt(args).unwrap();
    });
}

#[macro_export]
macro_rules! print {
    // If both foreground and background are specified
    ($fg:expr, $bg:expr, $($arg:tt)*) => {{
        let fg_color = $crate::vga_buffer::Color::from_str($fg).expect("Invalid foreground color");
        let bg_color = $crate::vga_buffer::Color::from_str($bg).expect("Invalid background color");
        let color_code = $crate::vga_buffer::ColorCode::new(fg_color, bg_color);
        $crate::vga_buffer::_print_with_color(format_args!($($arg)*), color_code);
    }};
    // Default case: no color specified, use the default color (white on black)
    ($($arg:tt)*) => {{
        let default_color_code = $crate::vga_buffer::Writer::default_color_code();
        $crate::vga_buffer::_print_with_color(format_args!($($arg)*), default_color_code);
    }};
}

#[macro_export]
macro_rules! println {
    // If both foreground and background are specified
    ($fg:expr, $bg:expr, $($arg:tt)*) => {{
        let fg_color = $crate::vga_buffer::Color::from_str($fg).expect("Invalid foreground color");
        let bg_color = $crate::vga_buffer::Color::from_str($bg).expect("Invalid background color");
        let color_code = $crate::vga_buffer::ColorCode::new(fg_color, bg_color);
        $crate::vga_buffer::_print_with_color(format_args!($($arg)*), color_code);
        $crate::vga_buffer::_print_with_color(format_args!("\n"), color_code);
    }};
    // Default case: no color specified, use the default color (white on black)
    () => {{
        let default_color_code = $crate::vga_buffer::Writer::default_color_code();
        $crate::vga_buffer::_print_with_color(format_args!("\n"), default_color_code);
    }};
    // Default case with string formatting
    ($($arg:tt)*) => {{
        let default_color_code = $crate::vga_buffer::Writer::default_color_code();
        $crate::vga_buffer::_print_with_color(format_args!($($arg)*), default_color_code);
        $crate::vga_buffer::_print_with_color(format_args!("\n"), default_color_code);
    }};
}




// A test function to check println works without panicking
#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}
// A test to check if no panic occurs when many lines are printed nad also shifted off the screen
#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}
// We can also create a test function to verify that the printed lines really appear on the screen
#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}