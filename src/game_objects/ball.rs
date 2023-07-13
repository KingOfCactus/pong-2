use raylib::prelude::*;

use crate::utils::*;
use crate::game_objects::*;

impl GameObject for Ball{
    fn update(&mut self, rl: &RaylibHandle) {
        self.update_velocity(rl);
        self.update_color(rl);
        self.translate(rl);
    }
}

impl Ball {
    pub fn new( position: Vector2, color: Color, 
                radius: f32, speed : f32) -> Self {
            Ball {
                input : InputData::new(), 
                velocity: Vector2::zero(), 
                position, radius, speed, color,
                prone_dir: Vector2 { x: -1.0, y: 0.0},
            }
    }

    // Fluctuates between grey and white
    fn update_color(&mut self, rl: &RaylibHandle) {
        let mut alpha = self.color.a as f32;
        let input_intensity = (self.input.dir.x.abs() + self.input.dir.y.abs()) / 2.0;

        // go closer to white if receiving input
        if self.input.raw_dir != Vector2::zero() { 
            alpha += 680.0 * input_intensity * rl.get_frame_time(); // TODO: Use logarithmic interpolation instead of a linear one
        }
        // go closer to grey if not
        else { 
            alpha -= 500.0 * (1.0 - input_intensity).powf(2.0) * rl.get_frame_time(); 
        }

        self.color.a = alpha.clamp(185.0, 255.0) as u8;
    }

    // Rust compiler don't let me name it move() >:(
    fn translate(&mut self, rl: &RaylibHandle)
    {
        self.position += self.velocity * rl.get_frame_time();
        self.position.y = self.position.y.clamp(self.radius, SCREEN_SIZE.y - self.radius); // keep it inside the screen
    }

    fn update_velocity(&mut self, rl: &RaylibHandle) {
        // get newest input data
        InputData::update_data(&mut self.input, rl);
        let desired_dir = self.input.dir * Vector2 { x: 1.0 / 2.25, y: 1.0 / 1.70 }; // TODO: Remove hardcoded vector2 multiplier
        self.velocity = (self.prone_dir + desired_dir) * self.speed;
    }
}