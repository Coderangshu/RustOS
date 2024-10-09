#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// a module to handle VGA hardware supported printing
mod vga_buffer;

// static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // don't mangle the name of this function
// this function is the entry point, since the linker looks for a function
// named `_start` by default
pub extern "C" fn _start() -> ! {
    // 0xb8000 contains the vga buffer, it is a special memory mapped to VGA hardware and
    // contains the contents of the displayed screen

    // let vga_buffer = 0xb8000 as *mut u8;

    // we use unsafe block around all memory writes as otherwise rust compiler won't
    // have allowed this behaviour, as it couldn't prove the raw pointers created do really exist
    // we tell the compiler we are sure about the viability of the pointers called

    // for (i,&byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    // before creating WRITER we need to write all print functions inside the vga module
    // and call from here in this way
    // vga_buffer::print_something();

    // after creating the WRITER we can directly print to screen from anywhere in the code base
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again,\n").unwrap();
    write!(vga_buffer::WRITER.lock(), "some numbers: {} {}", 43, 1.34343).unwrap();

    loop {}
}
