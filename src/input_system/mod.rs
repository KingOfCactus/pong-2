mod keyboard_input;
mod gamepad_input;
mod player_data;
use raylib::prelude::*;

const GAMEPAD_DEADZONE: f32 = 0.15;

#[derive(Clone, Copy)]
pub struct InputData {
    pub is_right_down: bool,
    pub is_left_down: bool,
    pub is_down_down: bool,
    pub is_up_down: bool,
    
    pub raw_dir: Vector2,
    pub dir: Vector2
}

impl InputData {
    fn new() -> Self {
        return Self {
            raw_dir: Vector2::zero(), 
            dir: Vector2::zero(),
            is_right_down: false, 
            is_left_down: false, 
            is_down_down: false, 
            is_up_down: false,
        }
    }
}

pub trait InputDevice {
    fn get_buttons(self: &mut Self, rl: &RaylibHandle) -> [bool; 4];
    fn get_axis(self: &mut Self, rl: &RaylibHandle) -> Vector2;
    fn use_axis(self: &mut Self) -> bool;
}

pub struct PlayerData {
    id: i32,
    input_snapness: f32,
    last_input: InputData,

    first_input_socd: bool,
    device: Box<dyn InputDevice>,
}

pub struct KeyboardInput;
pub struct GamepadInput {
    gamepad_id: i32,
    use_axis: bool
}