use crate::{VM, vm};

// 1nnn
pub(super) struct Jump {
    pub address: usize,
}

impl Jump {
    pub(super) fn disassemble(&self) -> String {
        format!("JMP {:03X}", self.address)
    }

    pub(super) fn documentation(&self) -> Vec<&str> {
        vec![
            "Jump to location nnn.",
            "The interpreter sets the program counter to nnn.",
        ]
    }

    pub(super) fn execute(&self, vm: &mut VM) {
        vm.program_counter = self.address;
    }
}

// 6kkk
pub struct LoadValue {
    pub register: usize,
    pub value: u8,
}

impl LoadValue {
    pub(super) fn disassemble(&self) -> String {
        format!("LD V{:1X}, {:2X}", self.register, self.value)
    }

    pub(super) fn documentation(&self) -> Vec<&str> {
        vec![
            "Set Vx = kk.",
            "The interpreter puts the value kk into register Vx.",
        ]
    }

    pub(super) fn execute(&self, vm: &mut VM) {
        vm.registers[self.register] = self.value;
    }
}

// 7xkk
pub struct AddValue {
    pub register: usize,
    pub value: u8,
}

impl AddValue {
    pub(super) fn disassemble(&self) -> String {
        format!("ADD V{:1X}, {:2X}", self.register, self.value)
    }

    pub(super) fn documentation(&self) -> Vec<&str> {
        vec![
            "Set Vx = Vx + kk.",
            "Adds the value kk to the value of register Vx, then stores the result in Vx.",
            "When overflow happen, ignore the overflow.",
        ]
    }

    pub(super) fn execute(&self, vm: &mut VM) {
        vm.registers[self.register] = vm.registers[self.register].wrapping_add(self.value);
    }
}

// Annn
pub struct LoadI {
    pub value: usize,
}

impl LoadI {
    pub(super) fn disassemble(&self) -> String {
        format!("LD I, {:03X}", self.value)
    }

    pub(super) fn documentation(&self) -> Vec<&str> {
        vec!["Set I = nnn.", "The value of register I is set to nnn."]
    }

    pub(super) fn execute(&self, vm: &mut VM) {
        vm.i = self.value;
    }
}
