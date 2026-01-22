use crate::bird::Bird;
use crate::obstacle::Obstacle;
use ggez::graphics::{self, Image, DrawParam};
use ggez::{event, Context, GameResult};
use ggez::input::keyboard::{KeyCode, KeyInput};

const OBSTACLE_SPAWN_TIMER: f32 = 1.8;

pub struct GameState {
    pub bird: Bird,
    pub obstacles: Vec<Obstacle>,
    pub game_over: bool,
    pub score: u32,
    pub spawn_timer: f32,
    // texture
    pub background: Image,
    pub tube_top: Image,
    pub tube_bottom: Image,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let bird = Bird::new(ctx)?;

        // my texture
        let background = Image::from_path(ctx, "/background.\
        ")?;
        let tube_top = Image::from_path(ctx, "/tubeup.png")?;
        let tube_bottom = Image::from_path(ctx, "/tubedown.png")?;

        Ok(Self {
            bird,
            obstacles: Vec::new(),
            game_over: false,
            score: 0,
            spawn_timer: 0.0,
            background,
            tube_top,
            tube_bottom,
        })
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.game_over {
            return Ok(());
        }

        let dt = ctx.time.delta().as_secs_f32();
        self.bird.update(dt);

        self.spawn_timer += dt;
        if self.spawn_timer >= OBSTACLE_SPAWN_TIMER {
            self.obstacles.push(Obstacle::new());
            self.spawn_timer = 0.0;
        }

        let bird_rect = self.bird.get_rect();

        for obstacle in &mut self.obstacles {
            obstacle.update();

            if obstacle.check_collision(&bird_rect) {
                self.game_over = true;
                println!("Game Over! Score: {}", self.score);
            }

            if !obstacle.passed && obstacle.x + 60.0 < self.bird.x {
                self.score += 1;
                obstacle.passed = true;
                println!("Score: {}", self.score);
            }
        }

        self.obstacles.retain(|o| !o.is_off_screen());

        if self.bird.y >= crate::SCREEN_HEIGHT - 35.0 {
            self.game_over = true;
            println!("Game Over (Hit Ground)! Score: {}", self.score);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        // background
        let scale_x = crate::SCREEN_WIDTH / self.background.width() as f32;
        let scale_y = crate::SCREEN_HEIGHT / self.background.height() as f32;
        let bg_params = DrawParam::new().scale([scale_x, scale_y]);
        canvas.draw(&self.background, bg_params);

                for obstacle in &self.obstacles {
            obstacle.draw(ctx, &mut canvas, &self.tube_top, &self.tube_bottom)?;
        }

        self.bird.draw(&mut canvas);

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> GameResult {
        if let Some(keycode) = input.keycode {
            if keycode == KeyCode::Space {
                if self.game_over {
                    *self = GameState::new(ctx)?;
                } else {
                    self.bird.jump();
                }
            }
        }
        Ok(())
    }
}