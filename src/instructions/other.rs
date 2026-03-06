use crate::{VM, instructions::base::Instruction, vm};

pub(super) struct Jump {
    pub address: usize,
}

pub struct LoadI {
    pub value: usize,
}

pub(super) struct LoadValue {
    pub register: usize,
    pub value: u8,
}

pub struct AddValue {
    pub register: usize,
    pub value: u8,
}

impl Instruction for Jump {
    fn disassemble(&self) -> String {
        format!("JMP {:03X}", self.address)
    }

    // 1nnn
    // Jump to location nnn.",
    // The interpreter sets the program counter to nnn.
    fn execute(&self, vm: &mut VM) {
        vm.program_counter = self.address;
    }
}

impl Instruction for LoadValue {
    fn disassemble(&self) -> String {
        format!("LD V{:1X}, {:2X}", self.register, self.value)
    }

    // 6kkk
    // Set Vx = kk.
    // The interpreter puts the value kk into register Vx.
    fn execute(&self, vm: &mut VM) {
        vm.registers[self.register] = self.value;
    }
}

impl Instruction for AddValue {
    fn disassemble(&self) -> String {
        format!("ADD V{:1X}, {:2X}", self.register, self.value)
    }

    // 7xkk
    // Set Vx = Vx + kk.
    // Adds the value kk to the value of register Vx, then stores the result in Vx.
    // When overflow happen, ignore the overflow.
    fn execute(&self, vm: &mut VM) {
        vm.registers[self.register] = vm.registers[self.register].wrapping_add(self.value);
    }
}

impl Instruction for LoadI {
    fn disassemble(&self) -> String {
        format!("LD I, {:03X}", self.value)
    }

    // Annn
    // Set I = nnn.", "The value of register I is set to nnn.
    fn execute(&self, vm: &mut VM) {
        vm.i = self.value;
    }
}
