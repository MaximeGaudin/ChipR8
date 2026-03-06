use crate::vm;
use crate::vm::VM;

// 3xkk
pub struct SkipIfValue {
    pub register: usize,
    pub value: u8,
}

impl SkipIfValue {
    pub(super) fn disassemble(&self) -> String {
        format!("SE V{:1X}, {:2X}", self.register, self.value)
    }

    pub(super) fn documentation(&self) -> Vec<&str> {
        vec![
            "Skip next instruction if Vx = kk.",
            "The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.",
        ]
    }

    pub(super) fn execute(&self, vm: &mut VM) {
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
    pub(super) fn disassemble(&self) -> String {
        format!("SNE V{:1X}, {:2X}", self.register, self.value)
    }

    pub(super) fn documentation(&self) -> Vec<&str> {
        vec![
            "Skip next instruction if Vx = kk.",
            "The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.",
        ]
    }

    pub(super) fn execute(&self, vm: &mut VM) {
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
    pub(super) fn disassemble(&self) -> String {
        format!("SE V{:1X}, V{:1X}", self.register_x, self.register_y)
    }

    pub(super) fn documentation(&self) -> Vec<&str> {
        vec![
            "Skip next instruction if Vx = Vy.",
            "The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.",
        ]
    }

    pub(super) fn execute(&self, vm: &mut VM) {
        if vm.registers[self.register_x] == vm.registers[self.register_y]  {
            vm.program_counter += 2
        }
    }
}


// 9xkk
pub struct SkipIfNotRegister {
    pub register_x: usize,
    pub register_y: usize,
}

impl SkipIfNotRegister {
    pub(super) fn disassemble(&self) -> String {
        format!("SNE V{:1X}, V{:1X}", self.register_x, self.register_y)
    }

    pub(super) fn documentation(&self) -> Vec<&str> {
        vec![
            "Skip next instruction if Vx != Vy.",
            "The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.",
        ]
    }

    pub(super) fn execute(&self, vm: &mut VM) {
        if vm.registers[self.register_x] != vm.registers[self.register_y]  {
            vm.program_counter += 2
        }
    }
}