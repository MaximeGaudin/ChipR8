use crate::Instruction;
use crate::vm;
use crate::vm::VM;

pub(super) struct ClearScreen {}

pub(super) struct Draw {
    pub register_x: usize,
    pub register_y: usize,
    pub n_bytes: usize,
}

impl Instruction for ClearScreen {
    fn disassemble(&self) -> String {
        "CLS".to_string()
    }

    // 00E0
    fn execute(&self, vm: &mut VM) {
        vm.screen.fill(0);
    }
}

impl Instruction for Draw {
    fn disassemble(&self) -> String {
        format!(
            "DRW V{:1X}, V{:1X}, {:1X}",
            self.register_x, self.register_y, self.n_bytes
        )
    }

    // Dxyn
    // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.",
    // The interpreter reads n bytes from memory, starting at the address stored in I.
    // These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
    // Sprites are XORed onto the existing screen.
    // If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
    // If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen.
    // See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
    fn execute(&self, vm: &mut VM) {
        let start_x = vm.registers[self.register_x] as usize;
        let start_y = vm.registers[self.register_y] as usize;

        // Reset collision register
        vm.registers[0xF] = 0;

        for row in 0..self.n_bytes as usize {
            let byte = vm.memory[vm.i + row];

            for column in 0..8 {
                // 0x80 is 1000_0000 in binary
                // Shifting it right by 'col' moves the 1 to the position we want to check
                let mask = 0x80 >> column;
                let bit = byte & mask;

                if bit != 0 {
                    let screen_x = (start_x + column) % vm::SCREEN_WIDTH;
                    let screen_y = (start_y + row) % vm::SCREEN_HEIGHT;

                    let screen_index = screen_x + (screen_y * vm::SCREEN_WIDTH);

                    // If collision, update the F register
                    if vm.screen[screen_index] == 1 {
                        vm.registers[0xF] = 1;
                    }

                    // XOR the pixel
                    vm.screen[screen_index] ^= 1;
                }
            }
        }
    }
}
