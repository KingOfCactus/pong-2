use crate::game_objects::*;

use std::fs;
use std::io::Write;
use raylib::prelude::*;
use raylib::prelude::Vector2;

const PADDLE_PADDING: f32 = 20.0;
const INITIAL_PADDLE_RANGE: f32 = 0.5;
const INITIAL_PADDLE_SPEED: f32 = 500.0;

pub const PADDLE_SIZE: Vector2 = Vector2 { x: 11.0, y: 65.0 };
pub const SCREEN_SIZE: Vector2 = Vector2 { x: 640.0, y: 480.0 };


pub fn get_highscore() -> i32 {
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

pub fn save_highscore(i: i32) {
    let mut file = fs::OpenOptions::new().write(true).open("highscore.txt").unwrap();
    let buffer: String = i.to_string();
    file.write_all(buffer.as_bytes()).unwrap();
}


pub fn init_window() -> (RaylibHandle, RaylibThread) {
    let (mut rl_handle, _thread) = raylib::init()
        .size(SCREEN_SIZE.x as i32, SCREEN_SIZE.y as i32)
        .title("Pong 2").vsync().build();

    rl_handle.set_target_fps(60);
    return (rl_handle, _thread);
}

pub fn init_gobjects() -> (Ball, Paddle, Paddle) {
        let player = Ball::new(
            Vector2 { x: SCREEN_SIZE.x * 0.9, y: SCREEN_SIZE.y * 0.5 },
            Color { r: 255, g: 255, b: 255, a: 185},
            10.0,
            500.0,
        );

        let left_paddle = Paddle::new(
            Vector2 { 
                x: PADDLE_PADDING, 
                y: SCREEN_SIZE.y / 2.0 - PADDLE_SIZE.y / 2.0 
            },
            Color::WHITE, PADDLE_SIZE, 
            INITIAL_PADDLE_SPEED,
            INITIAL_PADDLE_RANGE
        );

        let right_paddle = Paddle::new(
            Vector2 { 
                x: SCREEN_SIZE.x - PADDLE_SIZE.x - PADDLE_PADDING, 
                y: SCREEN_SIZE.y / 2.0 - PADDLE_SIZE.y / 2.0 
            },
            Color::WHITE, PADDLE_SIZE, 
            INITIAL_PADDLE_SPEED,
            INITIAL_PADDLE_RANGE
        );

        return (player, left_paddle, right_paddle);
}


pub fn get_debug_info(player: &Ball) -> String {
    let input = &player.input;
    let mut stats = format!("- Prone: ({:.2}, {:.2}) \n- Move: ({:.2}, {:.2}\n", 
                    player.prone_dir.x, player.prone_dir.y, 
                    input.dir.x, input.dir.y);

    if player.input.on_gamepad {
        stats += &format!(" {} \n ({:.2}, {:.2})", input.gamepad_name, 
                 input.raw_dir.x, input.raw_dir.y);
    }
    
    return stats;
}

pub fn drawn_screen (debug: &bool, rl: &mut RaylibHandle, thread: &RaylibThread, 
                     player: &Ball, left_paddle: &Paddle, right_paddle: &Paddle,
                     highscore: &i32, score: &i32) {
    // Clear screen
    let mut draw_handle = rl.begin_drawing(thread);
    draw_handle.clear_background(Color::BLACK);

    // Draw score text
    let text = format!("Hiscore: {}\n Score: {}", highscore, score);
    let centralized_x = SCREEN_SIZE.x / 2.0 - (measure_text(&text, 22) as f32 / 2.0);
    draw_handle.draw_text(&text, centralized_x as i32, (SCREEN_SIZE.y * 0.01) as i32, 22, Color::RED);
    
    // Draw game objects
    draw_handle.draw_circle_v(player.position, player.radius, player.color);
    draw_handle.draw_rectangle_rec(&left_paddle.hitbox, Color::GRAY);
    draw_handle.draw_rectangle_rec(&right_paddle.hitbox, Color::GRAY);

    // Draw debug info
    if *debug {
        let stats = get_debug_info(&player);
        draw_handle.draw_fps(0, 0);
        draw_handle.draw_text(&stats, 0, (SCREEN_SIZE.y * 0.05) as i32, 18, Color::GREEN);
    }
}
