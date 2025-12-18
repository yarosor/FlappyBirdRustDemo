use crate::SCREEN_HEIGHT;
use ggez::graphics::{self, Image, DrawParam};
use ggez::{Context, GameResult};

const GRAVITY: f32 = 0.25;
const JUMP_STRENGTH: f32 = -5.0;
const UPSTROKE_DURATION: f32 = 0.1;
const DOWNSTROKE_DURATION: f32 = 0.15;


const BIRD_SIZE: f32 = 35.0;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WingState {
    Idle,
    Upstroke,
    Downstroke,
}

pub struct Bird {
    pub x: f32,
    pub y: f32,
    pub velocity: f32,
    pub wing_state: WingState,
    pub anim_timer: f32,
    texture_idle: Image,
    texture_up: Image,
    texture_down: Image,
}

impl Bird {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let texture_idle = Image::from_path(ctx, "/bird_mid.png")?;
        let texture_up = Image::from_path(ctx, "/bird_up.png")?;
        let texture_down = Image::from_path(ctx, "/bird_down.png")?;

        Ok(Self {
            x: 50.0,
            y: SCREEN_HEIGHT / 2.0,
            velocity: 0.0,
            wing_state: WingState::Idle,
            anim_timer: 0.0,
            texture_idle,
            texture_up,
            texture_down,
        })
    }

    pub fn update(&mut self, dt: f32) {
        self.velocity += GRAVITY;
        self.y += self.velocity;


        if self.y < 0.0 {
            self.y = 0.0;
            self.velocity = 0.0;
        }

        if self.y > SCREEN_HEIGHT - BIRD_SIZE {
            self.y = SCREEN_HEIGHT - BIRD_SIZE;
            self.velocity = 0.0;
        }


        match self.wing_state {
            WingState::Upstroke => {
                self.anim_timer += dt;
                if self.anim_timer >= UPSTROKE_DURATION {
                    self.wing_state = WingState::Downstroke;
                    self.anim_timer = 0.0;
                }
            }
            WingState::Downstroke => {
                self.anim_timer += dt;
                if self.anim_timer >= DOWNSTROKE_DURATION {
                    self.wing_state = WingState::Idle;
                    self.anim_timer = 0.0;
                }
            }
            WingState::Idle => {}
        }
    }

    pub fn jump(&mut self) {
        self.velocity = JUMP_STRENGTH;
        self.wing_state = WingState::Upstroke;
        self.anim_timer = 0.0;
    }

    pub fn get_rect(&self) -> ggez::graphics::Rect {
        ggez::graphics::Rect::new(self.x, self.y, BIRD_SIZE, BIRD_SIZE)
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) {
        let current_texture = match self.wing_state {
            WingState::Idle => &self.texture_idle,
            WingState::Upstroke => &self.texture_up,
            WingState::Downstroke => &self.texture_down,
        };

        let scale_factor = BIRD_SIZE / current_texture.width() as f32;

        let draw_params = DrawParam::new()
            .dest([self.x, self.y])
            .scale([scale_factor, scale_factor]);

        canvas.draw(current_texture, draw_params);
    }
}