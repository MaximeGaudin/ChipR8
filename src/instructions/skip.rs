use super::base::Instruction;
use crate::vm::VM;

pub(super) struct SkipIfValue {
    pub register: usize,
    pub value: u8,
}

pub(super) struct SkipIfRegister {
    pub register_x: usize,
    pub register_y: usize,
}

pub(super) struct SkipIfNotValue {
    pub register: usize,
    pub value: u8,
}

pub(super) struct SkipIfNotRegister {
    pub register_x: usize,
    pub register_y: usize,
}

impl Instruction for SkipIfValue {
    fn disassemble(&self) -> String {
        format!("SE V{:1X}, {:2X}", self.register, self.value)
    }

    // 3xkk
    // Skip next instruction if Vx = kk.
    // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    fn execute(&self, vm: &mut VM) {
        if vm.registers[self.register] == self.value {
            vm.program_counter += 2
        }
    }
}

impl Instruction for SkipIfNotValue {
    fn disassemble(&self) -> String {
        format!("SNE V{:1X}, {:2X}", self.register, self.value)
    }

    // 4xkk
    // Skip next instruction if Vx = kk.
    // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    fn execute(&self, vm: &mut VM) {
        if vm.registers[self.register] != self.value {
            vm.program_counter += 2
        }
    }
}

impl Instruction for SkipIfRegister {
    fn disassemble(&self) -> String {
        format!("SE V{:1X}, V{:1X}", self.register_x, self.register_y)
    }

    // 5xkk
    // Skip next instruction if Vx = Vy.
    //  The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    fn execute(&self, vm: &mut VM) {
        if vm.registers[self.register_x] == vm.registers[self.register_y] {
            vm.program_counter += 2
        }
    }
}

impl Instruction for SkipIfNotRegister {
    fn disassemble(&self) -> String {
        format!("SNE V{:1X}, V{:1X}", self.register_x, self.register_y)
    }

    // 9xkk
    // Skip next instruction if Vx != Vy.
    // The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
    fn execute(&self, vm: &mut VM) {
        if vm.registers[self.register_x] != vm.registers[self.register_y] {
            vm.program_counter += 2
        }
    }
}
