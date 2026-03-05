use crate::VM;

// 0nnn - SYS addr
pub struct JumpSys {
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
pub struct ClearScreen {}

impl ClearScreen {
    fn disassemble(&self) -> String {
        "CLS".to_string()
    }

    fn documentation(&self) -> Vec<&str> {
        vec!["Clear the display."]
    }

    fn execute(&self, vm: &mut VM) {
        vm.screen.fill(0);
    }
}

// 00EE - RET
pub struct Return {}

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
pub struct Jump {
    pub address: u16,
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

// 6kkk
pub struct LoadValue {
    pub register: usize,
    pub value: u8,
}

impl LoadValue {
    fn disassemble(&self) -> String {
        format!("LD V{:1X}, {:2X}", self.register, self.value)
    }

    fn documentation(&self) -> Vec<&str> {
        vec![
            "Set Vx = kk.",
            "The interpreter puts the value kk into register Vx.",
        ]
    }

    fn execute(&self, vm: &mut VM) {
        vm.registers[self.register] = self.value;
    }
}

// Annn
pub struct LoadI {
    pub value: u16,
}

impl LoadI {
    fn disassemble(&self) -> String {
        format!("LD I, {:03X}", self.value)
    }

    fn documentation(&self) -> Vec<&str> {
        vec![
            "Set I = nnn.",
            "The value of register I is set to nnn.",
        ]
    }

    fn execute(&self, vm: &mut VM) {
        vm.i = self.value;
    }
}

pub struct Unknown {
    pub opcode: u16,
}
impl Unknown {
    fn disassemble(&self) -> String {
        format!("UNKNOWN: {:04X}", self.opcode)
    }

    fn documentation(&self) -> Vec<&str> {
        vec!["A unknown or not implemented instruction"]
    }

    fn execute(&self, vm: &mut VM) {
        // Nothing to do
    }
}

pub enum Instruction {
    JMP(Jump),
    CLS(ClearScreen),
    UNK(Unknown),
    LDV(LoadValue),
    LDI(LoadI),
}

impl Instruction {
    pub fn disassemble(&self) -> String {
        match self {
            Instruction::JMP(i) => i.disassemble(),
            Instruction::CLS(i) => i.disassemble(),
            Instruction::LDV(i) => i.disassemble(),
            Instruction::LDI(i) => i.disassemble(),

            Instruction::UNK(i) => i.disassemble(),
        }
    }

    pub fn execute(&self, vm: &mut VM) {
        match self {
            Instruction::JMP(i) => i.execute(vm),
            Instruction::CLS(i) => i.execute(vm),
            Instruction::LDV(i) => i.execute(vm),
            Instruction::LDI(i) => i.execute(vm),

            Instruction::UNK(i) => i.execute(vm),
        }
    }
}

pub fn opcode_to_instruction(opcode: u16) -> Instruction {
    match opcode & 0xF000 {
        0x0000 => match opcode {
            0x00E0 => Instruction::CLS(ClearScreen {}),
            _ => Instruction::UNK(Unknown { opcode }),
        },
        0x1000 => Instruction::JMP(Jump {
            address: opcode & 0x0FFF,
        }),
        0x6000 => Instruction::LDV(LoadValue { 
            register: ((opcode & 0x0F00) >> 8) as usize,
             value: (opcode & 0x00FF) as u8
        }),
        0xA000 => Instruction::LDI(LoadI { 
             value: (opcode & 0x0FFF) as u16
        }),

        _ => Instruction::UNK(Unknown { opcode }),
    }
}