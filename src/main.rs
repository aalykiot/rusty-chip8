mod chip8;
mod display;

use chip8::Chip8;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};

struct GameState {
    chip8: Chip8,
}

impl GameState {
    fn new() -> GameState {
        let mut chip8 = Chip8::new();
        chip8.load("./programs/ibm-logo.ch8");

        GameState { chip8 }
    }
}

impl event::EventHandler for GameState {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // skip if there are no display changes (performance check)
        if !self.chip8.display.should_update() {
            return Ok(());
        }

        graphics::clear(ctx, graphics::BLACK);
        self.chip8.display.buffer_update = false;

        for x in 0..display::DISPLAY_WIDTH {
            for y in 0..display::DISPLAY_HEIGHT {
                let index = x + display::DISPLAY_WIDTH * y;
                let color = match self.chip8.display.buffer[index] {
                    true => graphics::WHITE,
                    false => graphics::BLACK,
                };

                let tile = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new_i32(x as i32 * 10, y as i32 * 10, 10, 10),
                    color,
                )?;

                graphics::draw(ctx, &tile, graphics::DrawParam::default())?;
            }
        }
        graphics::present(ctx)
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const TARGET_FPS: u32 = 24;
        while timer::check_update_time(ctx, TARGET_FPS) {
            self.chip8.cycle();
        }
        Ok(())
    }
}

fn main() -> GameResult {
    let (context, event_loop) = &mut ContextBuilder::new("chip8_emulator", "alexalikiotis")
        .window_setup(conf::WindowSetup::default().title("Chip 8 - Emulator"))
        .window_mode(
            conf::WindowMode::default()
                .dimensions(640_f32, 320_f32)
                .resizable(false),
        )
        .build()?;

    let mut chip8 = Chip8::new();
    chip8.load("./programs/ibm-logo.ch8");

    let state = &mut GameState::new();
    event::run(context, event_loop, state)
}
