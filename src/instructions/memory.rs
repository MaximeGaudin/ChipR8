use rand::{Rng, rng};

use crate::{instructions::base::Instruction, vm::VM};

pub struct LoadValueToI {
    pub value: usize,
}

pub(super) struct LoadValueToRegister {
    pub register: usize,
    pub value: u8,
}

pub(super) struct LoadRegistersIntoMemory {
    pub register: usize,
}

pub(super) struct LoadMemoryIntoRegisters {
    pub register: usize,
}

pub(super) struct LoadBCD {
    pub register: usize,
}

pub(super) struct LoadRandomIntoRegister {
    pub register: usize,
    pub mask: u8,
}

// 6kkk
// Set Vx = kk.
// The interpreter puts the value kk into register Vx.
impl Instruction for LoadValueToRegister {
    fn disassemble(&self) -> String {
        format!("LD V{:1X}, {:2X}", self.register, self.value)
    }

    fn execute(&self, vm: &mut VM) {
        vm.registers[self.register] = self.value;
    }
}

// Annn
// Set I = nnn.", "The value of register I is set to nnn.
impl Instruction for LoadValueToI {
    fn disassemble(&self) -> String {
        format!("LD I, {:03X}", self.value)
    }

    fn execute(&self, vm: &mut VM) {
        vm.i = self.value;
    }
}

// Fx55 - LD [I], Vx
// Store registers V0 through Vx in memory starting at location I.
// The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
impl Instruction for LoadRegistersIntoMemory {
    fn disassemble(&self) -> String {
        format!("LD [I], {:1X}", self.register)
    }

    fn execute(&self, vm: &mut VM) {
        for j in 0..self.register {
            vm.memory[vm.i + j] = vm.registers[j];
        }
    }
}

// Fx65 - LD Vx, [I]
// Read registers V0 through Vx from memory starting at location I.
// The interpreter reads values from memory starting at location I into registers V0 through Vx.
impl Instruction for LoadMemoryIntoRegisters {
    fn disassemble(&self) -> String {
        format!("LD V{:1X}, [I]", self.register)
    }

    fn execute(&self, vm: &mut VM) {
        for j in 0..self.register {
            vm.registers[j] = vm.memory[vm.i + j];
        }
    }
}

// Fx33 - LD B, Vx
// Store BCD representation of Vx in memory locations I, I+1, and I+2.
// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
impl Instruction for LoadBCD {
    fn disassemble(&self) -> String {
        format!("LD B, V{:1X}", self.register)
    }

    fn execute(&self, vm: &mut VM) {
        // TODO
    }
}

// Cxkk - RND Vx, byte
// Set Vx = random byte AND kk.
// The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk.
// The results are stored in Vx.
//  See instruction 8xy2 for more information on AND.
impl Instruction for LoadRandomIntoRegister {
    fn execute(&self, vm: &mut VM) {
        let random_byte: u8 = rand::random();
        vm.registers[self.register] = random_byte & self.mask;
    }

    fn disassemble(&self) -> String {
        format!("RND V{:1X}, {:2X}", self.register, self.mask)
    }
}
