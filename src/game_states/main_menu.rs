use crate::utils::{SCREEN_SIZE, get_highscore};
use super::*;

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

impl GameState for MainMenu {
    fn is_active(&self) -> bool {
        return self.is_active;
    }

    fn get_next_state(&self) -> Box<dyn GameState> {
        return Box::new(GameLoop::new());
    }

    fn update(&mut self, rl: &RaylibHandle) {
        let mouse_pos = rl.get_mouse_position();

        // Update buttons
        self.singleplayer.focused = self.singleplayer.rect.check_collision_point_rec(mouse_pos);
        self.quit.focused = self.quit.rect.check_collision_point_rec(mouse_pos);

        // Exit if mouse isn't clicking
        if !rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            return;
        }

        if self.singleplayer.focused { self.on_click_play() }
        if self.quit.focused { println!("Quit") }
    }

    fn draw(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        // Clear screen
        let mut draw_handle = rl.begin_drawing(thread);
        draw_handle.clear_background(Color::BLACK);

        // Top
        draw_handle.draw_text(&self.title.text, self.title.pos.x as i32, self.title.pos.y as i32, self.title.size, self.title.color);

        // Middle
        // draw_handle.draw_rectangle_rec(&self.singleplayer.rect, Color::GRAY);
        draw_handle.draw_text(&self.singleplayer.text, self.singleplayer.pos.x as i32, self.singleplayer.pos.y as i32,
                              20 as i32, if self.singleplayer.focused {HOVERING_BTN_COLOR} else {BTN_COLOR});

        // draw_handle.draw_rectangle_rec(&self.quit.rect, Color::GRAY);
        draw_handle.draw_text(&self.quit.text, self.quit.pos.x as i32, self.quit.pos.y as i32,
            20 as i32, if self.quit.focused {HOVERING_BTN_COLOR} else {BTN_COLOR});

        // Bottom
        draw_handle.draw_text(&self.hiscore.text, self.hiscore.pos.x as i32, self.hiscore.pos.y as i32, self.hiscore.size as i32, self.hiscore.color);
    }
}

impl MainMenu {
    fn on_click_play(self: &mut Self) {
        self.is_active = false;
    }

    pub fn new() -> MainMenu {
        return MainMenu {
            is_active: true,

            title: Text {
                text: "Pong 2: The Enemy is Now Another".to_string(),
                color: Color::GOLD,
                size: 26,

                pos: Vector2 { 
                    x: SCREEN_SIZE.x / 2.0 - measure_text("Pong 2: The Enemy is Now Another", 26) as f32 / 2.0,
                    y: SCREEN_SIZE.y * 0.1 
                }
            },

            singleplayer: Button {
                text: "Singleplayer".to_string(),
                pos: Vector2 {
                    x: SCREEN_SIZE.x / 2.0 - measure_text("Singleplayer", 20) as f32 / 2.0,
                    y: SCREEN_SIZE.y * 0.4
                },

                focused: false,
                rect: Rectangle::new (
                    SCREEN_SIZE.x / 2.0 as f32 - (measure_text("Singleplayer", 20) + 30) as f32 / 2.0, 
                    SCREEN_SIZE.y * 0.4 - 10.0, measure_text("Singleplayer", 20) as f32 + 30.0, 40.0
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
            },

            hiscore: Text {
                text: format!("HiScore: {}", get_highscore()).to_string(),
                color: Color::WHITE,
                size: 16,

                pos: Vector2 { 
                    x: SCREEN_SIZE.x/2.0 - (measure_text(&format!("HiScore: {}", get_highscore()), 15) as f32/2.0),
                    y: SCREEN_SIZE.y * 0.95 
                }
            },
        }                
    }
}