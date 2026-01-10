# APRK OS

<p align="center">
  <img src="docs/assets/logo.png" alt="APRK OS Logo" width="200" />
</p>

<p align="center">
  <strong>A modern operating system kernel written in Rust for ARM64 architecture.</strong>
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#building">Building</a> •
  <a href="#running">Running</a> •
  <a href="#documentation">Documentation</a> •
  <a href="#contributing">Contributing</a>
</p>

---

## Overview

**APRK OS** is a modern operating system kernel built from scratch in Rust. It aims to completely master the hardware, providing a robust, Type-Safe foundation for ARM64 computing.

- **Architecture**: ARM64 (AArch64)
- **Language**: Rust + Assembly
- **Type**: Monolithic Kernel
- **License**: GPL-2.0

## Current Version

**v0.0.1 "Genesis"** — The beginning.

| Feature | Status |
|---------|--------|
| ARM64 Boot | ✅ |
| Serial Console (UART) | ✅ |
| Basic Panic Handler | ✅ |
| Memory Management | 🚧 |
| Process Scheduler | 🔜 |
| File System | 🔜 |
| Userspace | 🔜 |

## Features

### Implemented
- **ARM64 Bare Metal Boot**: Custom boot assembly that initializes the CPU
- **PL011 UART Driver**: Serial console output for QEMU virt machine
- **Panic Handling**: Graceful kernel panic with debug information

### Coming Soon
- Exception handling and interrupts
- Physical and virtual memory management
- Process scheduling
- System calls
- File system
- User mode programs

## Building

### Prerequisites

- **Rust** (nightly toolchain)
- **QEMU** (for ARM64 emulation)
- **GNU Make**

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install QEMU
# macOS:
brew install qemu

# Ubuntu/Debian:
sudo apt install qemu-system-arm

# The nightly toolchain and ARM64 target will be installed automatically
```

### Build Commands

```bash
# Build debug version
make build

# Build release version
make release

# Check code without building
make check

# Format code
make fmt

# Run clippy linter
make clippy
```

## Running

### On QEMU (Recommended)

```bash
# Build and run
make run

# Run release build
make run-release

# Exit QEMU: Press Ctrl+A, then X
```

### With Debugging

```bash
# Start QEMU with GDB server
make debug

# In another terminal, connect GDB:
aarch64-none-elf-gdb -ex 'target remote :1234' target/aarch64-unknown-none/debug/aprk-kernel
```

## Project Structure

```
aprk-os/
├── kernel/                  # Core kernel code
│   └── src/
│       ├── main.rs          # Kernel entry point
│       └── linker.ld        # Memory layout
├── arch/                    # Architecture-specific code
│   └── arm64/
│       └── src/
│           ├── boot.S       # Assembly entry point
│           ├── uart.rs      # Serial driver
│           └── cpu.rs       # CPU utilities
├── Documentation/           # System documentation
├── scripts/                 # Build and run scripts
├── Cargo.toml              # Workspace configuration
└── Makefile                # Build automation
```

## Documentation

Documentation follows a structured format:

- `Documentation/process/` — Development guidelines
- `Documentation/arch/arm64/` — ARM64 specifics
- `Documentation/mm/` — Memory management
- `Documentation/scheduler/` — Process scheduling

## Contributing

APRK OS is currently a personal project. Contribution guidelines will be added as the project matures.

## License

This project is licensed under the GNU General Public License v2.0 — see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- **Rust Embedded Community** — For excellent no_std resources
- **Rust Community** — For excellent bare-metal support
- **QEMU Project** — For making ARM64 development accessible

---

<p align="center">
  <em>APRK OS — Building an operating system from scratch, one commit at a time.</em>
</p>
