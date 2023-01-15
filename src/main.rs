use clap::Parser;
use getch::Getch;

use cpu::CPU;
use memory::Memory;

mod instruction;
mod memory;
mod registers;
mod cpu;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Size of memory. Should be big enough to load program file
    #[arg(short, long)]
    memory: usize,

    /// Run emulator in interactive mode. (Space - run next instruction, m - dump memory, r - dump registers)
    #[arg(short, long, default_value_t = false)]
    interactive: bool,

    /// Program file to emulate
    file: String,
}

fn main() {
    let args = Args::parse();

    let mut memory = Memory::new(args.memory);
    memory.load_file(args.file.as_str()).expect("File not found");

    let mut cpu = CPU::from_memory(&memory);


    if args.interactive {
        let g = Getch::new();
        println!("---INTERACTIVE MODE---");
        println!("<space> - run next command");
        println!("m - dump memory");
        println!("r - dump registers");
        println!("q - quite");
        println!();

        while !cpu.halted {
            let char = g.getch().unwrap_or(0) as char;
            match char {
                ' ' => cpu.tick(),
                'r' => cpu.dump_registers(),
                'm' => cpu.dump_memory(),
                'q' => break,
                _ => ()
            }
        }
    } else {
        cpu.run();
    }
}
