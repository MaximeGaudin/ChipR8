use std::{env, error::Error};

mod instructions;
use instructions::base::Instruction;

mod screen;
mod vm;

fn fetch_decode_execute(vm: &mut vm::VM) {
    let instruction = vm::get_current_instruction(vm);

    // println!("{}", instruction.disassemble());

    if instruction.is_unknown() {
        panic!("NOT IMPLEMENTED")
    }

    instruction.execute(vm);
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let mut vm = vm::init(vm::EmulationMode::Chip8);
    let mut raylib_context = screen::init();
    let opcode_per_frame = vm::CPU_TICK_RATE / screen::FRAME_RATE;

    let rom_path = if args.len() > 1 {
        format!("roms/{}.ch8", &args[1])
    } else {
        "roms/corax+.ch8".to_string()
    };
    vm::load_rom(rom_path, &mut vm).unwrap();

    while !raylib_context.handle.window_should_close() {
        for _ in 0..opcode_per_frame {
            fetch_decode_execute(&mut vm);
        }

        screen::render(&mut vm, &mut raylib_context);
    }

    Ok(())
}
