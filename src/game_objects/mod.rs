mod ball;
mod paddle;

use raylib::prelude::*;
use crate::input_system::*;

pub const PADDLE_PADDING: f32 = 20.0;
pub const MAX_PLAYER_SPEED: f32 = 500.0;
pub const INITIAL_PADDLE_RANGE: f32 = 0.5;
pub const INITIAL_PADDLE_SPEED: f32 = 500.0;

pub const PADDLE_SIZE: Vector2 = Vector2 { x: 11.0, y: 65.0 };

// Common trait for GameObjects
pub trait GameObject {
    fn update(&mut self, rl: &RaylibHandle);
}


pub struct Ball {
    pub position: Vector2,
    pub velocity: Vector2,
    pub colors: [Color; 3],
    pub color: Color,

    pub radius: f32,
    pub speed: f32,
    pub lives: i32,

    pub is_active: bool,
    pub input : InputData,
    pub prone_dir : Vector2,
}

pub struct Paddle {
    pub position: Vector2,
    pub velocity: f32,
    pub speed: f32,

    pub color: Color,
    pub size: Vector2,
    pub view_range: f32,
    
    pub hitbox: Rectangle,
    pub player_pos: Vector2,

    pub input: InputData,
    pub is_active: bool,
    pub player_controlled: bool,
}