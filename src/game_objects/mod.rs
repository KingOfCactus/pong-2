mod ball;
mod paddle;

use raylib::prelude::*;
use crate::input_system::*;

// Common trait for GameObjects
pub trait GameObject {
    fn update(&mut self, rl: &RaylibHandle);
}


pub struct Ball {
    pub position: Vector2,
    pub velocity: Vector2,
    pub color: Color,

    pub radius: f32,
    pub speed: f32,

    pub input : InputData,
    pub prone_dir : Vector2,
}

pub struct Paddle {
    pub position: Vector2,
    pub velocity: f32,
    pub color: Color,
    
    pub player_pos: Vector2,
    pub speed: f32,
    pub view_range: f32,
    pub size: Vector2,

    pub hitbox: Rectangle,
}