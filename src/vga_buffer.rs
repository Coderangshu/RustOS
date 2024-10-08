// as u4 doesn't exist we have to use u8 representation
// but there are only 16 colors and thus only 4 bits are sufficient
// so 4 more bits will stay empty and rust doesn't allow that
// so this line is added to allow dead code
#[allow(dead_code)]
// the below options are derived so that each color can be copied and printed
// into new variables as normally rust doesn't allow this
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
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
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

// we use volatile to tell the rust compiler not to optimize these
// buffer writes as this are important and cannot be skipped which
// it would have done by default as we are always writing ot the buffer
// and not reading from even once in our whole code, thus making the
// compiler think these writes as unimportant
use volatile::Volatile;
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// to write to screen we use the Writer type
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    // buffer stores a reference to the VGA buffer
    // static means that the reference is valid for lifetime
    buffer: &'static mut Buffer,
}

// Now we will use the Writer to modify the buffer's characters
// this method is to write as single ASCII byte
impl Writer {
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
}

impl Writer {
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
}

impl Writer {
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

// to print strings, we can convert string to individual bytes and print one by one
impl Writer {
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

// a wrapper fucntion for write_string so that we can use the Rust's built in write! / writeln! formatting macros
// this is similar to write_string only it has a return type fmt:Result, which can be achieved by implementing the
// core::fmt::Write trait
use core::fmt;
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// trying to create a global WRITER without carrying the Writer in every module
pub static WRITER: Writer = Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::Yellow, Color::Black),
    buffer: unsafe {&mut *(0xb8000 as *mut Buffer)},
};

// a test function to check the capabilities of the Writer we wrote
pub fn print_something() {
    use core::fmt::Write;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::LightRed, Color::White),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
    write!(writer, "The numbers are {} and {},\nthis is new line", 42, 1.0/3.0).unwrap();
}