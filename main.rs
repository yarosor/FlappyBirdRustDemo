mod bird;
mod game;
mod obstacle;

use game::GameState;
use ggez::{conf, event, ContextBuilder, GameResult};
use std::path::PathBuf;

pub const SCREEN_WIDTH: f32 =  400.0;
pub const SCREEN_HEIGHT: f32 = 400.0;

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        PathBuf::from("./resources")
    };

    let (mut ctx, event_loop) = ContextBuilder::new("flappy_bird", "Gemini")
        .window_setup(conf::WindowSetup::default().title("Flappy Bird"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .add_resource_path(resource_dir)
        .build()?;

    let game = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, game)
    
}