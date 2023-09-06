use raylib::prelude::*;

use crate::utils::*;
use crate::game_objects::*;

impl GameObject for Ball{
    fn update(&mut self, rl: &RaylibHandle) {
        let input = self.player_data.get_input(&rl);

        self.update_velocity(&input);
        self.update_color(rl, &input);
        self.translate(rl);
    }
}

impl Ball {
    pub fn new( position: Vector2, colors: [Color; 3], radius: f32, speed : f32) -> Self {
            Ball {
                lives: 3,
                is_active: true,
                color: colors[2], 
                velocity: Vector2::zero(), 
                prone_dir: Vector2::zero(),
                position, radius, speed, colors,
                player_data: PlayerData::new(0, Box::new(GamepadInput::new(0, true)), 3.0, true) 
                // player_data: PlayerData::new(0, Box::new(KeyboardInput::new()), 3.0, true)
            }
    }

    // Fluctuates between grey and white
    fn update_color(&mut self, rl: &RaylibHandle, input: &InputData) {
        let mut alpha = self.color.a as f32;
        let input_intensity = (input.dir.x.abs() + input.dir.y.abs()) / 2.0;

        // go closer to white if receiving self.input
        if input.raw_dir != Vector2::zero() { 
            alpha += 680.0 * input_intensity * 10.0 * rl.get_frame_time(); // TODO: Use logarithmic interpolation instead of a linear one
        }
        // go closer to grey if not
        else { 
            alpha -= 500.0 * (1.0 - input_intensity).powf(2.0) * rl.get_frame_time(); 
        }
        
        self.color = self.colors[self.lives as usize - 1];
        self.color.a = alpha.clamp(self.color.a as f32, 255.0) as u8;
    }

    // Rust compiler don't let me name it move() >:(
    fn translate(&mut self, rl: &RaylibHandle)
    {
        self.position += self.velocity * rl.get_frame_time();
        self.position.y = self.position.y.clamp(self.radius, SCREEN_SIZE.y - self.radius); // keep it inside the screen
    }

    fn update_velocity(&mut self, input: &InputData) {
        let desired_dir = input.dir * Vector2 { x: 1.0 / 2.0, y: 1.0 / 1.50}; // TODO: Remove hardcoded vector2 multiplier
        self.velocity = (self.prone_dir + desired_dir) * self.speed;
    }
}