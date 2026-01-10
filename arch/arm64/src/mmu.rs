// =============================================================================
// APRK OS - Memory Management Unit (MMU)
// =============================================================================
// Handles virtual memory setup for ARM64.
// For Phase 2, we implement a simple identity mapping (VA=PA).
// =============================================================================

use core::arch::asm;

// Number of entries in a page table
const ENTRIES_COUNT: usize = 512;

// Page Table Entry flags
const PROT_VALID: u64 = 1 << 0;
const PROT_BLOCK: u64 = 0 << 1; // 0 = Block, 1 = Table
const AF: u64 = 1 << 10;      // Access Flag (Must be 1 to avoid fault)

// Memory Attributes (Index into MAIR_EL1)
const MT_DEVICE_NGNRNE: u64 = 0;
const MT_NORMAL_NC: u64 = 1;
const MT_NORMAL: u64 = 2; // Cacheable

// Access Permissions
const AP_RW_EL1: u64 = 0 << 6; // Read-Write EL1 only

// Shareability
const SH_INNER: u64 = 3 << 8;

/// A translation table (4KB).
#[repr(C, align(4096))]
struct Table {
    entries: [u64; ENTRIES_COUNT],
}

// Statically allocate page tables.
#[no_mangle]
static mut L1_TABLE: Table = Table { entries: [0; ENTRIES_COUNT] };

/// Initialize the MMU.
/// 
/// # Safety
/// Must only be called during boot. Changes memory view globally.
pub unsafe fn init() {
    // -------------------------------------------------------------------------
    // 1. Setup MAIR_EL1 (Memory Attribute Indirection Register)
    // -------------------------------------------------------------------------
    let mair_val: u64 = (0x00 << (8 * MT_DEVICE_NGNRNE)) |
                        (0x44 << (8 * MT_NORMAL_NC)) |
                        (0xFF << (8 * MT_NORMAL));
    asm!("msr mair_el1, {}", in(reg) mair_val);

    // -------------------------------------------------------------------------
    // 2. Setup Page Tables (Identity Map 0-2GB)
    // -------------------------------------------------------------------------
    let l1_table_ptr = core::ptr::addr_of_mut!(L1_TABLE);

    // Entry 0: 0-1GB (Devices / MMIO)
    (*l1_table_ptr).entries[0] = 
        0x0000_0000 | 
        PROT_VALID | 
        PROT_BLOCK | 
        (MT_DEVICE_NGNRNE << 2) | 
        AP_RW_EL1 |
        AF;

    // Entry 1: 1GB-2GB (RAM at 0x4000_0000)
    (*l1_table_ptr).entries[1] = 
        0x4000_0000 | 
        PROT_VALID | 
        PROT_BLOCK | 
        (MT_NORMAL << 2) | 
        AP_RW_EL1 |
        SH_INNER | 
        AF;

    // -------------------------------------------------------------------------
    // 3. Setup TCR_EL1 (Translation Control Register)
    // -------------------------------------------------------------------------
    // T0SZ = 25 (39-bit VA)
    // TG0 = 0 (4KB granule)
    // SH0 = 3 (Inner Shareable)
    // ORGN0/IRGN0 = 1 (Normal WB Write-Back Cacheable)
    let tcr_val: u64 = (25 << 0)  | // T0SZ
                       (3 << 12) | // SH0
                       (1 << 10) | // ORGN0
                       (1 << 8)  | // IRGN0
                       (0 << 14) | // TG0 (4KB)
                       (2 << 32);  // IPS (40-bit PA)
    asm!("msr tcr_el1, {}", in(reg) tcr_val);

    // -------------------------------------------------------------------------
    // 4. Invalidate TLBs to ensure no stale mappings
    // -------------------------------------------------------------------------
    asm!("tlbi vmalle1is", "dsb sy", "isb");

    // -------------------------------------------------------------------------
    // 5. Set TTBR0_EL1 and Enable MMU
    // -------------------------------------------------------------------------
    let ttbr0 = l1_table_ptr as u64;
    asm!("msr ttbr0_el1, {}", in(reg) ttbr0);
    asm!("isb");

    let mut sctlr: u64;
    asm!("mrs {}, sctlr_el1", out(reg) sctlr);
    sctlr |= 1 | (1 << 2) | (1 << 12); // M, C, I bits
    asm!("msr sctlr_el1, {}", in(reg) sctlr);
    
    asm!("isb");
}
