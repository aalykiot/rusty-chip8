mod chip8;
mod display;

use piston_window::*;

use chip8::Chip8;
use display::{Display, DISPLAY_HEIGHT, DISPLAY_WIDTH};

fn from_key_code(key: Key) -> Option<usize> {
    match key {
        Key::D1 => Some(0x0),
        Key::D2 => Some(0x1),
        Key::D3 => Some(0x2),
        Key::D4 => Some(0x3),
        Key::W => Some(0x5),
        Key::Q => Some(0x4),
        Key::E => Some(0x6),
        Key::R => Some(0x7),
        Key::A => Some(0x8),
        Key::S => Some(0x9),
        Key::D => Some(0xA),
        Key::F => Some(0xB),
        Key::Z => Some(0xC),
        Key::X => Some(0xD),
        Key::C => Some(0xE),
        Key::V => Some(0xF),
        _ => None,
    }
}

fn draw_screen(display: &mut Display, window: &mut PistonWindow, event: &Event) {
    // skip if there are no display changes (performance check)
    if !display.should_update() {
        return;
    }
    window.draw_2d(event, |ctx, graphics, _| {
        // default screen to black
        clear(color::BLACK, graphics);
        // iterate through display buffer for active pixels
        for x in 0..DISPLAY_WIDTH {
            for y in 0..DISPLAY_HEIGHT {
                let index = x + DISPLAY_WIDTH * y;
                if display.buffer[index] {
                    rectangle(
                        color::WHITE,
                        [x as f64 * 10.0, y as f64 * 10.0, 10.0, 10.0],
                        ctx.transform,
                        graphics,
                    );
                }
            }
        }
    });
}

fn main() {
    // create piston window instance
    let mut window: PistonWindow = WindowSettings::new("Chip 8 - Emulator", (640, 320))
        .exit_on_esc(true)
        .build()
        .unwrap();

    // create chip8 emulator and load ROM to memory
    let mut chip8 = Chip8::new();
    chip8.load("./programs/ibm-logo.ch8");

    while let Some(e) = window.next() {
        // event for render updates
        if let Some(_) = e.render_args() {
            draw_screen(&mut chip8.display, &mut window, &e);
        }
        // event for game state update
        if let Some(u) = e.update_args() {
            chip8.cycle(u.dt);
        }
        // event for key press
        if let Some(Button::Keyboard(keycode)) = e.press_args() {
            if let Some(key) = from_key_code(keycode) {
                chip8.handle_key_down(key);
            }
        }
        // event for key release
        if let Some(Button::Keyboard(keycode)) = e.release_args() {
            if let Some(key) = from_key_code(keycode) {
                chip8.handle_key_up(key);
            }
        }
    }
}
