#![no_std]
#![no_main]

use aprk_user_lib::{print, exit};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print("\n");
    print("=========================================\n");
    print("  Hello from USER SPACE! (Phase 10)      \n");
    print("=========================================\n");
    print("I am a separate ELF binary loaded from TarFS.\n");
    print("I am making System Calls via SVC instruction.\n");
    print("\n");
    print("Exiting now...\n");
    exit();
}
