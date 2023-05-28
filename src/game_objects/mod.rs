use crate::utils::*;
use crate::input_system::*;
use raylib::prelude::*;

// Common trait for GameObjects
pub trait GameObject {
    fn update(&mut self, rl: &mut RaylibHandle);
    fn draw(&mut self);
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

impl GameObject for Paddle {
    fn update(&mut self, rl: &mut RaylibHandle) {
        self.update_velocity();
        self.translate(rl);
    }

    fn draw(&mut self) {
        return;
    }
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

impl GameObject for Ball{
    fn update(&mut self, rl: &mut RaylibHandle) {
        self.update_velocity(rl);
        self.update_color(rl);
        self.translate(rl);
    }

    fn draw(&mut self) {
        return;
    }
}


impl Paddle {
    pub fn new( position: Vector2, color: Color,
                size: Vector2, speed : f32, view_range: f32) -> Self {
            Paddle { 
                view_range,
                position,
                speed,
                color, 
                size,

                velocity: 0.0,
                player_pos: Vector2::zero(),
                hitbox: Rectangle::new(
                    position.x, 
                    position.y, 
                    size.x, 
                    size.y
                )
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


impl Ball {
    pub fn new( position: Vector2, color: Color, 
                radius: f32, speed : f32) -> Self {
            Ball { 
                position, radius, speed, color,
                input : InputData::new(), 
                velocity: Vector2::zero(),
                prone_dir: Vector2::zero(), 
            }
    }

    // Fluctuates between grey and white
    fn update_color(&mut self, rl: &mut RaylibHandle) {
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
    fn translate(&mut self, rl: &mut RaylibHandle)
    {
        self.position += self.velocity * rl.get_frame_time();
        self.position.y = self.position.y.clamp(self.radius, SCREEN_SIZE.y - self.radius); // keep it inside the screen
    }

    fn update_velocity(&mut self, rl: &mut RaylibHandle) {
        // get newest input data
        InputData::update_data(&mut self.input, rl);
        let desired_dir = self.input.dir * Vector2 { x: 1.0 / 2.25, y: 1.0 / 1.70 }; // TODO: Remove hardcoded vector2 multiplier
        self.velocity = (self.prone_dir + desired_dir) * self.speed;
    }
}