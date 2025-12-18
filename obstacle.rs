use ggez::graphics::{self, Color, DrawMode, Mesh, Rect, DrawParam};
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

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        // Верхняя труба
        let top_rect = Rect::new(self.x, 0.0, OBSTACLE_WIDTH, self.gap_y);

       
        let bottom_rect = Rect::new(
            self.x,
            self.gap_y + GAP_HEIGHT,
            OBSTACLE_WIDTH,
            SCREEN_HEIGHT - (self.gap_y + GAP_HEIGHT),
        );

        
        let mesh_top = Mesh::new_rectangle(ctx, DrawMode::fill(), top_rect, Color::GREEN)?;
        let mesh_bottom = Mesh::new_rectangle(ctx, DrawMode::fill(), bottom_rect, Color::GREEN)?;

        canvas.draw(&mesh_top, DrawParam::default());
        canvas.draw(&mesh_bottom, DrawParam::default());

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