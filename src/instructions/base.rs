use crate::instructions::arithmetic::AddRegisterToI;
use crate::instructions::arithmetic::AddRegisterToRegister;
use crate::instructions::arithmetic::AddValueToRegister;
use crate::instructions::arithmetic::And;
use crate::instructions::arithmetic::LoadRegisterToRegister;
use crate::instructions::arithmetic::Or;
use crate::instructions::arithmetic::ShiftLeftRegisterToRegister;
use crate::instructions::arithmetic::ShiftRightRegisterToRegister;
use crate::instructions::arithmetic::SubRegisterToRegister;
use crate::instructions::arithmetic::SubReverseRegisterToRegister;
use crate::instructions::arithmetic::Xor;
use crate::instructions::keyboard::SkipIfKeyNotPressed;
use crate::instructions::keyboard::SkipIfKeyPressed;
use crate::instructions::memory::LoadBCD;
use crate::instructions::memory::LoadMemoryIntoRegisters;
use crate::instructions::memory::LoadRandomIntoRegister;
use crate::instructions::memory::LoadRegistersIntoMemory;
use crate::instructions::memory::LoadValueToI;
use crate::instructions::memory::LoadValueToRegister;
use crate::instructions::other::*;
use crate::instructions::screen::*;
use crate::instructions::skip::*;
use crate::instructions::subroutines::*;
use crate::instructions::timers::LoadDelayTimerIntoRegister;
use crate::instructions::timers::SetDelayTimer;
use crate::instructions::timers::SetSoundTimer;
use crate::vm::VM;

pub trait Instruction {
    fn execute(&self, vm: &mut VM);
    fn disassemble(&self) -> String;
    fn is_unknown(&self) -> bool {
        false
    }
}

pub(super) struct Invalid {
    pub opcode: u16,
}

impl Instruction for Invalid {
    fn disassemble(&self) -> String {
        format!("Invalid: {:04X}", self.opcode)
    }

    fn execute(&self, _vm: &mut VM) {
        // Nothing to do
    }

    fn is_unknown(&self) -> bool {
        true
    }
}

pub fn opcode_to_instruction(opcode: u16) -> Box<dyn Instruction> {
    match opcode & 0xF000 {
        0x0000 => match opcode {
            0x00E0 => Box::new(ClearScreen {}),
            0x00EE => Box::new(Return {}),
            _ => Box::new(Invalid { opcode }),
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
        0x6000 => Box::new(LoadValueToRegister {
            register: ((opcode & 0x0F00) >> 8) as usize,
            value: (opcode & 0x00FF) as u8,
        }),
        0x7000 => Box::new(AddValueToRegister {
            register: ((opcode & 0x0F00) >> 8) as usize,
            value: (opcode & 0x00FF) as u8,
        }),
        0x8000 => match opcode & 0x000F {
            0x0000 => Box::new(LoadRegisterToRegister {
                register_x: ((opcode & 0x0F00) >> 8) as usize,
                register_y: ((opcode & 0x00F0) >> 4) as usize,
            }),
            0x0001 => Box::new(Or {
                register_x: ((opcode & 0x0F00) >> 8) as usize,
                register_y: ((opcode & 0x00F0) >> 4) as usize,
            }),
            0x0002 => Box::new(And {
                register_x: ((opcode & 0x0F00) >> 8) as usize,
                register_y: ((opcode & 0x00F0) >> 4) as usize,
            }),
            0x0003 => Box::new(Xor {
                register_x: ((opcode & 0x0F00) >> 8) as usize,
                register_y: ((opcode & 0x00F0) >> 4) as usize,
            }),
            0x0004 => Box::new(AddRegisterToRegister {
                register_x: ((opcode & 0x0F00) >> 8) as usize,
                register_y: ((opcode & 0x00F0) >> 4) as usize,
            }),
            0x0005 => Box::new(SubRegisterToRegister {
                register_x: ((opcode & 0x0F00) >> 8) as usize,
                register_y: ((opcode & 0x00F0) >> 4) as usize,
            }),
            0x0007 => Box::new(SubReverseRegisterToRegister {
                register_x: ((opcode & 0x0F00) >> 8) as usize,
                register_y: ((opcode & 0x00F0) >> 4) as usize,
            }),
            0x0006 => Box::new(ShiftRightRegisterToRegister {
                register_x: ((opcode & 0x0F00) >> 8) as usize,
                register_y: ((opcode & 0x00F0) >> 4) as usize,
            }),
            0x000E => Box::new(ShiftLeftRegisterToRegister {
                register_x: ((opcode & 0x0F00) >> 8) as usize,
                register_y: ((opcode & 0x00F0) >> 4) as usize,
            }),
            _ => Box::new(Invalid { opcode }),
        },
        0x9000 => Box::new(SkipIfNotRegister {
            register_x: ((opcode & 0x0F00) >> 8) as usize,
            register_y: ((opcode & 0x00F0) >> 4) as usize,
        }),
        0xA000 => Box::new(LoadValueToI {
            value: (opcode & 0x0FFF) as usize,
        }),
        0xC000 => Box::new(LoadRandomIntoRegister {
            register: ((opcode & 0x0F00) >> 8) as usize,
            mask: (opcode & 0x00FF) as u8,
        }),
        0xD000 => Box::new(Draw {
            register_x: ((opcode & 0x0F00) >> 8) as usize,
            register_y: ((opcode & 0x00F0) >> 4) as usize,
            n_bytes: (opcode & 0x000F) as usize,
        }),
        0xE000 => match opcode & 0x00FF {
            0x009E => Box::new(SkipIfKeyPressed {
                register: ((opcode & 0x0F00) >> 8) as usize,
            }),
            0x00A1 => Box::new(SkipIfKeyNotPressed {
                register: ((opcode & 0x0F00) >> 8) as usize,
            }),
            _ => Box::new(Invalid { opcode }),
        },
        0xF000 => match opcode & 0x00FF {
            0x0007 => Box::new(LoadDelayTimerIntoRegister {
                register: ((opcode & 0x0F00) >> 8) as usize,
            }),
            0x0015 => Box::new(SetDelayTimer {
                register: ((opcode & 0x0F00) >> 8) as usize,
            }),
            0x0018 => Box::new(SetSoundTimer {
                register: ((opcode & 0x0F00) >> 8) as usize,
            }),
            0x0055 => Box::new(LoadRegistersIntoMemory {
                register: ((opcode & 0x0F00) >> 8) as usize,
            }),
            0x0065 => Box::new(LoadMemoryIntoRegisters {
                register: ((opcode & 0x0F00) >> 8) as usize,
            }),
            0x0033 => Box::new(LoadBCD {
                register: ((opcode & 0x0F00) >> 8) as usize,
            }),
            0x001E => Box::new(AddRegisterToI {
                register: ((opcode & 0x0F00) >> 8) as usize,
            }),
            _ => Box::new(Invalid { opcode }),
        },

        _ => Box::new(Invalid { opcode }),
    }
}
