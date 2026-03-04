use raylib::prelude::*;

use crate::vm::VM;

static SCALE: i32 = 15;
static WIDTH: i32 = 64 * SCALE;
static HEIGHT: i32 = 32 * SCALE;
static BACKGROUND_COLOR : Color = Color::WHITE;

pub struct RaylibContext {
    pub handle: RaylibHandle,
    pub thread: RaylibThread,
}

pub fn init() -> RaylibContext {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Chip R8")
        .build();

    rl.set_target_fps(60);

    RaylibContext { handle: rl, thread }
}

pub fn render(vm: &mut VM, context: &mut RaylibContext) {
    let mut d = context.handle.begin_drawing(&context.thread);

    d.clear_background(BACKGROUND_COLOR);

    // TODO: Render active pixels
}
