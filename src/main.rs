mod input_system;
mod game_objects;
mod utils;

use crate::utils::*;
use crate::game_objects::*;

use rand::*;
use raylib::prelude::*;
use raylib::ffi::KeyboardKey::*;
use raylib::ffi::GamepadAxis::*;
use raylib::ffi::GamepadButton::*;

fn main() {
    let (mut rl, thread) = init_window();
    let (mut player, mut left_paddle, mut right_paddle) = start_game();

    let mut score = 0;
    let mut show_stats = true;
    let mut highscore = get_highscore(); 

    // Each frame
    while !rl.window_should_close() {
        // <-- GAME LOGIC -->
        player.update(&rl);

        left_paddle.player_pos = player.position;
        left_paddle.update(&rl);

        right_paddle.player_pos = player.position;
        right_paddle.update(&rl);

        // Restart if player is outside of the screen
        if player.position.x > SCREEN_SIZE.x || player.position.x < 0.0 {

            // Reset variables
            player.position = SCREEN_SIZE / 2.0;
            left_paddle.position.y =  SCREEN_SIZE.y / 2.0 - PADDLE_SIZE.y / 2.0;
            right_paddle.position.y =  SCREEN_SIZE.y / 2.0 - PADDLE_SIZE.y / 2.0;

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
        let hit_paddle = left_paddle.hitbox.check_collision_circle_rec(player.position, player.radius + 5.0) ||
                         right_paddle.hitbox.check_collision_circle_rec(player.position, player.radius + 5.0);
        
        if hit_paddle {
            let mut new_angle: f32 = thread_rng().gen();

            // Copy player.input direction or keep previous direction 
            if player.input.raw_dir.y == 0.0 { new_angle *= player.prone_dir.y.signum(); }
            else { new_angle *= player.input.raw_dir.y.signum(); }
            
            // Keep player out of the paddles
            let min = left_paddle.position.x + PADDLE_SIZE.x + player.radius;
            let max = right_paddle.position.x - PADDLE_SIZE.x - player.radius;
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

        let mut draw_handle = rl.begin_drawing(&thread);
        draw_handle.clear_background(Color::BLACK);

        if show_stats {
            draw_handle.draw_fps(0, 0);
            draw_handle.draw_text(&stats, 0, (SCREEN_SIZE.y * 0.05) as i32, 18, Color::GREEN);
        }

        let text = "Hiscore: ".to_owned() + &highscore.to_string() + "\nScore: " + &score.to_string();
        let text_x = SCREEN_SIZE.x / 2.0 - (measure_text(&text, 22) as f32 / 2.0);

        draw_handle.draw_text(&text, text_x as i32, (SCREEN_SIZE.y * 0.01) as i32, 22, Color::RED);
        
        draw_handle.draw_circle_v(player.position, player.radius, player.color);
        draw_handle.draw_rectangle_rec(&left_paddle.hitbox, Color::GRAY);
        draw_handle.draw_rectangle_rec(&right_paddle.hitbox, Color::GRAY);
    }
}