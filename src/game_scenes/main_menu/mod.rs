mod title_screen;
mod device_screen;
mod connect_screen;
mod multiplayer_screen;

use super::*;
use crate::utils::*;
use regex::Regex;

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

struct MultiplayerScreen {
    local_multiplayer: Button,
    online_multiplayer: Button,

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

struct ConnectScreen {
    title_txt: Text,
    connect_btn: Button,

    player_id: i32,
    player_txt: Text,
    
    player_names:Vec<String>,
    player_colors: Vec<Color>,

    device_id: i32,
    device_txt: Text,

    device_btns: Vec<Button>,
    player_btns: Vec<Button>,

    remote_ip_txt: Text,
    remote_ip_field: TextField,

    remote_info_txt: Text,
    connection_status_txt: Text,

    is_active: bool,
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
    }

    fn draw(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {        
        let elements = self.current_screen.get_elements(rl);
        let mut draw_handle = rl.begin_drawing(thread);
        draw_handle.clear_background(Color::BLACK);
        
        for text in &elements.texts {
            draw_handle.draw_text(&text.text, text.pos.x as i32, text.pos.y as i32, 
                text.size, text.color);
        }

        for (button, color) in &elements.buttons {
            draw_handle.draw_text(&button.text, button.pos.x as i32, button.pos.y as i32,
                20 as i32, color);
        }

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
            is_active: true
        }            
    }
}