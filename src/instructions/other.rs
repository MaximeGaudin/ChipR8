use crate::{instructions::base::Instruction, vm::VM};

pub(super) struct Jump {
    pub address: usize,
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
