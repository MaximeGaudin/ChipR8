use std::error::Error;

mod instructions;
use instructions::base::Instruction;

mod vm;
mod screen;

fn fetch_decode_execute(vm: &mut vm::VM) {
    let instruction = vm::get_current_instruction(vm);

    println!("{}", instruction.disassemble());

    if instruction.is_unknown() {
        panic!("NOT IMPLEMENTED")
    }

    instruction.execute(vm);
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut vm = vm::init(vm::EmulationMode::Chip8);
    let mut raylib_context = screen::init();
    let opcode_per_frame = vm::CPU_TICK_RATE / screen::FRAME_RATE;

    vm::load_rom("roms/maze.ch8".to_string(), &mut vm).unwrap();

    // Boucle principale
    while !raylib_context.handle.window_should_close() {
        for _ in 0..opcode_per_frame {
            fetch_decode_execute(&mut vm);
        }

        screen::render(&mut vm, &mut raylib_context);
    }

    Ok(())
}
