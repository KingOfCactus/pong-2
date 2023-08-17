use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;
use raylib::consts::GamepadAxis::*;
use raylib::consts::GamepadButton::*;

const GAMEPAD_DEADZONE: f32 = 0.20;

pub struct InputData {
    pub on_gamepad: bool,
    pub gamepad_name: String,

    pub smoothness : f32,
    pub raw_dir: Vector2,
    pub dir: Vector2,

    pub is_right_down: bool,
    pub is_left_down: bool,
    pub is_down_down: bool,
    pub is_up_down: bool
}

impl InputData {
    pub fn new(smoothness:f32) -> Self {
        return Self {
            on_gamepad: false,
            gamepad_name: String::new(),
            
            smoothness: smoothness,
            raw_dir: Vector2::zero(),
            dir: Vector2::zero(),

            is_right_down: false,
            is_left_down: false,
            is_down_down: false,
            is_up_down: false,
        };
    }

    pub fn update_data(input: &mut Self, rl: &RaylibHandle) {
        // Update gamepad data
        input.on_gamepad = rl.is_gamepad_available(0);
        let mut gamepad_axis = Vector2 {
            x: rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_X),
            y: rl.get_gamepad_axis_movement(0, GAMEPAD_AXIS_LEFT_Y)
        };

        // Ignore axis movements in the deadzone
        if gamepad_axis.x.abs() < GAMEPAD_DEADZONE { gamepad_axis.x = 0.0; } 
        if gamepad_axis.y.abs() < GAMEPAD_DEADZONE { gamepad_axis.y = 0.0; }

        if input.on_gamepad { input.gamepad_name = rl.get_gamepad_name(0).expect("UNKNOWN"); }
        
        // Update buttons data
        if input.on_gamepad {
            input.is_right_down = rl.is_gamepad_button_down(0, GAMEPAD_BUTTON_LEFT_FACE_RIGHT) || gamepad_axis.x > GAMEPAD_DEADZONE;
            input.is_left_down = rl.is_gamepad_button_down(0, GAMEPAD_BUTTON_LEFT_FACE_LEFT) || gamepad_axis.x < -GAMEPAD_DEADZONE;
            input.is_down_down = rl.is_gamepad_button_down(0, GAMEPAD_BUTTON_LEFT_FACE_DOWN) || gamepad_axis.y < -GAMEPAD_DEADZONE;
            input.is_up_down = rl.is_gamepad_button_down(0, GAMEPAD_BUTTON_LEFT_FACE_UP) || gamepad_axis.y > GAMEPAD_DEADZONE;
        }
        else {
            input.is_right_down = rl.is_key_down(KEY_D) || rl.is_key_down(KEY_RIGHT);
            input.is_left_down = rl.is_key_down(KEY_A) || rl.is_key_down(KEY_LEFT);
            input.is_down_down = rl.is_key_down(KEY_S) || rl.is_key_down(KEY_DOWN);
            input.is_up_down = rl.is_key_down(KEY_W) || rl.is_key_down(KEY_UP);
        }
    
        // Update raw direction
        if input.on_gamepad && gamepad_axis != Vector2::zero() {
            input.raw_dir.x =  gamepad_axis.x;
            input.raw_dir.y =  gamepad_axis.y;
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