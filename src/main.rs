use raylib::prelude::*;
use std::error::Error;

mod instructions;
use instructions::*;

mod vm;
use vm::VM;

mod screen;

mod tests;

fn fetch_decode_execute(vm: &mut VM) {
    let instruction = vm::get_current_instruction(vm);

    println!("{}", instruction.disassemble());
    instruction.execute(vm);
}

fn main() -> Result<(), Box<dyn Error>> {
    // 1. Boot: Init memory, init PC, init registers...
    // 2. Load Rom into Memory
    // 2. Start the fetch_decode_execute cycle
    let mut vm = vm::init();
    let mut raylib_context = screen::init();

    vm::load_rom("roms/chip8-logo.ch8".to_string(), &mut vm).unwrap();

    // Boucle principale
    while !raylib_context.handle.window_should_close() {
        fetch_decode_execute(&mut vm);
        screen::render(&mut vm, &mut raylib_context);
    }

    Ok(())
}
