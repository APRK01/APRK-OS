// =============================================================================
// APRK OS - Exception Handling (Rust)
// =============================================================================
// Rust handlers for the exceptions defined in exception.S
// =============================================================================

use crate::println;
use crate::gic::Gic;
use crate::timer::Timer;
use core::time::Duration;

extern "C" {
    fn kernel_syscall_handler(id: u64, arg0: u64, arg1: u64);
}

/// Initialize exceptions.
/// Sets the VBAR_EL1 register to point to our vector table.
pub unsafe fn init() {
    extern "C" {
        static exception_vector_table: u8;
    }
    
    let vector_addr = &exception_vector_table as *const u8 as u64;
    
    // Set VBAR_EL1 (Vector Base Address Register)
    core::arch::asm!("msr vbar_el1, {}", in(reg) vector_addr);
}

/// Handler for Synchronous Exceptions (e.g., Data Abort, SVC).
#[no_mangle]
pub extern "C" fn handle_sync_exception() {
    let esr: u64;
    
    unsafe {
        core::arch::asm!("mrs {}, esr_el1", out(reg) esr);
    }
    
    let ec = (esr >> 26) & 0x3F;
    
    // EC = 0x15 is SVC (System Call) from AArch64
    // EC = 0x15 is SVC (System Call) from AArch64
    if ec == 0x15 {
        let id: u64;
        let arg0: u64;
        let arg1: u64;
        unsafe {
             // x8 holds syscall number, x0, x1 holds args
            core::arch::asm!(
                "mov {0}, x8",
                "mov {1}, x0",
                "mov {2}, x1",
                out(reg) id,
                out(reg) arg0,
                out(reg) arg1
            );
            kernel_syscall_handler(id, arg0, arg1);
        }
        return; // Return to user
    }
    
    let elr: u64;
    unsafe {
        core::arch::asm!("mrs {}, elr_el1", out(reg) elr);
    }
    
    println!("\n!!! SYNCHRONOUS EXCEPTION !!!");
    println!("ESR_EL1: {:#018x}", esr);
    println!("ELR_EL1: {:#018x}", elr);
    println!("System halted.");
    
    loop { core::hint::spin_loop(); }
}

/// Handler for IRQ Exceptions (Hardware Interrupts).
#[no_mangle]
pub extern "C" fn handle_irq_exception() {
    // 1. Acknowledge interrupt from GIC
    let iar = Gic::acknowledge();
    let irq_id = iar & 0x3FF; // Lower 10 bits are the ID

    // 2. Handle the interrupt
    match irq_id {
        27 | 30 => {
            // Timer Interrupt
            extern "Rust" { fn kernel_tick(); }
            unsafe { kernel_tick(); }
            
            Timer::set_next_tick(Duration::from_secs(1));
        }
        33 => {
            // UART Interrupt
            crate::uart::handle_irq();
        }
        1023 => {
            // Spurious - ignore
        }
        _ => {
            println!("[IRQ] Unknown interrupt ID: {}", irq_id);
        }
    }

    // 3. Signal End Of Interrupt to GIC
    Gic::end_interrupt(iar);
}
