use crate::utils::SCREEN_SIZE;
use super::*;

const HOVERING_BTN_COLOR: Color = Color::WHITE;
const BTN_COLOR: Color = Color::new(150, 150, 150, 255);

pub struct Text {
    color: Color,
    text: String,
    pos: Vector2,
}

pub struct Button {
    rect: Rectangle,
    text: String,
    pos: Vector2,
    focused: bool,
}

impl GameState for MainMenu {
    fn update(&mut self, rl: &RaylibHandle) {
        let mouse_pos = rl.get_mouse_position();

        // Update buttons
        self.play.focused = self.play.rect.check_collision_point_rec(mouse_pos);
        self.config.focused = self.config.rect.check_collision_point_rec(mouse_pos);
        self.quit.focused = self.quit.rect.check_collision_point_rec(mouse_pos);

        // Exit if mouse isn't clicking
        if !rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            return;
        }

        if self.play.focused {  }
        if self.config.focused { println!("Config") }
        if self.quit.focused { println!("Quit") }
    }

    fn draw(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        // Clear screen
        let mut draw_handle = rl.begin_drawing(thread);
        draw_handle.clear_background(Color::BLACK);

        // Texts
        draw_handle.draw_text(&self.title.text, self.title.pos.x as i32, self.title.pos.y as i32, 26 as i32, self.title.color);

        // Buttons
        // draw_handle.draw_rectangle_rec(&self.play.rect, Color::GRAY);
        draw_handle.draw_text(&self.play.text, self.play.pos.x as i32, self.play.pos.y as i32,
                              20 as i32, if self.play.focused {HOVERING_BTN_COLOR} else {BTN_COLOR});

        // draw_handle.draw_rectangle_rec(&self.config.rect, Color::GRAY);
        draw_handle.draw_text(&self.config.text, self.config.pos.x as i32, self.config.pos.y as i32,
            20 as i32, if self.config.focused {HOVERING_BTN_COLOR} else {BTN_COLOR});

        // draw_handle.draw_rectangle_rec(&self.quit.rect, Color::GRAY);
        draw_handle.draw_text(&self.quit.text, self.quit.pos.x as i32, self.quit.pos.y as i32,
            20 as i32, if self.quit.focused {HOVERING_BTN_COLOR} else {BTN_COLOR});

    }
}

impl MainMenu {
    pub fn new() -> MainMenu {
        return MainMenu {
            title: Text {
                color: Color::GOLD,
                text: "Pong 2 - The Enemy is Now Another".to_string(),
                pos: Vector2 { 
                    x: SCREEN_SIZE.x / 2.0 - measure_text("Pong 2 - The Enemy is Now Another", 26) as f32 / 2.0,
                    y: SCREEN_SIZE.y * 0.1 
                }
            },

            play: Button {
                text: "Play".to_string(),
                pos: Vector2 {
                    x: SCREEN_SIZE.x / 2.0 - measure_text("Play", 20) as f32 / 2.0,
                    y: SCREEN_SIZE.y * 0.4
                },

                focused: false,
                rect: Rectangle::new (
                    SCREEN_SIZE.x / 2.0 as f32 - (measure_text("Play", 20) + 30) as f32 / 2.0, 
                    SCREEN_SIZE.y * 0.4 - 10.0, measure_text("Play", 20) as f32 + 30.0, 40.0
                )
            },
            
            config: Button {
                text: "Settings".to_string(),
                pos: Vector2 {
                    x: SCREEN_SIZE.x / 2.0 - measure_text("Settings", 20) as f32 / 2.0,
                    y: SCREEN_SIZE.y * 0.5
                },

                focused: false,
                rect: Rectangle::new (
                    SCREEN_SIZE.x / 2.0 as f32 - (measure_text("Settings", 20) + 30) as f32 / 2.0, 
                    SCREEN_SIZE.y * 0.5 - 10.0, measure_text("Settings", 20) as f32 + 30.0, 40.0
                )
            },
            
            quit: Button {
                focused: false,
                pos: Vector2 {
                    x: SCREEN_SIZE.x / 2.0 - measure_text("Quit", 20) as f32 / 2.0,
                    y: SCREEN_SIZE.y * 0.6
                },
                
                text: "Quit".to_string(),
                rect: Rectangle::new (
                    SCREEN_SIZE.x / 2.0 as f32 - (measure_text("Quit", 20) + 30) as f32 / 2.0, 
                    SCREEN_SIZE.y * 0.6 - 10.0, measure_text("Quit", 20) as f32 + 30.0, 40.0
                )
            }
        }                
    }
}
