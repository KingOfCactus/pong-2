use rand::*;
use raylib::prelude::*;
use raylib::ffi::KeyboardKey::*;

use crate::utils::*;
use crate::game_states::*;
use crate::game_objects::*;

impl GameState for GameLoop {
    fn is_active(&self) -> bool {
        return self.is_active;
    }

    fn get_next_state(&self) -> Box<dyn GameState> {
        return Box::new(MainMenu::new());
    }
    
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
        draw_handle.draw_text(&text, centralized_x as i32, (SCREEN_SIZE.y * 0.01) as i32, 22, self.score_color);
        
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
            checkpoint: 0,
            hiscore: get_highscore(),
            score_color: Color::DARKGREEN,
            
            is_active: true,
            debug_mode: false,
            bounced_vertically: false,

            player: Ball::new(
                Vector2::new(SCREEN_SIZE.x * 0.9, SCREEN_SIZE.y * 0.5), [
                    Color::new(188, 212, 230, 150), // 1 live - #BCD4E6
                    Color::new(137, 207, 240, 150), // 2 lives - #89CFF0
                    Color::new(10, 255, 255, 150)   // 3 lives - #6CB4EE
                ], 10.8, MAX_PLAYER_SPEED * 0.63,
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
            ),
        }
    }

    fn update_difficulty(self: &mut Self) {
        match self.score {
            0 => {
                self.checkpoint = 0;
                self.score_color = Color::DARKGREEN;
                self.player.speed = MAX_PLAYER_SPEED * 0.63;

                self.left_paddle.speed = INITIAL_PADDLE_SPEED;
                self.right_paddle.speed = INITIAL_PADDLE_SPEED;
                self.left_paddle.view_range = INITIAL_PADDLE_RANGE;
                self.right_paddle.view_range = INITIAL_PADDLE_RANGE;
            },

            10 => {
                self.checkpoint = 10;
                self.score_color = Color::GREEN;
                self.player.speed = MAX_PLAYER_SPEED * 0.75;

                self.left_paddle.speed = INITIAL_PADDLE_SPEED * 0.85;
                self.right_paddle.speed = INITIAL_PADDLE_SPEED * 0.85;
            },

            25 => {
                self.checkpoint = 25;
                self.score_color = Color::YELLOW;
                self.player.speed = MAX_PLAYER_SPEED * 0.85;

                self.left_paddle.speed = INITIAL_PADDLE_SPEED * 0.74;
                self.right_paddle.speed = INITIAL_PADDLE_SPEED * 0.74;
                self.left_paddle.view_range = INITIAL_PADDLE_RANGE * 0.9;
                self.right_paddle.view_range = INITIAL_PADDLE_RANGE * 0.9;
            },

            50 => {
                self.checkpoint = 50;
                self.score_color = Color::GOLD;
                self.player.speed = MAX_PLAYER_SPEED * 0.95;

                self.left_paddle.speed = INITIAL_PADDLE_SPEED * 0.7;
                self.right_paddle.speed = INITIAL_PADDLE_SPEED * 0.7;
                self.left_paddle.view_range = INITIAL_PADDLE_RANGE * 0.75;
                self.right_paddle.view_range = INITIAL_PADDLE_RANGE * 0.75;
            }

            75 => {
                self.checkpoint = 75;
                self.score_color = Color::RED;
                self.player.speed = MAX_PLAYER_SPEED;
                self.left_paddle.speed = INITIAL_PADDLE_SPEED * 0.55;
                self.right_paddle.speed = INITIAL_PADDLE_SPEED * 0.55;
                self.left_paddle.view_range = INITIAL_PADDLE_RANGE * 0.7;
                self.right_paddle.view_range = INITIAL_PADDLE_RANGE * 0.7;
            } 
            _=> {},
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
            
            // Reset checkpoint if lose all lives 
            if self.player.lives <= 1 {
                self.player.radius += 1.6; 
                self.player.lives = 3;
                self.checkpoint = 0;
            }
            else { 
                self.player.lives -= 1; 
                self.player.radius -= 0.8;
            }
            
            self.score = self.checkpoint;
            self.update_difficulty();
        }

        // Bounce when hit a paddle
        let hit_paddle = self.left_paddle.hitbox.check_collision_circle_rec(self.player.position, self.player.radius + 5.0) ||
                         self.right_paddle.hitbox.check_collision_circle_rec(self.player.position, self.player.radius + 5.0);
        
        if hit_paddle {
            let mut new_angle: f32 = thread_rng().gen_range(0.65..1.0);

            // Copy player.input direction or keep previous direction 
            if self.bounced_vertically || self.player.input.raw_dir.y == 0.0 { 
                new_angle *= self.player.prone_dir.y.signum();
            }
            else { 
                new_angle *= self.player.input.raw_dir.y.signum(); 
            }
            
            // Randomly invert new angle direction when score is above 100
            if self.score >= 100 && thread_rng().gen_range(0.0..1.0) > 0.65 { new_angle *= -1.0; }

            // Keep player out of the paddles
            let min = self.left_paddle.position.x + PADDLE_SIZE.x + self.player.radius;
            let max = self.right_paddle.position.x - PADDLE_SIZE.x - self.player.radius;
            self.player.position = self.player.position.clamp(min, max);

            // Set new direction
            self.bounced_vertically = false;
            self.player.prone_dir.x *= -1.0;
            self.player.prone_dir.y = new_angle;
            self.player.input.dir = Vector2 { x: 0.0, y: 0.0 };
          
            self.score += 1;
            self.update_difficulty();
        }

        // Bounce when hit top or bottom screen
        if self.player.position.y == self.player.radius || self.player.position.y == SCREEN_SIZE.y - self.player.radius {
            let mut new_angle = self.player.prone_dir.y.abs();
            new_angle = new_angle.clamp(0.6, 1.0);

            new_angle *= -self.player.prone_dir.y.signum();
            self.player.prone_dir.y = new_angle;
            self.bounced_vertically = true;

            self.player.input.dir.x *= 0.5;
            self.player.input.dir.y = 0.0;
        }

    }
}