pub mod stdio;

use std::env;
use std::fs::File;

use rvemu_core::cpu::*;
use rvemu_core::memory::*;

use stdio::*;

/// Output current registers to the console.
fn dump_registers(cpu: &Cpu) {
    println!("{}", cpu.xregs);
    println!("---------------------------------------------------");
    println!("{}", cpu.fregs);
    println!("---------------------------------------------------");
    println!("pc: {}", cpu.pc);
}

/// Main function of RISC-V emulator for the CLI version.
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: ./rvemu-cli <binary-file-name>");
    }

    let mut file = File::open(&args[1])?;
    let mut dram = Vec::new();
    file.read_to_end(&mut dram)?;

    let mut cpu = Cpu::new();
    let mut mem = Memory::new();
    mem.dram.splice(..dram.len(), dram.iter().cloned());

    cpu.start(&mut mem, stdin_check);

    dump_registers(&cpu);

    Ok(())
}
