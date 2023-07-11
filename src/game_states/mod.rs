use rand::*;
use raylib::prelude::*;
use raylib::ffi::KeyboardKey::*;

use crate::utils::*;
use crate::game_objects::*;

pub trait GameState {
    fn update(&mut self, rl: &RaylibHandle);
    fn draw(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread); // Needs to be the last called method, as it drops the RaylibHandle
}

pub struct GameLoop {
    score: i32,
    hiscore: i32,
    debug_mode: bool,

    player: Ball,
    left_paddle: Paddle,
    right_paddle: Paddle
}

impl GameState for GameLoop {
    fn update(&mut self, rl: &RaylibHandle){
        // Toggle debug mode
        if rl.is_key_pressed(KEY_TAB) { 
            self.debug_mode = !self.debug_mode; 
        }

        // Update player and it references
        self.player.update(&rl);
        self.left_paddle.player_pos = self.player.position;
        self.right_paddle.player_pos = self.player.position;
        
        // Update paddles
        self.left_paddle.update(&rl);
        self.right_paddle.update(&rl);

        self.check_collisions();
    }


    fn draw(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread){
        // Clear screen
        let mut draw_handle = rl.begin_drawing(thread);
        draw_handle.clear_background(Color::BLACK);

        // Draw score text
        let text = format!("Hiscore: {}\n Score: {}", self.hiscore, self.score);
        let centralized_x = SCREEN_SIZE.x / 2.0 - (measure_text(&text, 22) as f32 / 2.0);
        draw_handle.draw_text(&text, centralized_x as i32, (SCREEN_SIZE.y * 0.01) as i32, 22, Color::RED);
        
        // Draw game objects
        draw_handle.draw_circle_v(self.player.position, self.player.radius, self.player.color);
        draw_handle.draw_rectangle_rec(&self.left_paddle.hitbox, Color::GRAY);
        draw_handle.draw_rectangle_rec(&self.right_paddle.hitbox, Color::GRAY);

        // Draw debug info
        if self.debug_mode {
            let stats = self.get_debug_info();
            draw_handle.draw_fps(0, 0);
            draw_handle.draw_text(&stats, 0, (SCREEN_SIZE.y * 0.05) as i32, 18, Color::GREEN);
        }
    }
}

impl GameLoop {
    pub fn new() -> GameLoop {
        return GameLoop {
            score: 0, 
            debug_mode: true,  
            hiscore: get_highscore(), 

            player: Ball::new(
                Vector2 { x: SCREEN_SIZE.x * 0.9, y: SCREEN_SIZE.y * 0.5 },
                Color { r: 255, g: 255, b: 255, a: 185},
                10.0,
                500.0,
            ),

            left_paddle: Paddle::new(
                Vector2 { 
                    x: PADDLE_PADDING, 
                    y: SCREEN_SIZE.y / 2.0 - PADDLE_SIZE.y / 2.0 
                },
                Color::WHITE, PADDLE_SIZE, 
                INITIAL_PADDLE_SPEED,
                INITIAL_PADDLE_RANGE
            ), 

            right_paddle: Paddle::new(
                Vector2 { 
                    x: SCREEN_SIZE.x - PADDLE_SIZE.x - PADDLE_PADDING, 
                    y: SCREEN_SIZE.y / 2.0 - PADDLE_SIZE.y / 2.0 
                },
                Color::WHITE, PADDLE_SIZE, 
                INITIAL_PADDLE_SPEED,
                INITIAL_PADDLE_RANGE
            ) 
        }
    }

    fn get_debug_info(self: &Self) -> String {
        let input = &self.player.input;
        let mut stats = format!("- Prone: ({:.2}, {:.2}) \n- Move: ({:.2}, {:.2}\n", 
                        self.player.prone_dir.x, self.player.prone_dir.y, 
                        input.dir.x, input.dir.y);
    
        if self.player.input.on_gamepad {
            stats += &format!(" {} \n ({:.2}, {:.2})", input.gamepad_name, 
                     input.raw_dir.x, input.raw_dir.y);
        }
        
        return stats;
    }

    fn check_collisions(self: &mut Self) {
         // Restart if player is outside of the screen
         if self.player.position.x > SCREEN_SIZE.x || self.player.position.x < 0.0 {

            // Reset variables
            self.player.position = SCREEN_SIZE / 2.0;
            self.left_paddle.position.y =  SCREEN_SIZE.y / 2.0 - PADDLE_SIZE.y / 2.0;
            self.right_paddle.position.y =  SCREEN_SIZE.y / 2.0 - PADDLE_SIZE.y / 2.0;

            self.player.input.dir = Vector2::zero();
            self.player.prone_dir = Vector2 { x: -1.0, y: 0.0 };

            // Check for a new highscore
            if self.score > self.hiscore { 
                save_highscore(self.score);
                self.hiscore = self.score;
            }
            else { self.score = 0 }

        }

        // Bounce when hit a paddle
        let hit_paddle = self.left_paddle.hitbox.check_collision_circle_rec(self.player.position, self.player.radius + 5.0) ||
                         self.right_paddle.hitbox.check_collision_circle_rec(self.player.position, self.player.radius + 5.0);
        
        if hit_paddle {
            let mut new_angle: f32 = thread_rng().gen();

            // Copy player.input direction or keep previous direction 
            if self.player.input.raw_dir.y == 0.0 { new_angle *= self.player.prone_dir.y.signum(); }
            else { new_angle *= self.player.input.raw_dir.y.signum(); }
            
            // Keep player out of the paddles
            let min = self.left_paddle.position.x + PADDLE_SIZE.x + self.player.radius;
            let max = self.right_paddle.position.x - PADDLE_SIZE.x - self.player.radius;
            self.player.position = self.player.position.clamp(min, max);

            // Set new direction
            self.player.prone_dir.x *= -1.0;
            self.player.prone_dir.y = new_angle;
            self.player.input.dir = Vector2 { x: 0.0, y: 0.0 };
          
            self.score += 1;
        }

        // Bounce when hit top or bottom screen
        if self.player.position.y == self.player.radius || self.player.position.y == SCREEN_SIZE.y - self.player.radius {
            let mut new_angle = self.player.prone_dir.y.abs();
            new_angle = new_angle.clamp(0.6, 1.0);

            new_angle *= -self.player.prone_dir.y.signum();
            self.player.prone_dir.y = new_angle;
            
            self.player.input.dir.x *= 0.5;
            self.player.input.dir.y = 0.0;
        }

    }
}