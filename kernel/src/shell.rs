use aprk_arch_arm64::{print, println, uart};
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn run() {
    println!("\n[shell] Welcome to APRK OS Shell!");
    println!("Type 'help' for commands.");
    print!("> ");
    
    let mut buffer = String::new();
    
    unsafe { aprk_arch_arm64::cpu::enable_interrupts(); }

    loop {
        if let Some(c) = uart::get_char() {
            if c == b'\n' || c == b'\r' {
                println!(); // Newline
                execute_command(buffer.trim());
                buffer.clear();
                print!("> ");
            } else if c == 8 || c == 127 {
                // Backspace
                if !buffer.is_empty() {
                     buffer.pop();
                     // Visual backspace: Move back, overwrite with space, move back
                     print!("\x08 \x08");
                }
            } else {
                buffer.push(c as char);
                // Echo character
                print!("{}", c as char);
            }
        } else {
            // No input? Spin loop or yield?
            // Just spin loop for now.
             for _ in 0..10_000 { unsafe { core::arch::asm!("nop") } }
        }
    }
}

fn execute_command(cmd_line: &str) {
    let mut parts = cmd_line.split_whitespace();
    let cmd = match parts.next() {
        Some(c) => c,
        None => return,
    };
    let args: Vec<&str> = parts.collect();

    match cmd {
        "help" => {
             println!("Available commands:");
             println!("  help           - Show this menu");
             println!("  clear          - Clear screen");
             println!("  whoami         - Print user info");
             println!("  uname          - Print system info");
             println!("  ls             - List files");
             println!("  cat <file>     - Print file content");
        },
        "clear" => print!("\x1b[2J\x1b[1;1H"),
        "whoami" => println!("root (KERNEL_GOD_MODE)"),
        "uname" => println!("APRK OS v0.0.1 aarch64"),
        "ls" => {
            crate::fs::ls(crate::fs::RAMDISK);
        },
        "cat" => {
            if let Some(filename) = args.get(0) {
                if let Some(file) = crate::fs::get_file(crate::fs::RAMDISK, filename) {
                    if file.is_dir {
                         println!("Error: '{}' is a directory", filename);
                    } else {
                         // Print data as string (UTF-8 lossy)
                         let content = core::str::from_utf8(file.data)
                             .unwrap_or("<Binary Data>");
                         println!("{}", content);
                    }
                } else {
                    println!("Error: File '{}' not found", filename);
                }
            } else {
                println!("Usage: cat <filename>");
            }
        },
        _ => println!("Unknown command: '{}'", cmd),
    }
}
