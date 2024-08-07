use rand::*;
use raylib::prelude::*;
use raylib::ffi::KeyboardKey::*;

use crate::input_system::*;
use crate::utils::*;
use crate::game_scenes::*;
use crate::game_objects::*;

impl GameScene for GameLoop {
    fn update(self: &mut Self, rl: &RaylibHandle){
        let ball_input = self.players_input[0].get_data(rl);
        let paddle_input = self.players_input[1].get_data(rl);

        // Toggle debug mode
        if rl.is_key_pressed(KEY_TAB) { 
            self.debug_mode = !self.debug_mode; 
        }

        // Respawn ball if outside of the screen
        self.ball.is_active = self.ball.position.x > 0.0 && self.ball.position.x <= SCREEN_SIZE.x;
        if !self.ball.is_active {
            self.respawn_player(&rl);
            return;
        }

        // After respawn, wait for input to apply prone_dir
        if self.ball.prone_dir == Vector2::zero() && ball_input.dir != Vector2::zero() {
            self.ball.prone_dir = Vector2::new(-1.0, 0.0);
        }

        // Update ball and it references
        self.ball.update(&rl, &ball_input);
        self.left_paddle.player_pos = self.ball.position;
        self.right_paddle.player_pos = self.ball.position;
        
        // Update paddles
        self.left_paddle.update(&rl, &paddle_input);
        self.right_paddle.update(&rl, &paddle_input);
        self.check_ball_collisions(&ball_input);
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
        if self.ball.is_active { 
            draw_handle.draw_circle_v(self.ball.position, self.ball.radius, self.ball.color);
        }
        draw_handle.draw_rectangle_rec(&self.left_paddle.hitbox, &self.left_paddle.color);
        draw_handle.draw_rectangle_rec(&self.right_paddle.hitbox, &self.right_paddle.color);

        // Draw debug info
        if self.debug_mode {
            let stats = self.get_debug_info();
            draw_handle.draw_fps(0, 0);
            draw_handle.draw_text(&stats, 0, (SCREEN_SIZE.y * 0.05) as i32, 18, Color::GREEN);
        }
    }

    fn is_active(&self) -> bool { return self.is_active; }
    fn get_next_scene(&self, _rl: &RaylibHandle) -> Box<dyn GameScene> { return Box::new(MainMenu::new()); }
}

impl GameLoop {
    fn check_ball_collisions(self: &mut Self, ball_input: &InputData) {
        let hit_vertical_edge = self.ball.position.y == self.ball.radius || self.ball.position.y == SCREEN_SIZE.y - self.ball.radius;
        let hit_paddle = self.right_paddle.hitbox.check_collision_circle_rec(self.ball.position, self.ball.radius + 5.0) ||
                         self.left_paddle.hitbox.check_collision_circle_rec(self.ball.position, self.ball.radius + 5.0);
        
        if hit_paddle { self.paddle_bounce(ball_input); }
        if hit_vertical_edge { self.edge_bounce(ball_input); }
    }

    // Bounce ball when hit top or bottom screen
    fn edge_bounce(self: &mut Self, ball_input: &InputData) {
        let entry_angle = self.ball.velocity.normalized().y.abs();

        // Calculates out angle exponentially
        let mut new_angle = entry_angle.powf(1.65).clamp(0.4, 0.55);
        new_angle *= -self.ball.velocity.y.signum();

        // Height down player horizontal input
        let new_input = Vector2::new(ball_input.dir.x * 0.5, 0.0);
        self.players_input[0].override_last_dir(new_input);

        // Apply new angle
        self.ball.prone_dir.y = new_angle;
        self.bounced_vertically = true;
    }

