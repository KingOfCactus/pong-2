use super::*;
use crate::utils::{SCREEN_SIZE, get_highscore};

const HOVERING_BTN_COLOR: Color = Color::WHITE;
const BTN_COLOR: Color = Color::new(150, 150, 150, 255);

pub struct Text {
    color: Color,
    text: String,
    size: i32,
    pos: Vector2,
}

pub struct Button {
    rect: Rectangle,
    text: String,
    pos: Vector2,
    focused: bool,
}

impl Text {
    pub fn new(text: &str, relative_pos: Vector2, color: Color, size: i32,) -> Text {
        Text {
            pos: Vector2 { 
                x: SCREEN_SIZE.x * relative_pos.x - measure_text(text, size) as f32 / 2.0,
                y: SCREEN_SIZE.y * relative_pos.y 
            },
            text: text.to_string(), color: color, size: size,
        }
    }
}

impl Button {
    pub fn new(text: &str, relative_pos: Vector2) -> Button {
        Button {
            text: text.to_string(),
            pos: Vector2 {
                x: SCREEN_SIZE.x * relative_pos.x - measure_text(&text, 20) as f32 / 2.0,
                y: SCREEN_SIZE.y * relative_pos.y
            },
            
            focused: false,
            rect: Rectangle::new (
                SCREEN_SIZE.x * relative_pos.x as f32 - (measure_text(&text, 20) + 30) as f32 / 2.0, 
                SCREEN_SIZE.y * relative_pos.y - 10.0, measure_text(&text, 20) as f32 + 30.0, 40.0
            )
        }
    }
}

impl GameScene for MainMenu {
    fn update(self: &mut Self, rl: &RaylibHandle) {
        let mouse_pos = rl.get_mouse_position();

        // Update buttons
        self.singleplayer.focused = self.singleplayer.rect.check_collision_point_rec(mouse_pos) && !self.on_mltplyr_screen;
        self.multiplayer.focused = self.multiplayer.rect.check_collision_point_rec(mouse_pos) && !self.on_mltplyr_screen;
        self.quit.focused = self.quit.rect.check_collision_point_rec(mouse_pos) && !self.on_mltplyr_screen;

        self.local_multiplayer.focused = self.local_multiplayer.rect.check_collision_point_rec(mouse_pos) && self.on_mltplyr_screen;
        self.online_multiplayer.focused = self.online_multiplayer.rect.check_collision_point_rec(mouse_pos) && self.on_mltplyr_screen;

        // Exit if mouse isn't clicking
        if !rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) { return; }

        // Main screen
        if self.singleplayer.focused { self.start_game(false); }
        if self.multiplayer.focused { self.on_mltplyr_screen = true; }
        if self.quit.focused { self.quit(); }

        // Multiplayer screen
        if self.local_multiplayer.focused { self.start_game(true); }
        if self.online_multiplayer.focused { self.quit(); }
        
    }

    fn draw(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        // Clear screen
        let mut draw_handle = rl.begin_drawing(thread);
        draw_handle.clear_background(Color::BLACK);
        
        // Multiplayer Screen
        if self.on_mltplyr_screen {
            draw_handle.draw_text(&self.local_multiplayer.text, self.local_multiplayer.pos.x as i32, self.local_multiplayer.pos.y as i32,
                20 as i32, if self.local_multiplayer.focused {HOVERING_BTN_COLOR} else {BTN_COLOR});

            draw_handle.draw_text(&self.online_multiplayer.text, self.online_multiplayer.pos.x as i32, self.online_multiplayer.pos.y as i32,
                20 as i32, if self.online_multiplayer.focused {HOVERING_BTN_COLOR} else {BTN_COLOR});

            // Draw rects
            // draw_handle.draw_rectangle_rec(&self.local_multiplayer.rect, Color::GRAY);
            // draw_handle.draw_rectangle_rec(&self.online_multiplayer.rect, Color::GRAY);
            
            return;
        }

        // Draw title
        draw_handle.draw_text(&self.title.text, self.title.pos.x as i32, self.title.pos.y as i32, 
                               self.title.size, self.title.color);

        // Draw buttons
        draw_handle.draw_text(&self.singleplayer.text, self.singleplayer.pos.x as i32, self.singleplayer.pos.y as i32,
                               20 as i32, if self.singleplayer.focused {HOVERING_BTN_COLOR} else {BTN_COLOR});

        draw_handle.draw_text(&self.multiplayer.text, self.multiplayer.pos.x as i32, self.multiplayer.pos.y as i32,
                               20 as i32, if self.multiplayer.focused {HOVERING_BTN_COLOR} else {BTN_COLOR});
        
        draw_handle.draw_text(&self.quit.text, self.quit.pos.x as i32, self.quit.pos.y as i32,
                               20 as i32, if self.quit.focused {HOVERING_BTN_COLOR} else {BTN_COLOR});
    
        // Draw score text
        draw_handle.draw_text(&self.hiscore.text, self.hiscore.pos.x as i32, self.hiscore.pos.y as i32, 
                               self.hiscore.size as i32, self.hiscore.color);

        // Draw rects
        // draw_handle.draw_rectangle_rec(&self.singleplayer.rect, Color::GRAY);
        // draw_handle.draw_rectangle_rec(&self.multiplayer.rect, Color::GRAY);
        // draw_handle.draw_rectangle_rec(&self.quit.rect, Color::GRAY);
    }

    fn is_active(&self) -> bool { return self.is_active; }
    fn get_next_scene(&self) -> Box<dyn GameScene> { return Box::new(GameLoop::new(self.selected_mode)); }
}

impl MainMenu {
    fn quit(self: &mut Self) { todo!("Implement this") }

    fn start_game(self: &mut Self, selected_multiplayer: bool) {
        if selected_multiplayer { self.selected_mode = GameMode::Multiplayer }
        self.is_active = false;
    }

    pub fn new() -> MainMenu {
        return MainMenu {
            title: Text::new(
                "Pong 2: The Enemy is Now Another", 
                Vector2::new(0.5, 0.1), 
                Color::GOLD, 
                26
            ),
            
            hiscore: Text::new(
                &format!("HiScore: {}", get_highscore()), 
                Vector2::new(0.5, 0.95),
                Color::WHITE, 
                16
            ),

            singleplayer: Button::new(
                "Singleplayer", 
                Vector2::new(0.5, 0.4)
            ),

            multiplayer: Button::new(
                "Multiplayer", 
                Vector2::new(0.5, 0.5)
            ),
            
            quit: Button::new(
                "Quit", 
                Vector2::new(0.5, 0.6)
            ),
            

            on_mltplyr_screen: false,
            selected_mode: GameMode::Singleplayer,

            local_multiplayer: Button::new(
                "Local Multiplayer", 
                Vector2::new(0.5, 0.4)
            ),
            
            online_multiplayer: Button::new(
                "Online Multiplayer", 
                Vector2::new(0.5, 0.5)
            ),

            is_active: true,
        }                
    }
}