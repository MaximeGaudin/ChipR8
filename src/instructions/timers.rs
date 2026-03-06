use crate::{instructions::base::Instruction, vm::VM};

pub(super) struct SetDelayTimer {
    pub register: usize,
}

pub(super) struct SetSoundTimer {
    pub register: usize,
}

pub(super) struct LoadDelayTimerIntoRegister {
    pub register: usize,
}

// Fx07 - LD Vx, DT
// Set Vx = delay timer value.
// The value of DT is placed into Vx.
impl Instruction for LoadDelayTimerIntoRegister {
    fn disassemble(&self) -> String {
        format!("LD V{:1X}, DT", self.register)
    }

    fn execute(&self, vm: &mut VM) {
        vm.registers[self.register] = vm.delay_timer_register
    }
}


// Fx15 - LD DT, Vx
// Set delay timer = Vx.
// DT is set equal to the value of Vx.
impl Instruction for SetDelayTimer {
    fn disassemble(&self) -> String {
        format!("LD DT, V{:1X}", self.register)
    }

    fn execute(&self, vm: &mut VM) {
        vm.delay_timer_register = vm.registers[self.register];
    }
}



// Fx18 - LD ST, Vx
// Set sound timer = Vx.
// ST is set equal to the value of Vx.
impl Instruction for SetSoundTimer {
    fn disassemble(&self) -> String {
        format!("LD ST, V{:1X}", self.register)
    }

    fn execute(&self, vm: &mut VM) {
        vm.sound_timer_register = vm.registers[self.register];
    }
}