    // Bounce ball when hits a paddle
    fn paddle_bounce(self: &mut Self, ball_input: &InputData)
    {
        let close_to_edge = self.ball.position.y >= SCREEN_SIZE.y - self.ball.radius - 73.0 || 
                            self.ball.position.y <= self.ball.radius + 73.0;

        let mut new_angle: f32 = thread_rng().gen_range(0.45..1.0);
        if close_to_edge { new_angle *= 1.5; }
           
        // Decides new angle signum
        if close_to_edge || self.bounced_vertically || ball_input.raw_dir.y == 0.0 { 
            new_angle *= self.ball.prone_dir.y.signum();
        }
        else { new_angle *= ball_input.raw_dir.y.signum(); }

        // Move ball out of the paddles
        let min = self.left_paddle.position.x + PADDLE_SIZE.x + self.ball.radius;
        let max = self.right_paddle.position.x - PADDLE_SIZE.x - self.ball.radius;
        self.ball.position = self.ball.position.clamp(min, max);

        // Update paddles
        self.left_paddle.is_active = !self.left_paddle.is_active;
        self.right_paddle.is_active = !self.right_paddle.is_active;

        // Set new prone_dir
        self.ball.prone_dir.x *= -1.0;
        self.ball.prone_dir.y = new_angle;
        
        self.score += 1;
        self.update_difficulty();

        self.bounced_vertically = false;
        self.players_input[0].override_last_dir(Vector2::zero());
    }
    
    fn respawn_player(self: &mut Self, rl: &RaylibHandle) {
        // Wait for 1 second
        self.respawn_timer += rl.get_frame_time();
        if self.respawn_timer < 1.0 { return; }

        // Reset variables
        self.ball.position = SCREEN_SIZE / 2.0;
        self.left_paddle.position.y =  SCREEN_SIZE.y / 2.0 - PADDLE_SIZE.y / 2.0;
        self.right_paddle.position.y =  SCREEN_SIZE.y / 2.0 - PADDLE_SIZE.y / 2.0;

        self.players_input[0].override_last_dir(Vector2::zero());
        self.ball.prone_dir = Vector2 { x: -1.0, y: 0.0 };
        
        // Check for a new highscore
        if self.score > self.hiscore { 
            MiscUtils::save_highscore(self.score);
            self.hiscore = self.score;
        }
        
        // Reset checkpoint if lose all lives 
        if self.ball.lives <= 1 { 
            self.ball.prone_dir = Vector2::zero();
            self.ball.radius += 1.6; 
            self.ball.lives = 3;
            self.checkpoint = 0;
        }
        else { 
            self.ball.lives -= 1; 
            self.ball.radius -= 0.8;
        }
        
        self.ball.is_active = true;
        self.left_paddle.is_active = true;
        self.right_paddle.is_active = false;

        self.score = self.checkpoint;
        self.respawn_timer = 0.0;
        self.update_difficulty();
    }

    fn get_debug_info(self: &Self) -> String {
        // let mut stats = format!("- Prone: ({:.2}, {:.2}) \n- Move: ({:.2}, {:.2}\n", 
        //                 self.ball.prone_dir.x, self.ball.prone_dir.y, 
        //                 ball_input.dir.x, ball_input.dir.y);
    
        // if ball_input.on_gamepad {
        //    stats += &format!(" {} \n ({:.2}, {:.2})", input.gamepad_name, 
        //             input.raw_dir.x, input.raw_dir.y);
        // }
        
        // return stats;

        todo!("Fix this and made so that every localplayer information in show");
    }
    
