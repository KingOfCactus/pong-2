use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;
use raylib::consts::GamepadAxis::*;
use raylib::consts::GamepadButton::*;

const VECTOR_ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
const GAMEPAD_DEADZONE: f32 = 0.7;

pub struct InputData {
    pub on_gamepad: bool,

    pub smoothness : f32,
    pub raw_dir: Vector2,
    pub dir: Vector2,

    pub is_right_down: bool,
    pub is_left_down: bool,
    pub is_down_down: bool,
    pub is_up_down: bool
}

impl InputData {
    pub fn new() -> Self {
        return Self {
            on_gamepad: false,

            smoothness: 0.0,
            raw_dir: VECTOR_ZERO,
            dir: VECTOR_ZERO,

            is_right_down: false,
            is_left_down: false,
            is_down_down: false,
            is_up_down: false,
        };
    }

    pub fn update_data(input: &mut Self, rl: &RaylibHandle) {
        // Update gamepad data
        input.on_gamepad = rl.is_gamepad_available(0);
        let gamepad_axis = Vector2 {
            x: rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_X),
            y: rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_Y)
        };
    
        // Update buttons data
        if input.on_gamepad {
            input.is_right_down = rl.is_gamepad_button_down(0, GAMEPAD_BUTTON_LEFT_FACE_RIGHT) || rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_X) > GAMEPAD_DEADZONE;
            input.is_left_down = rl.is_gamepad_button_down(0, GAMEPAD_BUTTON_LEFT_FACE_LEFT) || rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_X) < -GAMEPAD_DEADZONE;
            input.is_down_down = rl.is_gamepad_button_down(0, GAMEPAD_BUTTON_LEFT_FACE_DOWN) || rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_Y) < -GAMEPAD_DEADZONE;
            input.is_up_down = rl.is_gamepad_button_down(0, GAMEPAD_BUTTON_LEFT_FACE_UP) || rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_Y) > GAMEPAD_DEADZONE;
        }
        else {
            input.is_right_down = rl.is_key_down(KEY_D) || rl.is_key_down(KEY_RIGHT);
            input.is_left_down = rl.is_key_down(KEY_A) || rl.is_key_down(KEY_LEFT);
            input.is_down_down = rl.is_key_down(KEY_S) || rl.is_key_down(KEY_DOWN);
            input.is_up_down = rl.is_key_down(KEY_W) || rl.is_key_down(KEY_UP);
        }
    
        // Update raw direction
        if input.on_gamepad && gamepad_axis != VECTOR_ZERO {
            input.raw_dir.x =  rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_X);
            input.raw_dir.y =  rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_Y);
        }
        else {
            if !input.is_right_down && !input.is_left_down { input.raw_dir.x = 0.0; }
            else if input.is_right_down && !input.is_left_down { input.raw_dir.x = 1.0; }
            else if !input.is_right_down { input.raw_dir.x = -1.0; }
            
            if !input.is_down_down && !input.is_up_down { input.raw_dir.y = 0.0; }
            else if input.is_down_down && !input.is_up_down { input.raw_dir.y = 1.0; }
            else if !input.is_down_down {input.raw_dir.y = -1.0; }
        }
    
        // Smooth raw_dir into input direction
        input.dir = input.dir.lerp(input.raw_dir, input.smoothness * rl.get_frame_time());
    }
}