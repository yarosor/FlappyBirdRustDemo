use ggez::graphics::{self, Image, Rect, DrawParam};
use ggez::{Context, GameResult};
use rand::Rng;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

const OBSTACLE_WIDTH: f32 = 60.0;
const OBSTACLE_SPEED: f32 = 3.0;
const GAP_HEIGHT: f32 = 150.0;

pub struct Obstacle {
    pub x: f32,
    pub gap_y: f32,
    pub passed: bool,
}

impl Obstacle {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        let min_gap = 50.0;
        let max_gap = SCREEN_HEIGHT - GAP_HEIGHT - 50.0;
        let gap_y = rng.gen_range(min_gap..max_gap);

        Self {
            x: SCREEN_WIDTH,
            gap_y,
            passed: false,
        }
    }

    pub fn update(&mut self) {
        self.x -= OBSTACLE_SPEED;
    }

    //draw
    pub fn draw(
        &self,
        _ctx: &mut Context,
        canvas: &mut graphics::Canvas,
        top_img: &Image,
        bot_img: &Image,
    ) -> GameResult {
        //up tube
        let scale_top_x = OBSTACLE_WIDTH / top_img.width() as f32;
        let scale_top_y = self.gap_y / top_img.height() as f32;

        let params_top = DrawParam::new()
            .dest([self.x, 0.0])
            .scale([scale_top_x, scale_top_y]);

        canvas.draw(top_img, params_top);

        //down tube
        let bottom_y = self.gap_y + GAP_HEIGHT;
        let bottom_height = SCREEN_HEIGHT - bottom_y;

        let scale_bot_x = OBSTACLE_WIDTH / bot_img.width() as f32;
        let scale_bot_y = bottom_height / bot_img.height() as f32;

        let params_bot = DrawParam::new()
            .dest([self.x, bottom_y])
            .scale([scale_bot_x, scale_bot_y]);

        canvas.draw(bot_img, params_bot);

        Ok(())
    }

    pub fn check_collision(&self, bird_rect: &Rect) -> bool {
        let top_rect = Rect::new(self.x, 0.0, OBSTACLE_WIDTH, self.gap_y);
        let bottom_rect = Rect::new(
            self.x,
            self.gap_y + GAP_HEIGHT,
            OBSTACLE_WIDTH,
            SCREEN_HEIGHT - (self.gap_y + GAP_HEIGHT),
        );

        top_rect.overlaps(bird_rect) || bottom_rect.overlaps(bird_rect)
    }

    pub fn is_off_screen(&self) -> bool {
        self.x < -OBSTACLE_WIDTH
    }
}