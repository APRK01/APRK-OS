#![no_std]

use core::panic::PanicInfo;

/// Exit the current process.
pub fn exit() -> ! {
    unsafe {
        core::arch::asm!(
            "mov x8, #1", // Syscall ID: EXIT
            "svc #0",
            options(noreturn)
        );
    }
}

/// Print a string to the console.
pub fn print(s: &str) {
    unsafe {
        core::arch::asm!(
            "mov x8, #0", // Syscall ID: PRINT
            "svc #0",
            in("x0") s.as_ptr(),
            in("x1") s.len(),
            clobber_abi("C")
        );
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print("PANIC in user mode!\n");
    exit()
}
