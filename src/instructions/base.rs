use crate::vm::VM;
use crate::instructions::arithmetic::*;
use crate::instructions::other::*;
use crate::instructions::subroutines::*;
use crate::instructions::screen::*;
use crate::instructions::skip::*;

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
    DRW(Draw),
    ADV(AddValue),

    SEV(SkipIfValue),
    SENV(SkipIfNotValue),
    SER(SkipIfRegister),
    SENR(SkipIfNotRegister),

    CALL(Call),
    RET(Return),
}

impl Instruction {
    pub fn disassemble(&self) -> String {
        match self {
            Instruction::JMP(i) => i.disassemble(),
            Instruction::CLS(i) => i.disassemble(),
            Instruction::DRW(i) => i.disassemble(),
            Instruction::ADV(i) => i.disassemble(),
            Instruction::LDV(i) => i.disassemble(),
            Instruction::LDI(i) => i.disassemble(),
            Instruction::SEV(i) => i.disassemble(),
            Instruction::SER(i) => i.disassemble(),
            Instruction::SENV(i) => i.disassemble(),
            Instruction::SENR(i) => i.disassemble(),
            Instruction::CALL(i) => i.disassemble(),
            Instruction::RET(i) => i.disassemble(),

            Instruction::UNK(i) => i.disassemble(),
        }
    }

    pub fn execute(&self, vm: &mut VM) {
        match self {
            Instruction::CLS(i) => i.execute(vm),
            Instruction::DRW(i) => i.execute(vm),

            Instruction::JMP(i) => i.execute(vm),
            Instruction::LDV(i) => i.execute(vm),
            Instruction::LDI(i) => i.execute(vm),
            Instruction::ADV(i) => i.execute(vm),
            Instruction::SEV(i) => i.execute(vm),
            Instruction::SER(i) => i.execute(vm),
            Instruction::SENV(i) => i.execute(vm),
            Instruction::SENR(i) => i.execute(vm),
            Instruction::CALL(i) => i.execute(vm),
            Instruction::RET(i) => i.execute(vm),

            Instruction::UNK(i) => i.execute(vm),
        }
    }
}

pub fn opcode_to_instruction(opcode: u16) -> Instruction {
    match opcode & 0xF000 {
        0x0000 => match opcode {
            0x00E0 => Instruction::CLS(ClearScreen {}),
            0x00EE => Instruction::RET(Return {}),
            _ => Instruction::UNK(Unknown { opcode }),
        },
        0x1000 => Instruction::JMP(Jump {
            address: (opcode & 0x0FFF) as usize,
        }),
        0x2000 => Instruction::CALL(Call {
            address: (opcode & 0x0FFF) as usize,
        }),
        0x3000 => Instruction::SEV(SkipIfValue {
            register: ((opcode & 0x0F00) >> 8) as usize,
            value: (opcode & 0x00FF) as u8,
        }),
        0x4000 => Instruction::SENV(SkipIfNotValue {
            register: ((opcode & 0x0F00) >> 8) as usize,
            value: (opcode & 0x00FF) as u8,
        }),
        0x5000 => Instruction::SER(SkipIfRegister {
            register_x: ((opcode & 0x0F00) >> 8) as usize,
            register_y: ((opcode & 0x00F0) >> 4) as usize,
        }),
        0x6000 => Instruction::LDV(LoadValue {
            register: ((opcode & 0x0F00) >> 8) as usize,
            value: (opcode & 0x00FF) as u8,
        }),
        0x7000 => Instruction::ADV(AddValue {
            register: ((opcode & 0x0F00) >> 8) as usize,
            value: (opcode & 0x00FF) as u8,
        }),
        0x9000 => Instruction::SENR(SkipIfNotRegister {
            register_x: ((opcode & 0x0F00) >> 8) as usize,
            register_y: ((opcode & 0x00F0) >> 4) as usize,
        }),
        0xA000 => Instruction::LDI(LoadI {
            value: (opcode & 0x0FFF) as usize,
        }),
        0xD000 => Instruction::DRW(Draw {
            register_x: ((opcode & 0x0F00) >> 8) as usize,
            register_y: ((opcode & 0x00F0) >> 4) as usize,
            n_bytes: (opcode & 0x000F) as usize,
        }),

        _ => Instruction::UNK(Unknown { opcode }),
    }
}