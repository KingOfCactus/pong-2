use super::*;
use raylib::consts::GamepadAxis::*;
use raylib::consts::GamepadButton::*;

impl InputDevice for GamepadInput {
    fn use_axis(self: &mut Self) -> bool { return self.use_axis; }
    fn get_axis(self: &mut Self, rl: &RaylibHandle) -> Vector2 {
        // Panic if if the device don't have axis or it's set to not use it
        if !self.use_axis {
            panic!("ERROR: Trying to get axis movement data from a input device without axis (Gamepad {})", self.gamepad_id);
        }
        
        return Vector2::new(rl.get_gamepad_axis_movement(self.gamepad_id, GAMEPAD_AXIS_LEFT_X),
                            rl.get_gamepad_axis_movement(self.gamepad_id, GAMEPAD_AXIS_LEFT_Y));       
    }

    fn get_buttons(self: &mut Self, rl: &RaylibHandle) -> [bool; 4]{
        // Treat left analog as dpad
        if self.use_axis {
            let is_right_down = rl.is_gamepad_button_down(self.gamepad_id, GAMEPAD_BUTTON_LEFT_FACE_RIGHT);
            let is_left_down = rl.is_gamepad_button_down(self.gamepad_id, GAMEPAD_BUTTON_LEFT_FACE_LEFT);
            let is_down_down = rl.is_gamepad_button_down(self.gamepad_id, GAMEPAD_BUTTON_LEFT_FACE_DOWN);
            let is_up_down = rl.is_gamepad_button_down(self.gamepad_id, GAMEPAD_BUTTON_LEFT_FACE_UP);
            return [is_right_down, is_left_down, is_down_down, is_up_down];
        }

        // DPad buttons
        let is_right_down = rl.get_gamepad_axis_movement(self.gamepad_id, GAMEPAD_AXIS_LEFT_X) > GAMEPAD_DEADZONE;
        let is_left_down = rl.get_gamepad_axis_movement(self.gamepad_id, GAMEPAD_AXIS_LEFT_X) < -GAMEPAD_DEADZONE;
        let is_down_down = rl.get_gamepad_axis_movement(self.gamepad_id, GAMEPAD_AXIS_LEFT_Y) < -GAMEPAD_DEADZONE;
        let is_up_down = rl.get_gamepad_axis_movement(self.gamepad_id, GAMEPAD_AXIS_LEFT_Y) > GAMEPAD_DEADZONE;
        return [is_right_down, is_left_down, is_down_down, is_up_down];
    }
}

impl GamepadInput {
    pub fn new(gamepad_id: i32, use_axis: bool) -> Self {
        return Self {
            use_axis: use_axis,
            gamepad_id: gamepad_id,
        };
    }
}
