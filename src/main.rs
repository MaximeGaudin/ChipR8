use std::error::Error;

mod instructions;
use instructions::*;

mod vm;
use vm::VM;

mod tests;

fn opcode_to_instruction(opcode: u16) -> Instruction {
    match opcode & 0xF000 {
        0x1000 => Instruction::JMP(Jump {
            address: opcode & 0x0FFF,
        }),
        _ => Instruction::UNK(Unknown { opcode }),
    }
}

fn load_rom(path: String) -> Result<Vec<u16>, std::io::Error> {
    let content = std::fs::read(path)?;

    Ok(content
        .chunks_exact(2)
        .map(|chunk| {((chunk[0] as u16) << 8) | (chunk[1] as u16) })
        .collect())
}

fn fetch_decode_execute(vm: &mut VM) {

}

fn main() -> Result<(),  Box<dyn Error>>{
    // 1. Boot: Init memory, init PC, init registers...
    // 2. Load Rom into Memory
    // 2. Start the fetch_decode_execute cycle
    let instructions : Vec<Instruction> = load_rom("roms/corax89.ch8".to_string())?.iter()
        .map(|o| opcode_to_instruction(*o))
        .collect();

    for instruction in &instructions {
        println!("{}", instruction.disassemble())
    }

    let instructions_count = &instructions.len();
    let unknown_count = &instructions
        .into_iter()
        .filter(|i| matches!(i, Instruction::UNK(_)))
        .count();

    println!("Total instructions: {}", instructions_count);
    println!("Total UNK instructions: {}", unknown_count);

    Ok(())
}