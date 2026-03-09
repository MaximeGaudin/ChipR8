use macroquad::{
    audio::{PlaySoundParams, play_sound, stop_sound},
    prelude::*,
};

use std::error::Error;

use include_dir::{Dir, include_dir};

mod instructions;
use instructions::base::Instruction;

mod screen;
mod sound;
mod vm;

static ROMS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/roms");

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
    const TIMER_INTERVAL: f32 = 1.0 / screen::FRAME_RATE as f32;
    const OPCODE_PER_TICK: u32 = vm::CPU_TICK_RATE / screen::FRAME_RATE;

    let mut vm = vm::init(vm::EmulationMode::Chip8);

    let buzz = sound::generate_beep().await;
    let mut is_buzzing = false;

    let args: Vec<String> = std::env::args().collect();
    let rom_name = if args.len() > 1 {
        &args[1]
    } else {
        "corax+"
    };
    let rom_data = ROMS
        .get_file(format!("{}.ch8", rom_name))
        .map(|file| file.contents())
        .unwrap(); // TODO: How to properly handle this and check file existance?
    vm::load_rom(rom_data, &mut vm).unwrap();

    let mut accumulator = 0.0;
    loop {
        let delta = get_frame_time();
        accumulator += delta;

        while accumulator >= TIMER_INTERVAL {
            for _ in 0..OPCODE_PER_TICK {
                fetch_decode_execute(&mut vm);
            }

            vm::update_timer(&mut vm);
            accumulator -= TIMER_INTERVAL;
        }

        if vm.sound_timer_register > 0 {
            if !is_buzzing {
                is_buzzing = true;
                play_sound(
                    &buzz,
                    PlaySoundParams {
                        looped: true,
                        volume: 1.0,
                    },
                );
            }
        } else {
            is_buzzing = false;
            stop_sound(&buzz);
        }

        screen::render(&mut vm);
        next_frame().await
    }
}
