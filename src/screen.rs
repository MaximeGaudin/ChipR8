use macroquad::prelude::*;

use crate::vm::{self, VM};

pub static SCALE: f32 = 15.0;

static BACKGROUND_COLOR: Color = WHITE;
static PIXEL_COLOR: Color = BLACK;

pub static FRAME_RATE: u32 = 60;

pub fn render(vm: &mut VM) {
    clear_background(BACKGROUND_COLOR);

    for y in 0..vm::SCREEN_HEIGHT {
        for x in 0..vm::SCREEN_WIDTH {
            let screen_index = x + (y * vm::SCREEN_WIDTH);

            if vm.screen[screen_index] == 1 {
                draw_rectangle(
                    (x as f32) * SCALE,
                    (y as f32) * SCALE,
                    SCALE as f32,
                    SCALE as f32,
                    PIXEL_COLOR,
                );
            }
        }
    }

    draw_text(
        &format!("FPS: {}", get_fps()),
        (64.0 * SCALE) - 90.0,
        20.0,
        20.0,
        GREEN,
    );
}
