mod chip8;
mod display;

use piston_window::*;
use std::fs::File;
use std::{env, io::Read};

use chip8::Chip8;
use display::{Display, DISPLAY_HEIGHT, DISPLAY_WIDTH};

fn from_key_code(key: Key) -> Option<usize> {
    match key {
        Key::D1 => Some(0x1),
        Key::D2 => Some(0x2),
        Key::D3 => Some(0x3),
        Key::D4 => Some(0xC),
        Key::Q => Some(0x4),
        Key::W => Some(0x5),
        Key::E => Some(0x6),
        Key::R => Some(0xD),
        Key::A => Some(0x7),
        Key::S => Some(0x8),
        Key::D => Some(0x9),
        Key::F => Some(0xE),
        Key::Z => Some(0xA),
        Key::X => Some(0x0),
        Key::C => Some(0xB),
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
                if display.buffer[index] == 1 {
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
    // open file containing the chip8 ROM
    let path = env::args()
        .nth(1)
        .expect("you must provide a valid ROM for the emulator");

    let mut file = File::open(path).expect("an error occurred opening the file");

    // get data into a u8 array
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();

    // create piston window instance
    let mut window: PistonWindow = WindowSettings::new("Chip 8 - Emulator", (640, 320))
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut chip8 = Chip8::new(data);
    let mut chip8_time = 0.0;

    while let Some(e) = window.next() {
        // draw to screen updates
        if let Some(_) = e.render_args() {
            draw_screen(&mut chip8.display, &mut window, &e);
        }

        // game state updates
        if let Some(u) = e.update_args() {
            // run next cycle and keep track of delta time
            chip8.cycle(u.dt);
            chip8_time += u.dt;
            // // at 60 Hz decrement chip8's timers
            if chip8_time > 1.0 / 60.0 {
                chip8.decrement_timers();
                chip8_time -= 1.0 / 60.0;
            }
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
