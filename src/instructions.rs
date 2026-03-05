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

// 1nnn
pub struct LoadValue {
    pub register: usize,
    pub value: u8,
}

impl LoadValue {
    fn disassemble(&self) -> String {
        format!("LD V{:1X}, {:02X}", self.register, self.value)
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
}

impl Instruction {
    pub fn disassemble(&self) -> String {
        match self {
            Instruction::JMP(i) => i.disassemble(),
            Instruction::CLS(i) => i.disassemble(),
            Instruction::LDV(i) => i.disassemble(),

            Instruction::UNK(i) => i.disassemble(),
        }
    }

    pub fn execute(&self, vm: &mut VM) {
        match self {
            Instruction::JMP(i) => i.execute(vm),
            Instruction::CLS(i) => i.execute(vm),
            Instruction::LDV(i) => i.execute(vm),

            Instruction::UNK(i) => i.execute(vm),
        }
    }
}
