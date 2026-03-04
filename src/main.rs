use std::error::Error;

struct VM {}

// 0nnn - SYS addr
struct JumpSys {
    address: u16,
}

impl JumpSys {
    fn disassemble(&self) -> String {
        format!("SYS {:03X}", self.address)
    }

    fn documentation(&self) -> Vec<&str> {
        vec![
            "Jump to a machine code routine at nnn.",
            "This instruction is only used on the old computers on which Chip-8 was originally implemented. It is ignored by modern interpreters.",
        ]
    }

    fn execute(&self, vm: &mut VM) {
        /* IGNORED */
    }
}

// 00E0 - CLS
struct ClearScreen {}

impl ClearScreen {
    fn disassemble(&self) -> String {
        "CLS".to_string()
    }

    fn documentation(&self) -> Vec<&str> {
        vec!["Clear the display."]
    }

    fn execute(&self, vm: &mut VM) {
        // TODO
    }
}

// 00EE - RET
struct Return {}

impl Return {
    fn disassemble(&self) -> String {
        "RET".to_string()
    }

    fn documentation(&self) -> Vec<&str> {
        vec![
            "Return from a subroutine.",
            "The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.",
        ]
    }

    fn execute(&self, vm: &mut VM) {
        // TODO
    }
}

// 1nnn
struct Jump {
    address: u16,
}

impl Jump {
    fn disassemble(&self) -> String {
        format!("JMP {:03X}", self.address)
    }

    fn documentation(&self) -> Vec<&str> {
        vec![
            "Jump to location nnn.",
            "The interpreter sets the program counter to nnn.",
        ]
    }

    fn execute(&self, vm: &mut VM) {
        // TODO
    }
}

struct Unknown {
    opcode: u16,
}
impl Unknown {
    fn disassemble(&self) -> String {
        format!("UNKNOWN: {:04X}", self.opcode)
    }

    fn documentation(&self) -> Vec<&str> {
        vec!["A unknown or not implemented instruction"]
    }

    fn executable(&self, vm: &mut VM) {
        // Nothing to do
    }
}

enum Instruction {
    JMP(Jump),
    UNK(Unknown),
}

impl Instruction {
    fn disassemble(&self) -> String {
        match self {
            Instruction::JMP(i) => i.disassemble(),
            Instruction::UNK(i) => i.disassemble(),
        }
    }
}

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

fn main() -> Result<(),  Box<dyn Error>>{
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

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::load_rom;

    #[test]
    fn test_rom_content_read() {
        let rom_prefix = "roms/corax89";
        let reference_opcodes = to_opcodes(format!("{}.dis", rom_prefix)).unwrap();
        let opcodes = load_rom(format!("{}.ch8", rom_prefix)).unwrap();

        assert_eq!(reference_opcodes, opcodes);
    }

    fn to_opcodes(path: String) -> Result<Vec<u16>, Box<dyn Error>> {
        let content = std::fs::read_to_string(path)?;
        let bytes: Vec<u8> = content
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .filter_map(|s| {
                // Remove "0x" and try to parse
                let hex = s.trim_start_matches("0x");
                u8::from_str_radix(hex, 16).ok()
            })
            .collect();

        Ok(bytes
            .chunks_exact(2)
            .map(|chunk| {
                // byte1 = high bits, byte2 = low bits
                ((chunk[0] as u16) << 8) | (chunk[1] as u16)
            })
            .collect())
    }
}
