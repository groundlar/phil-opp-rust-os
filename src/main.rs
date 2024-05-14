#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(phil_opp_rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use phil_opp_rust_os::{print, println};
use x86_64::registers::control::Cr3;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    phil_opp_rust_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    phil_opp_rust_os::test_panic_handler(info)
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hullo, Wörld!");

    phil_opp_rust_os::init();

    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );
    // PhysAddr(0x1000)
    // We can't access this directly for obvious reasons.

    #[cfg(test)]
    test_main();

    println!("We didn't crash!");
    phil_opp_rust_os::hlt_loop();
}

#[test_case]
fn test_it_works() {
    assert_eq!(1, 1);
}
