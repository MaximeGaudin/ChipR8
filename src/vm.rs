use crate::instructions::*;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
const PROGRAM_START: usize = 0x200;

pub struct VM {
    pub memory: [u8; 4096],
    pub screen: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],
    pub registers: [u8; 16],
    pub i: u16,
    pub program_counter: usize,
}

pub fn init() -> VM {
    let mut vm = VM {
        memory: [0; 4096],
        screen: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
        registers: [0; 16],
        i: 0,
        program_counter: PROGRAM_START,
    };

    for i in 0..FONT_SET.len() {
        vm.memory[i] = FONT_SET[i]
    }

    return vm;
}

pub fn load_rom(path: String, vm: &mut VM) -> Result<(), std::io::Error> {
    let content = std::fs::read(path)?;
    for i in 0..content.len() {
        vm.memory[PROGRAM_START + i] = content[i];
    }

    Ok(())
}

fn opcode_to_instruction(opcode: u16) -> Instruction {
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
        _ => Instruction::UNK(Unknown { opcode }),
    }
}

pub fn get_current_instruction(vm: &mut VM) -> Instruction {
    let b1 = vm.memory[vm.program_counter] as u16;
    let b2 = vm.memory[vm.program_counter + 1] as u16;

    vm.program_counter += 2;

    let opcode = (b1 << 8) | b2;
    println!("{:04X}", opcode);
    opcode_to_instruction(opcode)
}
