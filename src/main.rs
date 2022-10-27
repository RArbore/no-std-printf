#![feature(lang_items)]
#![feature(start)]
#![no_main]
#![no_std]

use core::arch::asm;
use core::panic::PanicInfo;

static HELLO_WORLD: &[u8] = b"Hello, world!\n";

#[no_mangle]
#[start]
pub unsafe extern "C" fn _start(_argc: isize, _argv: *const *const u8) -> () {
    asm!(
            "mov rsi, {hello_world}",
            "mov rdx, 14",
            "mov rax, 1",
            "mov rdi, 1",
            "syscall",
            hello_world = in(reg) (HELLO_WORLD.as_ptr())
        );

    asm!(
            "mov rax, 60",
            "mov rdi, 0",
            "syscall"
        );
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

