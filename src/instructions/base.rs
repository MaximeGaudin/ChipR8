use crate::instructions::other::*;
use crate::instructions::screen::*;
use crate::instructions::skip::*;
use crate::instructions::subroutines::*;
use crate::vm::VM;

pub(super) struct Unknown {
    pub opcode: u16,
}

impl Instruction for Unknown {
    fn disassemble(&self) -> String {
        format!("UNKNOWN: {:04X}", self.opcode)
    }

    fn execute(&self, vm: &mut VM) {
        // Nothing to do
    }

    fn is_unknown(&self) -> bool {
        true
    }
}

pub trait Instruction {
    fn execute(&self, vm: &mut VM);
    fn disassemble(&self) -> String;
    fn is_unknown(&self) -> bool {
        false
    }
}

pub fn opcode_to_instruction(opcode: u16) -> Box<dyn Instruction> {
    match opcode & 0xF000 {
        0x0000 => match opcode {
            0x00E0 => Box::new(ClearScreen {}),
            0x00EE => Box::new(Return {}),
            _ => Box::new(Unknown { opcode }),
        },
        0x1000 => Box::new(Jump {
            address: (opcode & 0x0FFF) as usize,
        }),
        0x2000 => Box::new(Call {
            address: (opcode & 0x0FFF) as usize,
        }),
        0x3000 => Box::new(SkipIfValue {
            register: ((opcode & 0x0F00) >> 8) as usize,
            value: (opcode & 0x00FF) as u8,
        }),
        0x4000 => Box::new(SkipIfNotValue {
            register: ((opcode & 0x0F00) >> 8) as usize,
            value: (opcode & 0x00FF) as u8,
        }),
        0x5000 => Box::new(SkipIfRegister {
            register_x: ((opcode & 0x0F00) >> 8) as usize,
            register_y: ((opcode & 0x00F0) >> 4) as usize,
        }),
        0x6000 => Box::new(LoadValue {
            register: ((opcode & 0x0F00) >> 8) as usize,
            value: (opcode & 0x00FF) as u8,
        }),
        0x7000 => Box::new(AddValue {
            register: ((opcode & 0x0F00) >> 8) as usize,
            value: (opcode & 0x00FF) as u8,
        }),
        0x9000 => Box::new(SkipIfNotRegister {
            register_x: ((opcode & 0x0F00) >> 8) as usize,
            register_y: ((opcode & 0x00F0) >> 4) as usize,
        }),
        0xA000 => Box::new(LoadI {
            value: (opcode & 0x0FFF) as usize,
        }),
        0xD000 => Box::new(Draw {
            register_x: ((opcode & 0x0F00) >> 8) as usize,
            register_y: ((opcode & 0x00F0) >> 4) as usize,
            n_bytes: (opcode & 0x000F) as usize,
        }),

        _ => Box::new(Unknown { opcode }),
    }
}
