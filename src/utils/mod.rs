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

pub fn init_window() -> (RaylibHandle, RaylibThread) {
    let (mut rl_handle, _thread) = raylib::init()
        .size(SCREEN_SIZE.x as i32, SCREEN_SIZE.y as i32)
        .title("Pong 2").vsync().build();

    rl_handle.set_target_fps(60);
    return (rl_handle, _thread);
}

pub fn save_highscore(i: i32) {
    let mut file = fs::OpenOptions::new().write(true).open("highscore.txt").unwrap();
    let mut buffer: String = i.to_string();
    file.write_all(buffer.as_bytes()).unwrap();
}

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

pub fn start_game() -> (Ball, Paddle, Paddle) {
        let mut player = Ball::new(
            Vector2 { x: SCREEN_SIZE.x * 0.9, y: SCREEN_SIZE.y * 0.5 },
            Color { r: 255, g: 255, b: 255, a: 185},
            10.0,
            500.0,
        );

        let mut left_paddle = Paddle::new(
            Vector2 { 
                x: PADDLE_PADDING, 
                y: SCREEN_SIZE.y / 2.0 - PADDLE_SIZE.y / 2.0 
            },
            Color::WHITE, PADDLE_SIZE, 
            INITIAL_PADDLE_SPEED,
            INITIAL_PADDLE_RANGE
        );

        let mut right_paddle = Paddle::new(
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
