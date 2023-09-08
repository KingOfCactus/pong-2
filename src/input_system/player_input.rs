use super::*;

impl PlayerInput {
    pub fn get_data(self: &mut Self, rl: &RaylibHandle) -> InputData {
        // Update data if wasn't already this frame
        if self.last_data.sample_time != rl.get_time() {
            self.last_data = self.read_data(&rl)
        }

        return self.last_data;
    }

    fn read_data(self: &mut Self, rl: &RaylibHandle) -> InputData {
        let mut data = InputData::new(rl.get_time());
        let last_dir = self.last_data.dir;

        // Get buttons
        let buttons = self.device.get_buttons(&rl);
        data.is_right_down = buttons[0];
        data.is_left_down = buttons[1];
        data.is_down_down = buttons[2];
        data.is_up_down = buttons[3];

        // Get raw direction
        if self.device.use_axis() { data.raw_dir = self.device.get_axis(&rl); }
        else { data.raw_dir = self.buttons_to_dir(&buttons); }

        // Smooth raw direction to dir
        data.dir = last_dir.lerp(data.raw_dir, self.input_snapness * rl.get_frame_time());
        return data;
    }

    // Convert buttons to a direction vector, while applying SOCD cleaning
    // (https://www.hitboxarcade.com/blogs/support/what-is-socd)
    fn buttons_to_dir(self: &mut Self, raw_data: &[bool; 4]) -> Vector2 {
        let mut dir = Vector2::zero();
        let is_right_down = raw_data[0];
        let is_left_down = raw_data[1];
        let is_down_down = raw_data[2];
        let is_up_down = raw_data[3];

        // Use First-Input Priority resolution
        if self.first_input_socd {
            if !is_right_down && !is_left_down { dir.x = 0.0; }
            else if is_right_down && !is_left_down { dir.x = 1.0; }
            else if !is_right_down { dir.x = -1.0; }
            
            if !is_down_down && !is_up_down { dir.y = 0.0; }
            else if is_down_down && !is_up_down { dir.y = 1.0; }
            else if !is_down_down {dir.y = -1.0; }
        }
        // Use Neutral resolution 
        else {
            if is_left_down && is_right_down { dir.x = 0.0; }
            else if is_left_down { dir.x = -1.0}
            else if is_right_down { dir.x = 1.0}
            else { dir.x = 0.0 }

            if is_up_down && is_down_down { dir.y = 0.0; }
            else if is_up_down { dir.y = -1.0}
            else if is_down_down { dir.y = 1.0}
            else { dir.y = 0.0 }
        }

        return dir;
    }

    pub fn new(player_id: i32, device: Box<dyn InputDevice>, snapness: f32, use_first_input: bool) -> Self {
        return Self { 
            id: player_id, 
            input_snapness: snapness, 
            last_data: InputData::new(0.0), 
            first_input_socd: use_first_input, 
            device: device
        }
    }
}