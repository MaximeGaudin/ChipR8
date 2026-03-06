use raylib::prelude::*;

use crate::vm::{self, SCREEN_WIDTH, VM};

static SCALE: i32 = 15;
static WIDTH: i32 = 64 * SCALE;
static HEIGHT: i32 = 32 * SCALE;

static BACKGROUND_COLOR: Color = Color::WHITE;
static PIXEL_COLOR: Color = Color::BLACK;

pub struct RaylibContext {
    pub handle: RaylibHandle,
    pub thread: RaylibThread,
}

pub fn init() -> RaylibContext {
    let (mut rl, thread) = raylib::init().size(WIDTH, HEIGHT).title("Chip R8").build();

    rl.set_target_fps(60);

    RaylibContext { handle: rl, thread }
}

pub fn render(vm: &mut VM, context: &mut RaylibContext) {
    let mut d = context.handle.begin_drawing(&context.thread);

    d.clear_background(BACKGROUND_COLOR);

    for y in 0..vm::SCREEN_HEIGHT {
        for x in 0..vm::SCREEN_WIDTH {
            let screen_index = x + (y * vm::SCREEN_WIDTH);

            if vm.screen[screen_index] == 1 {
                d.draw_rectangle(
                    (x as i32) * SCALE,
                    (y as i32) * SCALE,
                    SCALE,
                    SCALE,
                    PIXEL_COLOR,
                );
            }
        }
    }

    d.draw_fps((SCREEN_WIDTH as i32) * SCALE - 90, 10);
}
