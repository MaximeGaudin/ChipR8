use crate::vm::VM;

pub struct Return {}

impl Return {
    pub(super) fn disassemble(&self) -> String {
        "RET".to_string()
    }

    // 00E0
    // Return from a subroutine.
    // The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
    pub(super) fn execute(&self, vm: &mut VM) {
        if vm.stack_pointer == 0 {
            panic!("Stack Underflow");
        }

        vm.stack_pointer -= 1;
        vm.program_counter = vm.stack[vm.stack_pointer] as usize;
    }
}

pub(super) struct Call {
    pub address: usize,
}

impl Call {
    pub(super) fn disassemble(&self) -> String {
        format!("CALL {:03X}", self.address)
    }

    // 2nnn
    // Call subroutine at nnn.
    // The interpreter increments the stack pointer, then puts the current PC on the top of the stack. 
    // The PC is then set to nnn."
    pub(super) fn execute(&self, vm: &mut VM) {
        if vm.stack_pointer >= vm.stack.len() {
            panic!("Stack Overflow")
        }

        vm.stack[vm.stack_pointer] = vm.program_counter as u16;
        vm.stack_pointer += 1;
        vm.program_counter = self.address;
    }
}