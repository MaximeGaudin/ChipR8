use std::ops::Add;

use crate::{VM, vm};


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
        if vm.stack_pointer == 0 {
            panic!("Stack Underflow");
        }

        vm.stack_pointer -= 1;
        vm.program_counter = vm.stack[vm.stack_pointer] as usize;
    }
}

// 1nnn
pub struct Jump {
    pub address: usize,
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
        vm.program_counter = self.address;
    }
}


// 2nnn
pub struct Call {
    pub address: usize,
}

impl Call {
    fn disassemble(&self) -> String {
        format!("CALL {:03X}", self.address)
    }

    fn documentation(&self) -> Vec<&str> {
        vec![
            "Call subroutine at nnn.",
            "TThe interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.",
        ]
    }

    fn execute(&self, vm: &mut VM) {
        if vm.stack_pointer >= vm.stack.len() {
            panic!("Stack Overflow")
        }

        vm.stack[vm.stack_pointer] = vm.program_counter as u16;
        vm.stack_pointer += 1;
        vm.program_counter = self.address;
    }
}

// 3xkk
pub struct SkipIfValue {
    pub register: usize,
    pub value: u8,
}

impl SkipIfValue {
    fn disassemble(&self) -> String {
        format!("SE V{:1X}, {:2X}", self.register, self.value)
    }

    fn documentation(&self) -> Vec<&str> {
        vec![
            "Skip next instruction if Vx = kk.",
            "The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.",
        ]
    }

    fn execute(&self, vm: &mut VM) {
        if vm.registers[self.register] == self.value {
            vm.program_counter += 2
        }
    }
}

// 4xkk
pub struct SkipIfNotValue {
    pub register: usize,
    pub value: u8,
}

impl SkipIfNotValue {
    fn disassemble(&self) -> String {
        format!("SNE V{:1X}, {:2X}", self.register, self.value)
    }

    fn documentation(&self) -> Vec<&str> {
        vec![
            "Skip next instruction if Vx = kk.",
            "The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.",
        ]
    }

    fn execute(&self, vm: &mut VM) {
        if vm.registers[self.register] != self.value {
            vm.program_counter += 2
        }
    }
}


// 5xkk
pub struct SkipIfRegister {
    pub register_x: usize,
    pub register_y: usize,
}

impl SkipIfRegister {
    fn disassemble(&self) -> String {
        format!("SE V{:1X}, V{:1X}", self.register_x, self.register_y)
    }

    fn documentation(&self) -> Vec<&str> {
        vec![
            "Skip next instruction if Vx = Vy.",
            "The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.",
        ]
    }

    fn execute(&self, vm: &mut VM) {
        if vm.registers[self.register_x] == vm.registers[self.register_y]  {
            vm.program_counter += 2
        }
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


// 9xkk
pub struct SkipIfNotRegister {
    pub register_x: usize,
    pub register_y: usize,
}

impl SkipIfNotRegister {
    fn disassemble(&self) -> String {
        format!("SNE V{:1X}, V{:1X}", self.register_x, self.register_y)
    }

    fn documentation(&self) -> Vec<&str> {
        vec![
            "Skip next instruction if Vx != Vy.",
            "The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.",
        ]
    }

    fn execute(&self, vm: &mut VM) {
        if vm.registers[self.register_x] != vm.registers[self.register_y]  {
            vm.program_counter += 2
        }
    }
}

// 7xkk
pub struct AddValue {
    pub register: usize,
    pub value: u8,
}

impl AddValue {
    fn disassemble(&self) -> String {
        format!("ADD V{:1X}, {:2X}", self.register, self.value)
    }

    fn documentation(&self) -> Vec<&str> {
        vec![
            "Set Vx = Vx + kk.",
            "Adds the value kk to the value of register Vx, then stores the result in Vx.",
            "When overflow happen, ignore the overflow."
        ]
    }

    fn execute(&self, vm: &mut VM) {
        vm.registers[self.register] = vm.registers[self.register].wrapping_add(self.value);
    }
}


// Annn
pub struct LoadI {
    pub value: usize,
}

impl LoadI {
    fn disassemble(&self) -> String {
        format!("LD I, {:03X}", self.value)
    }

    fn documentation(&self) -> Vec<&str> {
        vec!["Set I = nnn.", "The value of register I is set to nnn."]
    }

    fn execute(&self, vm: &mut VM) {
        vm.i = self.value;
    }
}

// Dxyn
pub struct Draw {
    pub register_x: usize,
    pub register_y: usize,
    pub n_bytes: usize,
}

impl Draw {
    fn disassemble(&self) -> String {
        format!(
            "DRW V{:1X}, V{:1X}, {:1X}",
            self.register_x, self.register_y, self.n_bytes
        )
    }

    fn documentation(&self) -> Vec<&str> {
        vec![
            "Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.",
            "The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.",
        ]
    }

    fn execute(&self, vm: &mut VM) {
        let start_x = vm.registers[self.register_x] as usize;
        let start_y = vm.registers[self.register_y] as usize;

        // Reset collision register
        vm.registers[0xF] = 0;

        for row in 0..self.n_bytes as usize {
            let byte = vm.memory[vm.i + row];

            for column in 0..8 {
                // 0x80 is 1000_0000 in binary
                // Shifting it right by 'col' moves the 1 to the position we want to check
                let mask = 0x80 >> column;
                let bit = byte & mask;

                if bit != 0 {
                    let screen_x = (start_x + column) % vm::SCREEN_WIDTH;
                    let screen_y = (start_y + row) % vm::SCREEN_HEIGHT;

                    let screen_index = screen_x + (screen_y * vm::SCREEN_WIDTH);

                    // If collision, update the F register
                    if vm.screen[screen_index] == 1 {
                        vm.registers[0xF] = 1;
                    }

                    // XOR the pixel
                    vm.screen[screen_index] ^= 1;
                }
            }
        }
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
            Instruction::LDV(i) => i.disassemble(),
            Instruction::LDI(i) => i.disassemble(),
            Instruction::DRW(i) => i.disassemble(),
            Instruction::ADV(i) => i.disassemble(),
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
            Instruction::JMP(i) => i.execute(vm),
            Instruction::CLS(i) => i.execute(vm),
            Instruction::LDV(i) => i.execute(vm),
            Instruction::LDI(i) => i.execute(vm),
            Instruction::DRW(i) => i.execute(vm),
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
