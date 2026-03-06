use std::error::Error;

mod instructions;
use instructions::base::Instruction;

mod vm;
use vm::VM;

mod screen;

mod tests;

fn fetch_decode_execute(vm: &mut VM) {
    let instruction = vm::get_current_instruction(vm);

    println!("{}", instruction.disassemble());

    if instruction.is_unknown() {
        panic!("NOT IMPLEMENTED")
    }

    instruction.execute(vm);
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut vm = vm::init();
    let mut raylib_context = screen::init();

    vm::load_rom("roms/ibm-logo.ch8".to_string(), &mut vm).unwrap();

    // Boucle principale
    while !raylib_context.handle.window_should_close() {
        fetch_decode_execute(&mut vm);
        screen::render(&mut vm, &mut raylib_context);
    }

    Ok(())
}
