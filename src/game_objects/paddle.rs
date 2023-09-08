use raylib::prelude::*;

use crate::utils::*;
use crate::game_objects::*;


impl GameObject for Paddle {
    fn update(&mut self, rl: &RaylibHandle, input: &InputData) {        
        self.update_velocity(input);
        self.update_color(&rl);
        self.translate(rl);
    }
}

impl Paddle {
    pub fn new( position: Vector2, colors: [Color; 2], size: Vector2,  
                speed : f32, view_range: f32, player_controlled: bool) -> Paddle {
                    return Paddle { 
                        is_active: false, player_controlled, 
                        colors, view_range, position, speed, size, color: colors[0],
                        hitbox: Rectangle::new(position.x, position.y, size.x, size.y),
                        velocity: 0.0, player_pos: Vector2::zero(), 
                    }
    }

    fn update_color(&mut self, rl: &RaylibHandle) {
        if !self.player_controlled { return; }
        let closeness = self.get_player_pos_closeness();

        let mut alpha = self.color.a as f32;
        let mut input_intensity = 1.0;
        if !self.is_active { input_intensity = 0.0; }
        // let input_intensity = (input.dir.x.abs() + input.dir.y.abs()) / 2.0;

        // go to white if active
        if self.is_active{
            let step = closeness.powf(3.0) * ((self.color.a as f32 - 130.0) / (255.0 - 130.0));
            self.color.g = lerp(self.colors[0].g as f32, self.colors[1].g as f32, step) as u8;
            self.color.b = lerp(self.colors[0].b as f32, self.colors[1].b as f32, step) as u8;
            alpha += 50.0 * input_intensity * 10.0 * rl.get_frame_time();                      // TODO: Use logarithmic interpolation instead of linear
        }
        // go closer to grey if not
        else {
            let step = (self.color.a as f32 - 130.0) / (255.0 - 130.0);
            self.color.g = lerp(self.colors[0].g as f32, self.colors[1].g as f32, step) as u8;
            self.color.b = lerp(self.colors[0].b as f32, self.colors[1].b as f32, step) as u8; 
            alpha -= 500.0 * rl.get_frame_time();
        }
        
        // self.color = self.colors[self.lives as usize - 1];
        self.color.a = alpha.clamp(130.0 as f32, 255.0) as u8;
    }

    fn update_velocity(&mut self, input: &InputData) {
        let closeness = self.get_player_pos_closeness();
        let distance = self.player_pos.y - self.position.y;
        
        if self.player_controlled {
            if !self.is_active {
                self.velocity = 0.0;
                return;
            }

            self.velocity = (input.dir.y * self.speed * 0.6) + (input.raw_dir.y * closeness.powf(3.0) * self.speed * 0.7);
            return;
        }

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
        self.position.y = self.position.y.clamp(0.0, SCREEN_SIZE.y);

        self.hitbox.x = self.position.x;
        self.hitbox.y = self.position.y;
    }

    // How close the player_pos is from the paddle, normalized
    fn get_player_pos_closeness(&self) -> f32 {
        let view_distance = SCREEN_SIZE.x * self.view_range;
        let distance = self.player_pos.x - (self.position.x);
        return 1.0 - (distance / view_distance * distance.signum()).clamp(0.0, 1.0);
    }
}