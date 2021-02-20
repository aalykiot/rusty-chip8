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
        graphics::clear(ctx, graphics::BLACK);
        graphics::present(ctx)
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // const TARGET_FPS: u32 = 60;
        // while timer::check_update_time(ctx, TARGET_FPS) {
        //     self.chip8.cycle();
        // }
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
