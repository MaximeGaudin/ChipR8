use crate::{
    instructions::base::Instruction,
    vm::{EmulationMode, VM},
};

pub(super) struct LoadRegisterToRegister {
    pub register_x: usize,
    pub register_y: usize,
}

pub(super) struct Or {
    pub register_x: usize,
    pub register_y: usize,
}

pub(super) struct And {
    pub register_x: usize,
    pub register_y: usize,
}

pub(super) struct Xor {
    pub register_x: usize,
    pub register_y: usize,
}

pub(super) struct AddRegisterToRegister {
    pub register_x: usize,
    pub register_y: usize,
}
pub struct AddValueToRegister {
    pub register: usize,
    pub value: u8,
}

pub struct SubRegisterToRegister {
    pub register_x: usize,
    pub register_y: usize,
}

pub struct SubReverseRegisterToRegister {
    pub register_x: usize,
    pub register_y: usize,
}

pub struct ShiftRightRegisterToRegister {
    pub register_x: usize,
    pub register_y: usize,
}

pub struct ShiftLeftRegisterToRegister {
    pub register_x: usize,
    pub register_y: usize,
}

pub struct AddRegisterToI {
    pub register: usize,
}

// 8xy0 - LD Vx, Vy
// Set Vx = Vy.
// Stores the value of register Vy in register Vx.
impl Instruction for LoadRegisterToRegister {
    fn execute(&self, vm: &mut VM) {
        vm.registers[self.register_x] = vm.registers[self.register_y];
    }

    fn disassemble(&self) -> String {
        format!("LD V{:1X}, V{:1X}", self.register_x, self.register_y)
    }
}

// 8xy1 - OR Vx, Vy
// Set Vx = Vx OR Vy.
// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
// A bitwise OR compares the corrseponding bits from two values, and if either bit is 1, then the same bit in the result is also 1.
// Otherwise, it is 0.
impl Instruction for Or {
    fn execute(&self, vm: &mut VM) {
        vm.registers[self.register_x] |= vm.registers[self.register_y];
    }

    fn disassemble(&self) -> String {
        format!("OR V{:1X}, V{:1X}", self.register_x, self.register_y)
    }
}

// 8xy2 - AND Vx, Vy
// Set Vx = Vx AND Vy.
// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
// A bitwise AND compares the corrseponding bits from two values, and if both bits are 1, then the same bit in the result is also 1.
// Otherwise, it is 0.
impl Instruction for And {
    fn execute(&self, vm: &mut VM) {
        vm.registers[self.register_x] &= vm.registers[self.register_y];
    }

    fn disassemble(&self) -> String {
        format!("AND V{:1X}, V{:1X}", self.register_x, self.register_y)
    }
}

// 8xy3 - XOR Vx, Vy
// Set Vx = Vx XOR Vy.
// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
// An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1.
// Otherwise, it is 0.
impl Instruction for Xor {
    fn execute(&self, vm: &mut VM) {
        vm.registers[self.register_x] ^= vm.registers[self.register_y];
    }

    fn disassemble(&self) -> String {
        format!("XOR V{:1X}, V{:1X}", self.register_x, self.register_y)
    }
}

// 8xy4 - ADD Vx, Vy
// Set Vx = Vx + Vy, set VF = carry.
// The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
// Only the lowest 8 bits of the result are kept, and stored in Vx.
impl Instruction for AddRegisterToRegister {
    fn execute(&self, vm: &mut VM) {
        let value_x = vm.registers[self.register_x] as u16;
        let value_y = vm.registers[self.register_y] as u16;

        let carry = if value_x + value_y > 0xFF { 1 } else { 0 };

        // It will naturally truncate/wrap when cast back to u8
        vm.registers[self.register_x] = (value_x + value_y) as u8;

        // Set the carry last in case one of the register in the operands is 0xF
        vm.registers[0xF] = carry;
    }

    fn disassemble(&self) -> String {
        format!("ADD V{:1X}, V{:1X}", self.register_x, self.register_y)
    }
}

