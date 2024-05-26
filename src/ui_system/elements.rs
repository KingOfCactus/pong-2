use super::*;
use crate::utils::SCREEN_SIZE;

impl Text {
    pub fn new(text: &str, relative_pos: Vector2, color: Color, size: i32,) -> Text {
        Text {
            pos: Vector2 { 
                x: SCREEN_SIZE.x * relative_pos.x - measure_text(text, size) as f32 / 2.0,
                y: SCREEN_SIZE.y * relative_pos.y 
            },
            relative_pos: relative_pos, text: text.to_string(), color: color, size: size,
        }
    }

    pub fn centralize(self: &mut Self) {
        self.pos.x = SCREEN_SIZE.x * self.relative_pos.x 
                   - measure_text(self.text.as_str(), self.size) as f32 / 2.0;
    }
}

impl Button {
    const DEFAULT_COLOR:Color = Color::new(150, 150, 150, 255);
    const FOCUSED_COLOR:Color = Color::WHITE;
    const DISABLED_COLOR:Color = Color::RED;

    pub fn new(enabled: bool, text: &str, relative_pos: Vector2) -> Button {
        Button {
            text: text.to_string(),
            pos: Vector2 {
                x: SCREEN_SIZE.x * relative_pos.x - measure_text(&text, 20) as f32 / 2.0,
                y: SCREEN_SIZE.y * relative_pos.y
            },
            
            rect: Rectangle::new (                                                      
                SCREEN_SIZE.x * relative_pos.x as f32 - (measure_text(&text, 20) + 30) as f32 / 2.0,                 
                SCREEN_SIZE.y * relative_pos.y - 10.0, measure_text(&text, 20) as f32 + 30.0, 
                40.0
            ), enabled
        }
    }

    pub fn get_color(self: &Self, rl: &RaylibHandle) -> Color {
        if self.is_focused(&rl) {
            return Self::FOCUSED_COLOR;
        }

        if !self.enabled {
            return Self::DISABLED_COLOR;
        }

        return Self::DEFAULT_COLOR
    }

    pub fn is_pressed(self: &Self, rl: &RaylibHandle) -> bool {
        let clicked = rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON);
        return self.is_focused(rl) && clicked;
    }

    pub fn is_focused(self: &Self, rl: &RaylibHandle) -> bool {
        let mouse_pos = rl.get_mouse_position();
        return self.rect.check_collision_point_rec(mouse_pos);
    }
}

impl TextField {
    pub fn is_ipv4(self: &mut Self) -> bool{
        let regex = Regex::new("([0-9]{1,3})+([.]+[0-9]{1,3}){3}").expect("Invalid Regular Expression");
        return regex.is_match(&self.text.text);

        todo!("consider moving this code out from the TextField struct");
    }

    pub fn update(self: &mut Self, rl: &RaylibHandle, pointer: Vector2) {        
        let ascii = unsafe { ffi::GetKeyPressed() as u8 }; // gey_key_pressed() uses rl as mutable, can't use
        if ascii == 0 { return; }
        
        let is_placeholder = self.text.text == self.placeholder;        
        if is_placeholder { self.text.text = "".to_string(); }
        
        let is_erasing = ascii == KeyboardKey::KEY_BACKSPACE as u8;
        let is_full = self.text.text.len() >= self.max_length;

        if is_erasing { 
            self.text.text.pop();
            self.text.centralize();
            return;
        }
        else if is_full { return; }

        
        let input = (ascii as char).to_string(); 
        let valid_input = self.format.is_match(&input);
        let focused = self.rects[1].check_collision_point_rec(pointer);

        if !focused || !valid_input { return; }

        if self.text.text == self.placeholder {
            self.text.text = "".to_string();
        }
        
        let new_text = self.text.text.clone() + &input;
        self.text.text = new_text;
        self.text.centralize();
    }

    pub fn new(format: Regex, placeholder: &str, width: f32, text_size: i32,
        relative_pos: Vector2, outline_tickness: f32, colors: Vec<Color>, max_length: usize) -> TextField {
        TextField {
            pos: Vector2 {
                x: SCREEN_SIZE.x * relative_pos.x - measure_text(&placeholder, text_size) as f32 / 2.0,
                y: SCREEN_SIZE.y * relative_pos.y
            },
            
            rects: vec![ 
                Rectangle::new(
                    SCREEN_SIZE.x * relative_pos.x as f32 - width /2.0,
                    SCREEN_SIZE.y * relative_pos.y - 10.0,
                    width, 40.0),

                Rectangle::new(
                    SCREEN_SIZE.x * relative_pos.x as f32 - (width - outline_tickness) / 2.0,
                    SCREEN_SIZE.y * relative_pos.y - 10.0 + outline_tickness/2.0, 
                    width - outline_tickness, 
                    40.0 - outline_tickness
                )
            ],
                
            text: Text::new(placeholder, relative_pos, colors[0], text_size),
            placeholder: placeholder.to_string(),
            value: "".to_string(), max_length,
            colors, format
        }
    }
}