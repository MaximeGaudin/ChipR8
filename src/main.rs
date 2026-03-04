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
            "The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer."
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
            "The interpreter sets the program counter to nnn."
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

    fn documentation(&self) -> String {}

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
    let unknown_count = &instructions
        .into_iter()
        .filter(|i| matches!(i, Instruction::UNK(_)))
        .count();

    println!("Total instructions: {}", instructions_count);
    println!("Total UNK instructions: {}", unknown_count);
}