// 7xkk
// Set Vx = Vx + kk.
// Adds the value kk to the value of register Vx, then stores the result in Vx.
// When overflow happen, ignore the overflow.
impl Instruction for AddValueToRegister {
    fn disassemble(&self) -> String {
        format!("ADD V{:1X}, {:2X}", self.register, self.value)
    }

    fn execute(&self, vm: &mut VM) {
        vm.registers[self.register] = vm.registers[self.register].wrapping_add(self.value);
    }
}

// 8xy5 - SUB Vx, Vy
// Set Vx = Vx - Vy, set VF = NOT borrow.
// If Vx > Vy, then VF is set to 1, otherwise 0.
// Then Vy is subtracted from Vx, and the results stored in Vx.
impl Instruction for SubRegisterToRegister {
    fn execute(&self, vm: &mut VM) {
        let value_x = vm.registers[self.register_x];
        let value_y = vm.registers[self.register_y];

        let carry = if value_x >= value_y { 1 } else { 0 };

        // It will naturally truncate/wrap when cast back to u8
        vm.registers[self.register_x] = value_x.wrapping_sub(value_y);

        // Set the carry last in case one of the register in the operands is 0xF
        vm.registers[0xF] = carry;
    }

    fn disassemble(&self) -> String {
        format!("SUB V{:1X}, V{:1X}", self.register_x, self.register_y)
    }
}

// 8xy7 - SUBN Vx, Vy
// Set Vx = Vy - Vx, set VF = NOT borrow.
// If Vy > Vx, then VF is set to 1, otherwise 0.
// Then Vx is subtracted from Vy, and the results stored in Vx.
impl Instruction for SubReverseRegisterToRegister {
    fn execute(&self, vm: &mut VM) {
        let value_x = vm.registers[self.register_x];
        let value_y = vm.registers[self.register_y];

        let carry = if value_y >= value_x { 1 } else { 0 };

        // It will naturally truncate/wrap when cast back to u8
        vm.registers[self.register_x] = value_y.wrapping_sub(value_x);

        // Set the carry last in case one of the register in the operands is 0xF
        vm.registers[0xF] = carry;
    }

    fn disassemble(&self) -> String {
        format!("SUBN V{:1X}, V{:1X}", self.register_x, self.register_y)
    }
}

// 8xy6 - SHR Vx {, Vy}
// Set Vx = Vx SHR 1.
// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0.
// Then Vx is divided by 2.
impl Instruction for ShiftRightRegisterToRegister {
    fn execute(&self, vm: &mut VM) {
        let initial_value = if vm.mode == EmulationMode::Chip8 {
            vm.registers[self.register_y]
        } else {
            vm.registers[self.register_x]
        };

        let flag = initial_value & 0x01;
        vm.registers[self.register_x] = initial_value >> 1;
        vm.registers[0xF] = flag;
    }

    fn disassemble(&self) -> String {
        format!("SHR V{:1X}, V{:1X}", self.register_x, self.register_y)
    }
}

// 8xyE - SHL Vx {, Vy}
// Set Vx = Vx SHL 1.

// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
// Then Vx is multiplied by 2.
impl Instruction for ShiftLeftRegisterToRegister {
    fn execute(&self, vm: &mut VM) {
        let initial_value = if vm.mode == EmulationMode::Chip8 {
            vm.registers[self.register_y]
        } else {
            vm.registers[self.register_x]
        };

        let flag = (initial_value & 0x80) >> 7;
        vm.registers[self.register_x] = initial_value << 1;
        vm.registers[0xF] = flag;
    }

    fn disassemble(&self) -> String {
        format!("SHL V{:1X}, V{:1X}", self.register_x, self.register_y)
    }
}

// Fx1E - ADD I, Vx
// Set I = I + Vx.
// The values of I and Vx are added, and the results are stored in I.
impl Instruction for AddRegisterToI {
    fn execute(&self, vm: &mut VM) {
        vm.i += vm.registers[self.register] as usize;
    }

    fn disassemble(&self) -> String {
        format!("ADD I, V{:1X}", self.register)
    }
}
