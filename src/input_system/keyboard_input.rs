use super::*;
use raylib::consts::KeyboardKey::*;

impl InputDevice for KeyboardInput {
    // A keyboard don't have axis
    fn use_axis(self: &mut Self) -> bool { return false; }
    fn get_axis(self: &mut Self, _rl: &RaylibHandle) -> Vector2 {
        panic!("ERROR: Trying to get axis movement data from a input device without axis (Keyboard)");      
    }

    fn get_buttons(self: &mut Self, rl: &RaylibHandle) -> [bool; 4] {
        // Use WASD
        if self.use_wasd {
            return [rl.is_key_down(KEY_D), rl.is_key_down(KEY_A),
                    rl.is_key_down(KEY_S), rl.is_key_down(KEY_W)
            ]
        }

        // Use arrows
        return [rl.is_key_down(KEY_RIGHT), rl.is_key_down(KEY_LEFT),
                rl.is_key_down(KEY_DOWN), rl.is_key_down(KEY_UP)
        ]
    }
}

impl KeyboardInput {
    pub fn new(use_wasd: bool) -> Self {
        return Self { use_wasd };
    }
}
