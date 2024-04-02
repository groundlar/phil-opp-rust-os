#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    write!(vga_buffer::WRITER.lock(), "Hullo, Wörld!\n").unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        "Numbers! {} and {}\n ",
        42,
        1.0 / 3.0
    )
    .unwrap();
    loop {}
}
