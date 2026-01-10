/* =============================================================================
 * APRK OS - Linker Script
 * =============================================================================
 * Defines the memory layout for the kernel binary.
 * Target: QEMU virt machine (ARM64)
 *
 * Memory Map:
 * 0x40000000 - Kernel start (QEMU virt machine RAM starts here)
 * ============================================================================= */

/* Entry point - the _start symbol from boot.S */
ENTRY(_start)

/* QEMU virt machine loads kernel at 0x40080000 by default for ELF files */
/* We use 0x40080000 to leave room for device tree at 0x40000000 */
KERNEL_START = 0x40080000;

/* Stack size: 64KB should be plenty for early boot */
STACK_SIZE = 0x10000;

SECTIONS
{
    /* Set the location counter to kernel start address */
    . = KERNEL_START;

    /* -------------------------------------------------------------------------
     * .text section - Executable code
     * ------------------------------------------------------------------------- */
    .text : ALIGN(4096)
    {
        __text_start = .;
        
        /* Boot code must come first - it's our entry point */
        *(.text._start)
        *(.text .text.*)
        
        __text_end = .;
    }

    /* -------------------------------------------------------------------------
     * .rodata section - Read-only data (constants, strings)
     * ------------------------------------------------------------------------- */
    .rodata : ALIGN(4096)
    {
        __rodata_start = .;
        
        *(.rodata .rodata.*)
        
        __rodata_end = .;
    }

    /* -------------------------------------------------------------------------
     * .data section - Initialized read-write data
     * ------------------------------------------------------------------------- */
    .data : ALIGN(4096)
    {
        __data_start = .;
        
        *(.data .data.*)
        
        __data_end = .;
    }

    /* -------------------------------------------------------------------------
     * .bss section - Uninitialized data (zeroed at boot)
     * ------------------------------------------------------------------------- */
    .bss : ALIGN(4096)
    {
        __bss_start = .;
        
        *(.bss .bss.*)
        *(COMMON)
        
        __bss_end = .;
    }

    /* -------------------------------------------------------------------------
     * Stack - Grows downward from __stack_top
     * ------------------------------------------------------------------------- */
    . = ALIGN(4096);
    __stack_bottom = .;
    . += STACK_SIZE;
    __stack_top = .;

    /* -------------------------------------------------------------------------
     * End of kernel
     * ------------------------------------------------------------------------- */
    . = ALIGN(4096);
    __kernel_end = .;

    /* -------------------------------------------------------------------------
     * Discard unwanted sections
     * ------------------------------------------------------------------------- */
    /DISCARD/ :
    {
        *(.comment)
        *(.note*)
        *(.eh_frame*)
    }
}

/* =============================================================================
 * Assertions to catch linker issues early
 * ============================================================================= */

/* Ensure the kernel doesn't exceed 16MB (sanity check) */
ASSERT(__kernel_end - KERNEL_START < 0x1000000, "Kernel exceeds 16MB!")

/* Ensure BSS is properly aligned */
ASSERT(__bss_start % 8 == 0, "BSS start not 8-byte aligned!")
ASSERT(__bss_end % 8 == 0, "BSS end not 8-byte aligned!")

