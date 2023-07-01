#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::Write;
use vga_buffer::WRITER;

mod vga_buffer;


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel Panic!");
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    WRITER.lock().write_str("Hello again").unwrap();
    write!(WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    println!("some numbers: {} {}", 42, 1.337);
    println!("Hello, World{}", "!");
    println!("Hello Universe!!");

    loop {}
}
