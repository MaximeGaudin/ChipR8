struct VM {}

struct Jump {
    address: u16,
}
impl Jump {
    fn disassemble(&self) -> String {
        format!("JMP {:03X}", self.address)
    }
    fn execute(&self, vm: &mut VM) { /* logique jump */
    }
}

struct Unknown {
    opcode: u16,
}
impl Unknown {
    fn disassemble(&self) -> String {
        format!("UNKNOWN: {:04X}", self.opcode)
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
            Instruction::UNK(i) => i.disassemble()
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

fn load_rom(path: &str) -> Vec<Instruction> {
    let content = std::fs::read(path).unwrap();

    content
        .chunks_exact(2)
        .map(|chunk| {
            let opcode = ((chunk[0] as u16) << 8) | (chunk[1] as u16);
            opcode_to_instruction(opcode)
        })
        .collect()
}

fn main() {
    let instructions = load_rom("roms/corax89.ch8");

    
    for instruction in &instructions {
        println!("{}", instruction.disassemble())
    }

    let instructions_count = &instructions.len();
    let unknown_count = &instructions.into_iter()
    .filter(|i| matches!(i, Instruction::UNK(_)))
    .count();

    println!("Total instructions: {}", instructions_count);
    println!("Total UNK instructions: {}", unknown_count);
}
