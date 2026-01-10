// =============================================================================
// APRK OS - ARM64 Boot Assembly
// =============================================================================
// This is the first code that runs when APRK OS boots on ARM64.
// It sets up the initial environment and jumps to Rust code.
//
// Entry point: _start
// Target: QEMU virt machine (ARM64)
// =============================================================================

.section .text._start
.global _start

_start:
    // -------------------------------------------------------------------------
    // Step 1: Check processor ID - only boot on CPU 0
    // -------------------------------------------------------------------------
    // On multi-core systems, all cores start executing here.
    // We only want core 0 to continue; others should halt.
    mrs     x0, mpidr_el1           // Read Multiprocessor Affinity Register
    and     x0, x0, #0xFF           // Extract CPU ID (Aff0 field)
    cbnz    x0, halt                // If not CPU 0, halt

    // -------------------------------------------------------------------------
    // Step 2: Set up the stack pointer
    // -------------------------------------------------------------------------
    // The stack grows downward, so we point to the top of our stack area.
    // Stack is defined in the linker script.
    adrp    x0, __stack_top         // Load page address of stack top
    add     x0, x0, :lo12:__stack_top
    mov     sp, x0                  // Set stack pointer

    // -------------------------------------------------------------------------
    // Step 3: Clear the BSS section
    // -------------------------------------------------------------------------
    // BSS contains uninitialized global variables; we must zero them.
    adrp    x0, __bss_start         // Start of BSS
    add     x0, x0, :lo12:__bss_start
    adrp    x1, __bss_end           // End of BSS
    add     x1, x1, :lo12:__bss_end

bss_clear_loop:
    cmp     x0, x1                  // Check if we've reached the end
    b.ge    bss_clear_done          // If so, we're done
    str     xzr, [x0], #8           // Store zero and increment pointer
    b       bss_clear_loop          // Repeat

bss_clear_done:
    // -------------------------------------------------------------------------
    // Step 4: Jump to Rust kernel entry point
    // -------------------------------------------------------------------------
    // At this point:
    // - We're running on CPU 0 only
    // - Stack is set up
    // - BSS is zeroed
    // Time to hand control to Rust!
    bl      kernel_main             // Call the Rust entry point

    // -------------------------------------------------------------------------
    // If kernel_main returns (it shouldn't), halt the CPU
    // -------------------------------------------------------------------------
halt:
    wfe                             // Wait for event (low power halt)
    b       halt                    // Loop forever

// =============================================================================
// End of boot.S
// =============================================================================
