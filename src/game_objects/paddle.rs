use raylib::prelude::*;

use crate::utils::*;
use crate::game_objects::*;


impl GameObject for Paddle {
    fn update(&mut self, rl: &RaylibHandle) {
        self.update_velocity();
        self.translate(rl);
    }
}

impl Paddle {
    pub fn new( position: Vector2, color: Color,
                size: Vector2, speed : f32, view_range: f32) -> Paddle {
            return Paddle { 
                view_range,
                position,
                speed,
                color, 
                size,
                
                velocity: 0.0,
                hitbox: Rectangle::new(
                    position.x, position.y, 
                    size.x, size.y
                ),
                player_pos: Vector2::zero(),
            }
    }

    fn update_velocity(&mut self) {
        let closeness = self.get_player_pos_closeness();
        let distance = self.player_pos.y - self.position.y;

        // Don't move if player is out of view range
        if closeness <= 0.0 || distance.abs() < 5.0 { 
            self.velocity = 0.0;    
            return; 
        }

        let new_vel = distance * closeness.powf(2.0) * 60.0;
        self.velocity = new_vel.clamp(-self.speed, self.speed);
    }

    // Rust compiler don't let me name it move() >:(
    fn translate(&mut self, rl: &RaylibHandle) {
        self.position.y += self.velocity * rl.get_frame_time();
        self.hitbox.x = self.position.x;
        self.hitbox.y = self.position.y;
    }

    // How close the player_pos is from the paddle, normalized
    fn get_player_pos_closeness(&self) -> f32 {
        let view_distance = SCREEN_SIZE.x * self.view_range;
        let distance = self.player_pos.x - self.position.x;
        return 1.0 - (distance / view_distance * distance.signum()).clamp(0.0, 1.0);
    }
}