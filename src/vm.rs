use crate::instructions::base::Instruction;
use crate::instructions::base::opcode_to_instruction;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
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
    pub screen: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],
    pub memory: [u8; 4096],
    pub registers: [u8; 16],
    pub stack: [u16; 16],

    pub i: usize,
    pub program_counter: usize,
    pub stack_pointer: usize,
}

pub fn init() -> VM {
    let mut vm = VM {
        screen: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
        memory: [0; 4096],
        registers: [0; 16],
        stack: [0; 16],

        i: 0,
        program_counter: PROGRAM_START,
        stack_pointer: 0,
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

pub fn get_current_instruction(vm: &mut VM) -> Box<dyn Instruction> {
    let b1 = vm.memory[vm.program_counter] as u16;
    let b2 = vm.memory[vm.program_counter + 1] as u16;

    vm.program_counter += 2;

    let opcode = (b1 << 8) | b2;
    // println!("{:04X}", opcode);
    opcode_to_instruction(opcode)
}
