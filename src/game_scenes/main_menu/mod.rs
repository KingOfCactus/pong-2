mod ui_screens;
use regex::Regex;

use super::*;
use crate::utils::*;

pub enum MenuScreen { TitleScreen, DeviceScreen, ConnectScreen, MultiplayerScreen }

struct TitleScreen {
    title_txt: Text,
    hiscore_txt: Text,
    selected_mode: GameMode,

    singleplayer_btn: Button,  
    multiplayer_btn: Button,
    quit_btn: Button,

    is_active: bool,
    next_screen: MenuScreen
}

struct DeviceScreen {
    title_txt: Text,
    device_1_txt: Text,
    device_2_txt: Text,

    selected_devices: Vec<i32>,
    device_1_btns: Vec<Button>,
    device_2_btns: Vec<Button>,
    start_btn: Button,

    is_active: bool,
    selected_gamemode: GameMode
}

impl GameScene for MainMenu {
    fn update(self: &mut Self, rl: &RaylibHandle) {
        if !self.current_screen.is_active() {
            if self.current_screen.goes_to_scene() 
            {
                self.is_active = false;
                return;
            }
            self.current_screen = self.current_screen.get_next_screen(&rl);  
        }

        self.current_screen.update(&rl);

        // self.remove_ip_field.update(rl, rl.get_mouse_position());
        // self.connect_btns[4].enabled = self.remove_ip_field.is_ipv4();
    }

    fn draw(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let mouse_pos = rl.get_mouse_position();
        
        let mut elements = self.current_screen.get_elements(rl);
        let mut draw_handle = rl.begin_drawing(thread);
        draw_handle.clear_background(Color::BLACK);
        
        

        // Draw texts 
        for text in &elements.texts {
            draw_handle.draw_text(&text.text, text.pos.x as i32, text.pos.y as i32, 
                text.size, text.color);
        }

        // Draw buttons 
        for (button, color) in &elements.buttons {
            draw_handle.draw_text(&button.text, button.pos.x as i32, button.pos.y as i32,
                20 as i32, color);
        }

        // Draw text fields 
        for field in &elements.fields {
            draw_handle.draw_rectangle_rec(field.rects[0], field.colors[0]);
            draw_handle.draw_rectangle_rec(field.rects[1], field.colors[1]);
            draw_handle.draw_text(&field.text.text, field.text.pos.x as i32, field.text.pos.y as i32, 
                field.text.size, field.text.color);
        }
    }

    fn is_active(&self) -> bool { return self.is_active; }
    fn get_next_scene(&self, rl: &RaylibHandle) -> Box<dyn GameScene> { 
        return self.current_screen.get_next_scene(rl);
    }
}


impl MainMenu { 
    pub fn new() -> MainMenu {
        return MainMenu {
            current_screen: Box::new(TitleScreen::new()),

            local_multiplayer: Button::new(true, "Local Multiplayer", Vector2::new(0.5, 0.4)),
            online_multiplayer: Button::new(true, "Online Multiplayer", Vector2::new(0.5, 0.5)),

            connect_txts: vec![
                Text::new("Select Player and Device:", Vector2::new(0.270, 0.25), Color::WHITE, 20),
                Text::new("Player 1 (Ball)", Vector2::new(0.270, 0.375), Color::new(10, 255, 255, 150), 20),
                Text::new("Device", Vector2::new(0.270, 0.475), Color::GRAY, 20),

                Text::new("Remote Player Address:", Vector2::new(0.7415, 0.25), Color::WHITE, 20),
                // Text::new("xxx.xxx.xxx.xxx", Vector2::new(0.75, 0.375), Color::GRAY, 20),

                // Text::new("Remote client already\nselected player 1\nPing: 120 ms...", Vector2::new(0.3, 0.7), Color::GRAY, 20),
                // Text::new("Started connection,\nwaiting response...\n", Vector2::new(0.3, 0.7), Color::GRAY, 20),
                Text::new("\n(TODO)\n", Vector2::new(0.270, 0.7), Color::GRAY, 20),

                Text::new("\nWaiting for connection...\n", Vector2::new(0.7415, 0.7), Color::GRAY, 20)
                // Text::new("Remote client already\nselected player 1\nPing: 120 ms...", Vector2::new(0.3, 0.7), Color::GRAY, 20),
                // Text::new("Started connection,\nwaiting response...\n", Vector2::new(0.7, 0.7), Color::GRAY, 20),
                // Text::new("Timeout. Did the other\nplayer forgot to press\nthe 'Connect' button?", Vector2::new(0.75, 0.7), Color::GRAY, 20),
            ],
            

            remove_ip_field: TextField::new(Regex::new("[.,0-9]").expect("Invalid regex"), 
                                            "---.---.---.---", 185.0, 20, Vector2::new(0.7415, 0.375), 5.0, 
                                            vec![Color::WHITE, Color::new(30, 30, 30, 255)], 15),

            connect_btns: vec![
                // Player Id
                Button::new(true, ">", Vector2::new(0.470, 0.375)),
                Button::new(true, "<", Vector2::new(0.070, 0.375)),
                
                // Input Device
                Button::new(true, ">", Vector2::new(0.470, 0.475)),
                Button::new(true, "<", Vector2::new(0.070, 0.475)),

                Button::new(false, "Connect", Vector2::new(0.7415, 0.475)),
            ],
            
            is_active: true,
            selected_mode: GameMode::Singleplayer,
        }            
    }
}