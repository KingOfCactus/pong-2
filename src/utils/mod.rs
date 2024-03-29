use std::fs;
use std::io::Write;
use raylib::prelude::*;
use raylib::prelude::Vector2;

use crate::input_system::{InputDevice, KeyboardInput, GamepadInput};

pub const SCREEN_SIZE: Vector2 = Vector2 { x: 640.0, y: 480.0 };
const MAX_CONNECTED_GAMEPADS: usize = 4;

pub fn init_window() -> (RaylibHandle, RaylibThread) {
    let (mut rl_handle, _thread) = raylib::init()
        .size(SCREEN_SIZE.x as i32, SCREEN_SIZE.y as i32)
        .title("Pong 2").vsync().build();

    rl_handle.set_target_fps(60);
    return (rl_handle, _thread);
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

pub fn save_highscore(i: i32) {
    let mut file = fs::OpenOptions::new().write(true).open("highscore.txt").unwrap();
    let buffer: String = i.to_string();
    file.write_all(buffer.as_bytes()).unwrap();
}

pub fn get_connected_devices(rl: &RaylibHandle) -> Vec<Box<dyn InputDevice>> {
    let mut devices: Vec<Box<dyn InputDevice>> = Vec::with_capacity(MAX_CONNECTED_GAMEPADS + 2);
    devices.insert(0, Box::new(KeyboardInput::new(true)));
    devices.insert(1, Box::new(KeyboardInput::new(false)));

    for i in 0..MAX_CONNECTED_GAMEPADS{
        if rl.is_gamepad_available(i as i32) {
            devices.insert(i + 2, Box::new(GamepadInput::new(i as i32, true)));
        }
    }

    return devices;
}

pub fn get_device_by_id(id: i32) -> Box<dyn InputDevice>{
    match id {
        0 => return Box::new(KeyboardInput::new(true)),
        1 => return Box::new(KeyboardInput::new(false)),
        _ => return Box::new(GamepadInput::new(id - 2, true))
    }
}