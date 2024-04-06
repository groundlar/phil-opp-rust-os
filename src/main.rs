#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(phil_opp_rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use phil_opp_rust_os::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    phil_opp_rust_os::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hullo, Wörld!");

    phil_opp_rust_os::init();

    // Trigger page fault by assigning random memory.
    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    }

    #[cfg(test)]
    test_main();

    println!("We didn't crash!");
    loop {}
}

#[test_case]
fn test_it_works() {
    assert_eq!(1, 1);
}
