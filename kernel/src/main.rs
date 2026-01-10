// =============================================================================
// APRK OS - Kernel Entry Point
// =============================================================================
// This is the main Rust entry point for the APRK OS kernel.
// Called from boot.S after basic hardware initialization.
//
// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2025 APRK
// =============================================================================

#![no_std]
#![no_main]

use aprk_arch_arm64::{self as arch, cpu, println};
use core::panic::PanicInfo;

// =============================================================================
// Version Information
// =============================================================================

/// APRK OS version
const VERSION: &str = "0.0.1";

/// APRK OS codename
const CODENAME: &str = "Genesis";

// =============================================================================
// Kernel Entry Point
// =============================================================================

/// Kernel main entry point.
/// 
/// This function is called from assembly boot code after:
/// - CPU 0 is selected (other cores are halted)
/// - Stack is initialized
/// - BSS section is zeroed
/// 
/// # Safety
/// This function must be called only once, by the boot assembly code.
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    // Initialize architecture-specific hardware
    // This now includes MMU, Exceptions, GIC, and Timer!
    arch::init();

    // Print the APRK OS banner
    print_banner();

    // Print system information
    print_system_info();

    println!();
    println!("[kernel] Core subsystems initialized:");
    println!("         - MMU (Identity Map)");
    println!("         - Exceptions (Vector Table)");
    println!("         - GICv2 (Interrupt Controller)");
    println!("         - ARM Generic Timer");
    
    println!();
    println!("[kernel] Waiting for timer interrupts... (Press Ctrl+A, X to exit)");

    // Enter main loop - we just wait for interrupts now
    loop {
        // Wait For Event - puts CPU to sleep until interrupt fires
        unsafe { core::arch::asm!("wfe"); }
    }
}

// =============================================================================
// Boot Output
// =============================================================================

/// Print the APRK OS boot banner.
fn print_banner() {
    println!();
    println!(r"    _    ____  ____  _  __   ___  ____  ");
    println!(r"   / \  |  _ \|  _ \| |/ /  / _ \/ ___| ");
    println!(r"  / _ \ | |_) | |_) | ' /  | | | \___ \ ");
    println!(r" / ___ \|  __/|  _ <| . \  | |_| |___) |");
    println!(r"/_/   \_\_|   |_| \_\_|\_\  \___/|____/ ");
    println!();
    println!("APRK OS v{} \"{}\"", VERSION, CODENAME);
    println!("A modern operating system kernel for ARM64");
    println!();
    println!("============================================================");
}

/// Print system information.
fn print_system_info() {
    let el = cpu::current_el();
    let sp = cpu::read_sp();

    println!("[boot] Kernel loaded successfully");
    println!("[boot] Current Exception Level: EL{}", el);
    println!("[boot] Stack Pointer: {:#018x}", sp);
    println!("[boot] UART initialized");
}

// =============================================================================
// Panic Handler
// =============================================================================

/// Panic handler for kernel panics.
/// 
/// This is called when the kernel encounters an unrecoverable error.
/// We print diagnostic information and halt the CPU.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!();
    println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    println!("!!                     KERNEL PANIC                        !!");
    println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    println!();
    
    if let Some(location) = info.location() {
        println!("Location: {}:{}:{}", 
            location.file(), 
            location.line(), 
            location.column()
        );
    }
    
    if let Some(message) = info.message().as_str() {
        println!("Message: {}", message);
    } else {
        println!("Message: {}", info.message());
    }
    
    println!();
    println!("System halted.");
    
    cpu::halt();
}

// =============================================================================
// Tests (for future use)
// =============================================================================

#[cfg(test)]
mod tests {
    // Unit tests will go here
}
