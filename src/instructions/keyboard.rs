use crate::{instructions::base::Instruction, vm::VM};

pub(super) struct SkipIfKeyPressed {
    pub(super) register: usize,
}

pub(super) struct SkipIfKeyNotPressed {
   pub(super) register: usize,
}

// Ex9E - SKP Vx
// Skip next instruction if key with the value of Vx is pressed.
// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
impl Instruction for SkipIfKeyPressed {
    fn disassemble(&self) -> String {
        format!("SKP V{:1X}", self.register)
    }

    fn execute(&self, vm: &mut VM) {
        todo!()
    }
}

// ExA1 - SKNP Vx
// Skip next instruction if key with the value of Vx is not pressed.
// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
impl Instruction for SkipIfKeyNotPressed {
    fn disassemble(&self) -> String {
        format!("SKNP V{:1X}", self.register)
    }

    fn execute(&self, vm: &mut VM) {
        todo!()
    }
}