    pub fn new(selected_mode: GameMode, selected_devices: (Box<dyn InputDevice>, Box<dyn InputDevice>)) -> GameLoop {
        // Just in case
        if selected_mode == GameMode::None {
            panic!("GameMode wasn't selected. How did you manage to do this?");
        }
        
        return GameLoop {
             score: 0,
             checkpoint: 0,
             respawn_timer: 0.0,
 
             hiscore: MiscUtils::get_highscore(),
             game_mode: selected_mode,
             score_color: Color::DARKGREEN,
 
             is_active: true,
             debug_mode: false,
             bounced_vertically: false,
             
             players_input: vec![
                 // Player 1
                 // PlayerInput::new(0, Box::new(GamepadInput::new(0, true)), 3.0, true),
                 PlayerInput::new(0, selected_devices.0, 3.0, true),
 
                 // Player 2
                 // PlayerInput::new(1, Box::new(KeyboardInput::new()), 7.0, false),
                 PlayerInput::new(1, selected_devices.1, 7.0, false) 
             ],
             
             ball: Ball::new(
                 Vector2::new(SCREEN_SIZE.x * 0.5, SCREEN_SIZE.y * 0.5), [
                     Color::new(188, 212, 230, 150), // 1 live - #BCD4E6
                     Color::new(137, 207, 240, 150), // 2 lives - #89CFF0
                     Color::new(10, 255, 255, 150)   // 3 lives - #6CB4EE
                 ], 10.8, MAX_PLAYER_SPEED * 0.63
             ),
 
             left_paddle: Paddle::new(
                 Vector2 { 
                     x: PADDLE_PADDING, 
                     y: SCREEN_SIZE.y / 2.0 - PADDLE_SIZE.y / 2.0 
                 }, [
                     Color::new(255, 105, 97, 130), // Player is far - #FF6961
                     Color::new(255, 40, 0, 130)    // Player is close - #FF2800
                 ],
                 PADDLE_SIZE, INITIAL_PADDLE_SPEED,INITIAL_PADDLE_RANGE, 
                 selected_mode == GameMode::Multiplayer, true
             ), 
 
             right_paddle: Paddle::new(
                 Vector2 { 
                     x: SCREEN_SIZE.x - PADDLE_SIZE.x - PADDLE_PADDING, 
                     y: SCREEN_SIZE.y / 2.0 - PADDLE_SIZE.y / 2.0 
                 }, [
                     Color::new(255, 105, 97, 130), // Player is far - #FF6961
                     Color::new(255, 40, 0, 130)    // Player is close - #FF2800
                 ], 
                 PADDLE_SIZE, INITIAL_PADDLE_SPEED, INITIAL_PADDLE_RANGE, 
                 selected_mode == GameMode::Multiplayer, false
             ),
         };
     }

     fn update_difficulty(self: &mut Self) {
        match self.score {
            0 => {
                self.checkpoint = 0;
                self.score_color = Color::DARKGREEN;
                self.ball.speed = MAX_PLAYER_SPEED * 0.63;

                self.left_paddle.speed = INITIAL_PADDLE_SPEED;
                self.right_paddle.speed = INITIAL_PADDLE_SPEED;
                self.left_paddle.view_range = INITIAL_PADDLE_RANGE;
                self.right_paddle.view_range = INITIAL_PADDLE_RANGE;
            },

            10 => {
                self.checkpoint = 10;
                self.score_color = Color::GREEN;
                self.ball.speed = MAX_PLAYER_SPEED * 0.75;

                self.left_paddle.speed = INITIAL_PADDLE_SPEED * 0.9;
                self.right_paddle.speed = INITIAL_PADDLE_SPEED * 0.9;
            },

            25 => {
                self.checkpoint = 25;
                self.score_color = Color::YELLOW;
                self.ball.speed = MAX_PLAYER_SPEED * 0.85;

                self.left_paddle.speed = INITIAL_PADDLE_SPEED * 0.8;
                self.right_paddle.speed = INITIAL_PADDLE_SPEED * 0.8;
                self.left_paddle.view_range = INITIAL_PADDLE_RANGE * 0.8;
                self.right_paddle.view_range = INITIAL_PADDLE_RANGE * 0.8;
            },

            50 => {
                self.checkpoint = 50;
                self.score_color = Color::GOLD;
                self.ball.speed = MAX_PLAYER_SPEED * 0.90;

                self.left_paddle.speed = INITIAL_PADDLE_SPEED * 0.6;
                self.right_paddle.speed = INITIAL_PADDLE_SPEED * 0.6;
                self.left_paddle.view_range = INITIAL_PADDLE_RANGE * 0.75;
                self.right_paddle.view_range = INITIAL_PADDLE_RANGE * 0.75;
            }

            75 => {
                self.checkpoint = 75;
                self.score_color = Color::RED;
                self.ball.speed = MAX_PLAYER_SPEED;
                self.left_paddle.speed = INITIAL_PADDLE_SPEED * 0.5;
                self.right_paddle.speed = INITIAL_PADDLE_SPEED * 0.5;
                self.left_paddle.view_range = INITIAL_PADDLE_RANGE * 0.6;
                self.right_paddle.view_range = INITIAL_PADDLE_RANGE * 0.6;
            } 
            _=> {},
        }
    }
}