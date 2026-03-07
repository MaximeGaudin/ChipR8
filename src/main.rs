use macroquad::{audio::{PlaySoundParams, play_sound, stop_sound}, prelude::*};

use std::error::Error;

mod instructions;
use instructions::base::Instruction;

mod screen;
mod sound;
mod vm;

static ROM_DATA: &[u8] = include_bytes!("../roms/corax+.ch8");

fn fetch_decode_execute(vm: &mut vm::VM) {
    let instruction = vm::get_current_instruction(vm);

    if instruction.is_unknown() {
        panic!("{}", instruction.disassemble())
    }

    instruction.execute(vm);
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Chip R8".to_string(),
        window_width: (64 * screen::SCALE as i32),
        window_height: (32 * screen::SCALE as i32),
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut vm = vm::init(vm::EmulationMode::Chip8);

    let buzz = sound::generate_beep().await;
    let mut is_buzzing = false;

    let opcode_per_frame = vm::CPU_TICK_RATE / screen::FRAME_RATE;

    vm::load_rom(ROM_DATA, &mut vm).unwrap();

    loop {
        for _ in 0..opcode_per_frame {
            fetch_decode_execute(&mut vm);
        }

        if vm.sound_timer_register > 0 {
            if !is_buzzing {
                is_buzzing = true;
                play_sound(&buzz, PlaySoundParams {
                    looped: true,
                    volume: 1.0,
                });
            }
        } else {
            is_buzzing = false;
            stop_sound(&buzz);
        }

        screen::render(&mut vm);
        vm::update_timer(&mut vm);

        next_frame().await
    }
}
