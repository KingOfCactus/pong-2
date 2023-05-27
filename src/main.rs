mod input_system;
mod game_objects;
mod utils;

use crate::game_objects::*;
use crate::utils::*;

use rand;
use std::fs;
use std::io::Write;
use rand::Rng;
use rand::thread_rng;
use raylib::prelude::*;
use raylib::prelude::Vector2;
use raylib::consts::KeyboardKey::*;
use raylib::consts::GamepadAxis::*;
use raylib::consts::GamepadButton::*;

fn main() {
    let (mut rl, _thread) = raylib::init()
        .size(SCREEN_SIZE.x as i32, SCREEN_SIZE.y as i32)
        .title("Pong 2").vsync().build();
    
    rl.set_target_fps(60);

    let mut show_stats = true;

    let mut player = Ball::new (
        Vector2 { x: SCREEN_SIZE.x * 0.5, y: SCREEN_SIZE.y * 0.5 },
        Color { r: 255, g: 255, b: 255, a: 185},
        10.0,
        500.0,
    );
    
    let mut score = 0;
    player.input.smoothness = 3.0;
    
    let mut left_paddle = Rectangle::new( 
        PADDLE_PADDING, 
        SCREEN_SIZE.y / 2.0 - PADDLE_SIZE.y / 2.0,
        PADDLE_SIZE.x,
        PADDLE_SIZE.y
    );

    let mut right_paddle = Rectangle::new(
        SCREEN_SIZE.x - PADDLE_SIZE.x - PADDLE_PADDING,
        SCREEN_SIZE.y / 2.0 - PADDLE_SIZE.y / 2.0,
        PADDLE_SIZE.x,
        PADDLE_SIZE.y
    );

    let mut highscore = get_highscore();

    player.prone_dir = Vector2 { x: -1.0, y: 0.0}; 

    // Each frame
    while !rl.window_should_close() {
        // <-- GAME LOGIC -->

        player.update(&mut rl);

        // TODO: Remove from main
        player.position.y = player.position.y.clamp(player.radius, SCREEN_SIZE.y - player.radius);

        // Restart if player is outside of the screen
        if player.position.x > SCREEN_SIZE.x || player.position.x < 0.0 {

            // Reset variables
            player.position = SCREEN_SIZE / 2.0;
            left_paddle.y =  SCREEN_SIZE.y / 2.0 - PADDLE_SIZE.y / 2.0;
            right_paddle.y =  SCREEN_SIZE.y / 2.0 - PADDLE_SIZE.y / 2.0;

            player.input.dir = Vector2::zero();
            player.prone_dir = Vector2 { x: -1.0, y: 0.0 };

            // Check for a new highscore
            if score > highscore { 
                save_highscore(score);
                highscore = score;
            }
            else { score = 0 }

        }

        // Bounce when hit a paddle
        let hit_left_paddle = left_paddle.check_collision_circle_rec(player.position, player.radius + 5.0);
        let hit_right_paddle = right_paddle.check_collision_circle_rec(player.position, player.radius + 5.0);
        
        if hit_left_paddle || hit_right_paddle {
            let mut new_angle: f32 = thread_rng().gen();

            // Copy player.input direction or keep previous direction 
            if player.input.raw_dir.y == 0.0 { new_angle *= player.prone_dir.y.signum(); }
            else { new_angle *= player.input.raw_dir.y.signum(); }
            
            // Keep player out of the paddles
            let min = left_paddle.x + PADDLE_SIZE.x + player.radius;
            let max = right_paddle.x - PADDLE_SIZE.x - player.radius;
            player.position = player.position.clamp(min, max);

            // Set new direction
            player.prone_dir.x *= -1.0;
            player.prone_dir.y = new_angle;
            player.input.dir = Vector2 { x: 0.0, y: 0.0 };

            // Set new player stats
            // player_velocity = VECTOR_ZERO;          
            score += 1;
        }

        // Bounce when hit top or bottom screen
        if player.position.y == player.radius || player.position.y == SCREEN_SIZE.y - player.radius {
            let mut new_angle = player.prone_dir.y.abs();
            new_angle = new_angle.clamp(0.6, 1.0);

            new_angle *= -player.prone_dir.y.signum();
            player.prone_dir.y = new_angle;
            
            player.input.dir.x *= 0.5;
            player.input.dir.y = 0.0;
        }

        // LEFT PADDLET
        let mut paddlet_speed: f32 = 500.0;
        let mut paddlet_view_range = 0.5;
        
        let player_distance = (1.0 - ((player.position.x + left_paddle.x) / (SCREEN_SIZE.x * paddlet_view_range))).powf(2.0);

        let mut left_paddlet_target = (player.position.y - left_paddle.y) * player_distance * 80.0;
        if left_paddlet_target.abs() < 20.0 { left_paddlet_target = 0.0 }
        else { left_paddlet_target = left_paddlet_target.clamp(-paddlet_speed, paddlet_speed) }

        if player.position.x - left_paddle.x > SCREEN_SIZE.x * paddlet_view_range { left_paddlet_target = 0.0 }

        left_paddle.y += left_paddlet_target * rl.get_frame_time();


        // RIGHT PADDLET
        let player_distance = ((player.position.x - SCREEN_SIZE.x * paddlet_view_range) / (right_paddle.x - SCREEN_SIZE.x * paddlet_view_range)).powf(2.0);

        let mut right_paddlet_target = (player.position.y - right_paddle.y) * player_distance * 80.0;
        if right_paddlet_target.abs() < 10.0 { right_paddlet_target = 0.0 }
        else { right_paddlet_target = right_paddlet_target.clamp(-paddlet_speed, paddlet_speed) }

        if (right_paddle.x - player.position.x > SCREEN_SIZE.x * paddlet_view_range) { right_paddlet_target = 0.0 }

        right_paddle.y += right_paddlet_target * rl.get_frame_time();


        // <-- DRAW SCREEN -->
        if rl.is_key_pressed(KEY_TAB) || rl.is_gamepad_button_pressed(0, GAMEPAD_BUTTON_MIDDLE_RIGHT) { show_stats = !show_stats; }
        let mut stats;
        stats = "(".to_owned() + format!("{:.2}", player.prone_dir.x).as_str() + ", " + format!("{:.2}", player.prone_dir.y).as_str() + ")\n";
        stats = stats + "(" + format!("{:.2}", player.input.dir.x).as_str() + ", " + format!("{:.2}", player.input.dir.y).as_str() + ")\n";
        if player.input.on_gamepad {
            stats = stats + &rl.get_gamepad_name(0).expect("UNKNOWN") + " Connected \n";
            stats = stats + "(" + format!("{:.2}", rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_X)).as_str() + ", " + format!("{:.2}", rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_Y)).as_str() + ")\n";
        }
        else {
            let mut keys_down = "".to_string();
            if player.input.is_up_down { keys_down += "w "; }
            if player.input.is_down_down { keys_down += "s "; }
            if player.input.is_left_down { keys_down += "a "; }
            if player.input.is_right_down { keys_down += "d "; }
            stats += &keys_down;
        }

        let mut draw_handle = rl.begin_drawing(&_thread);
        draw_handle.clear_background(Color::BLACK);

        if show_stats {
            draw_handle.draw_fps(0, 0);
            draw_handle.draw_text(&stats, 0, (SCREEN_SIZE.y * 0.05) as i32, 18, Color::GREEN);
        }

        let text = "Hiscore: ".to_owned() + &highscore.to_string() + "\nScore: " + &score.to_string();
        let text_x = SCREEN_SIZE.x / 2.0 - (measure_text(&text, 22) as f32 / 2.0);

        draw_handle.draw_text(&text, text_x as i32, (SCREEN_SIZE.y * 0.01) as i32, 22, Color::RED);
        
        draw_handle.draw_circle_v(player.position, player.radius, player.color);
        draw_handle.draw_rectangle_rec(&left_paddle, Color::GRAY);
        draw_handle.draw_rectangle_rec(&right_paddle, Color::GRAY);
    }
    






    fn get_highscore() -> i32 {
        match fs::read_to_string("highscore.txt") {
            Ok(s) => return s.parse::<i32>().unwrap(),

            // Create file if doesn't exist
            _ => { 
                println!("File 'highscore.txt' doesn't exist, creating...");
                let mut file = fs::File::create("highscore.txt").unwrap();
                file.write_all(b"0").unwrap();
                return 0;
             }
        }
    }

    fn save_highscore(i: i32) {
        let mut file = fs::OpenOptions::new().write(true).open("highscore.txt").unwrap();
        let mut buffer: String = i.to_string();

        file.write_all(buffer.as_bytes()).unwrap();
    }
}